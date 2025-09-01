use gpui::prelude::*;
use gpui::{Render, Styled, div};

use crate::components::{CommandPalette, Footer, Header};
use crate::theme::Theme;

pub struct Panel {}

impl Panel {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for Panel {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let header = cx.new(|_| Header);
        let command_palette = cx.new(|_| CommandPalette);
        let footer = cx.new(|_| Footer);

        let theme = cx.global::<Theme>();

        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_start()
            .bg(theme.variables.base_200)
            .font_family("Berkeley Mono")
            .child(header)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p_3()
                    .justify_start()
                    .flex_1()
                    .w_full()
                    .child(command_palette),
            )
            .child(footer)
    }
}
