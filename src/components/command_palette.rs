use gpui::{App, Entity, Focusable, prelude::*};
use gpui::{Render, Window, div};
use gpui_component::ActiveTheme;
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
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.input_state.focus_handle(cx)
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div().w_full().p_4().child(
            TextInput::new(&self.input_state)
                .cleanable()
                .suffix(div().child("cmd+k").text_color(theme.muted_foreground)),
        )
    }
}
