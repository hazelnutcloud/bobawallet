use gpui::prelude::*;
use gpui::{Render, Rgba, Window, div};

use crate::theme::Theme;

pub struct Footer;

impl Render for Footer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .flex()
            .w_full()
            .items_center()
            .p_2()
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
            .child("localhost:1248")
    }
}
