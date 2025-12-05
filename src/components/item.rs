use gpui::prelude::*;
use gpui::div;
use gpui_component::checkbox::Checkbox;
use gpui::{Context, Window, EventEmitter};
use gpui_component::{Icon, IconName};
use gpui_component::button::Button;
use gpui_component::button::ButtonVariants;
use gpui::px;

pub struct TodoItem {
   pub id: usize,
    text: String,
    done: bool,
}

impl TodoItem {
    pub fn new(id: usize, text: String, _cx: &mut Context<Self>) -> Self {  
        Self { id, text, done: false }  
    }   
}

impl Render for TodoItem {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {  
        div()
            .group("item")
            .relative()
            .flex()
            .items_center()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .gap_5()
                    .flex()
                    .relative()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .flex()
                            .relative()
                            .items_start()
                            .gap_4()
                            .child(
                                Checkbox::new(("todo-item", self.id))
                                    .checked(self.done)
                                    .mt_1()
                                    .on_click(cx.listener(|this, checked, _window, cx| {
                                        this.done = *checked;
                                        cx.notify();
                                    }))
                            )
                            .child(
                                div()
                                    .relative()
                                    .id(("todo-text", self.id))
                                    .whitespace_normal()
                                    .max_w(px(250.)) 
                                    .w(px(250.))
                                    .when(self.done, |this| this.line_through().opacity(0.5)) 
                                    .child(self.text.clone())
                                    .on_click(cx.listener(|this, _event, _window, cx| {
                                        this.done = !this.done;
                                        cx.notify();
                                    }))
                            )
                    )
            )
            .child(
                Button::new("delete-btn")
                    .absolute()
                    .top(px(2.))
                    .right(px(10.))
                    .link()
                    .icon(
                        Icon::new(IconName::Close)  
                    )
                    .opacity(0.0)
                    .group_hover("item", |s| s.opacity(0.4))
                    .on_click(cx.listener(|this, _, _window, cx| {
                        cx.emit(this.id);
                    }))
            )
    }
}

impl EventEmitter<usize> for TodoItem    {}
