use gpui::{Action, actions, prelude::*};
use gpui::{Hsla, Render, Window, div};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::popup_menu::PopupMenuExt;
use gpui_component::{ActiveTheme, Sizable};

pub struct Footer;

#[derive(Clone, PartialEq, Action)]
#[action(no_json)]
pub struct SelectChain {
    pub chain_id: u32,
}

actions!(server, [Restart]);

impl Render for Footer {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .flex()
            .w_full()
            .items_center()
            .p_2()
            .text_xs()
            .bg(theme.accent)
            .border_color(theme.title_bar_border)
            .text_color(Hsla {
                a: 0.7,
                ..theme.foreground
            })
            .gap_0p5()
            .child("home")
            .child(div().flex_1())
            .child(
                Button::new("select-chain-button")
                    .child("mainnet")
                    .ghost()
                    .xsmall()
                    .popup_menu(|this, _, _| {
                        this.menu("rise", Box::new(SelectChain { chain_id: 11535 }))
                    })
                    .anchor(gpui::Corner::BottomLeft),
            )
            .child(div().size_2().rounded_full().bg(theme.green))
            .child(
                Button::new("configure-server-button")
                    .child("localhost:1248")
                    .ghost()
                    .xsmall()
                    .popup_menu(|this, _, _| this.menu("restart server", Box::new(Restart)))
                    .anchor(gpui::Corner::BottomRight),
            )
    }
}
