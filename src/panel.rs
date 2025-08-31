use gpui::{Render, Styled, div};
use gpui::{Rgba, prelude::*};

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
        let theme = cx.global::<Theme>();

        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_start()
            .bg(theme.variables.base_200)
            .font_family("Berkeley Mono")
            .child(
                div()
                    .flex()
                    .w_full()
                    .items_center()
                    .p_3()
                    .bg(theme.variables.base_100)
                    .text_color(Rgba {
                        a: 0.7,
                        ..theme.variables.base_content
                    })
                    .child("boba")
                    .child(div().flex_1())
                    .child("0xd2Be6...E1Bc3"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p_3()
                    .justify_start()
                    .flex_1(),
            )
            .child(
                div()
                    .flex()
                    .w_full()
                    .items_center()
                    .p_3()
                    .text_xs()
                    .bg(theme.variables.base_100)
                    .text_color(Rgba {
                        a: 0.7,
                        ..theme.variables.base_content
                    })
                    .gap_2()
                    .child("home")
                    .child(div().flex_1())
                    .child("mainnet")
                    .child(div().size_2().rounded_full().bg(theme.variables.success))
                    .child("localhost:1248"),
            )
    }
}
