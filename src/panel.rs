use gpui::prelude::*;
use gpui::{Render, Styled, div};

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
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .child(String::from("BobaWallet"))
    }
}
