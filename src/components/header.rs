use gpui::prelude::*;
use gpui::{Render, Rgba, Window, div, svg};

use crate::theme::Theme;

pub struct Header;

impl Render for Header {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .w_full()
            .items_center()
            .p_2()
            .text_sm()
            .bg(theme.variables.base_100)
            .text_color(Rgba {
                a: 0.7,
                ..theme.variables.base_content
            })
            .gap_1p5()
            .child(svg().path("icons/list.svg").size_3().text_color(Rgba {
                a: 0.7,
                ..theme.variables.base_content
            }))
            .child("boba")
            .child(div().flex_1())
            .child(
                svg()
                    .path("icons/caret-up-down.svg")
                    .size_3()
                    .text_color(Rgba {
                        a: 0.7,
                        ..theme.variables.base_content
                    }),
            )
            .child("0xd2Be6...E1Bc3")
            .child(
                svg()
                    .path("icons/copy-simple.svg")
                    .size_3()
                    .text_color(Rgba {
                        a: 0.7,
                        ..theme.variables.base_content
                    }),
            )
    }
}
