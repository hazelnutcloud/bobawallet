use gpui::{Entity, Focusable, prelude::*};
use gpui::{Render, Window, div};
use gpui_component::input::{InputState, TextInput};

pub struct CommandPalette {
    input_state: Entity<InputState>,
}

impl CommandPalette {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_state = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("> search / commands")
                .clean_on_escape()
        });
        Self { input_state }
    }
}

impl Focusable for CommandPalette {
    fn focus_handle(&self, cx: &gpui::App) -> gpui::FocusHandle {
        self.input_state.focus_handle(cx)
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().w_full().p_2().child(
            TextInput::new(&self.input_state)
                .cleanable()
                .suffix("cmd+k"),
        )
    }
}
