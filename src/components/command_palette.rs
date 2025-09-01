use gpui::prelude::*;
use gpui::{Render, Rgba, Window, div};

use crate::theme::Theme;

pub struct CommandPalette;

impl Render for CommandPalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div().w_full().p_2().child(
            div()
                .flex()
                .w_full()
                .rounded(theme.variables.radius_field)
                .px_4()
                .py_2()
                .items_center()
                .justify_between()
                .border_color(Rgba {
                    a: 0.2,
                    ..theme.variables.base_content
                })
                .text_color(Rgba {
                    a: 0.2,
                    ..theme.variables.base_content
                })
                .border_1()
                .child("> search / commands")
                .child("cmd+k"),
        )
    }
}
