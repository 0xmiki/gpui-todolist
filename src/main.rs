use gpui::*;
use gpui_component::*;
use std::path::PathBuf;
mod components;
mod window;

use components::TodoList;
use crate::{
    window::{get_window_options, blur_window},
};

fn main() {
    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
      
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        let theme_name = SharedString::from("macOS Classic Dark");
        
        if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx)
                .themes()
                .get(&theme_name)
                .cloned()
            {
                Theme::global_mut(cx).apply_config(&theme);
            }
        }) {
            eprintln!("Failed to watch themes directory: {}", err);
        }

        let window_opts = get_window_options(cx);
        cx.spawn(async move |cx| {
             
            cx.open_window(window_opts, |window, cx| {
                blur_window(window);
                let view = cx.new(|cx| TodoList::new(window, cx));
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}