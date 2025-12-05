use gpui::prelude::*;
use gpui::div;
use gpui_component::input::{InputState, Input, InputEvent};
use gpui_component::button::Button;
use gpui::{Context, Entity, Window, EventEmitter};
 use gpui_component::button::ButtonVariants;
use gpui::Focusable;

pub struct TextInput {
    input_state: Entity<InputState>,
}

impl TextInput {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx));
        cx.subscribe_in(&input_state, window, |this, _emitter, event, window, cx| {
            if let InputEvent::PressEnter { .. } = event {
                let text = this.input_state.read(cx).text().clone();
                if !text.to_string().is_empty() {
                    cx.emit(text.to_string());
                    this.input_state.update(cx, |state, cx| {
                        state.set_value("", window, cx);
                    });
                }
            }
        }).detach();

        // Focus the input initially
        input_state.update(cx, |state, cx| {
            state.focus_handle(cx).focus(window);
        });

        Self {
            input_state,
        }
    }
}

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .child(
                Input::new(&self.input_state)
                .cleanable(true)
            )
            // .child(
            //     div()
            //         .child(
            //             Button::new("add-todo-button")
            //             .primary()
            //                 .label("Add Todo")
            //                 .on_click(cx.listener(|this, _, _window, cx| {
            //                     let text = this.input_state.read(cx).text().clone();
            //                     if !text.to_string().is_empty() {
            //                         cx.emit(text.to_string());
            //                         this.input_state = cx.new(|cx| InputState::new(_window, cx));
            //                             this.input_state.update(cx, |state, cx| {
            //                             state.focus_handle(cx).focus(_window);
            //                         });
            //                         cx.notify();
            //                     }
            //                 }))
            //         )
            // )
    }
}

impl EventEmitter<String> for TextInput {}
