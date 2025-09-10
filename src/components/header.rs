use gpui::{Action, Corner, SharedString, prelude::*, px, rems};
use gpui::{Hsla, Render, Window, div, svg};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::popup_menu::PopupMenuExt;
use gpui_component::{ActiveTheme, Icon, Sizable, StyledExt};

#[derive(Action, PartialEq, Clone)]
#[action(no_json)]
pub struct SelectAccount {
    pub account: SharedString,
}

#[derive(Action, PartialEq, Clone)]
#[action(no_json)]
pub struct ExpandAccount {
    pub account: SharedString,
}

pub struct Header;

impl Render for Header {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let fg = theme.foreground;
        let muted_fg = theme.muted_foreground;

        div()
            .flex()
            .w_full()
            .items_center()
            .p_2()
            .text_sm()
            .bg(theme.accent)
            .border_color(theme.title_bar_border)
            .text_color(Hsla {
                a: 0.7,
                ..theme.foreground
            })
            .gap_0p5()
            .child(
                Button::new("menu-button")
                    .child("boba")
                    .ghost()
                    .xsmall()
                    .popup_menu(|this, _, _| {
                        this.link("bobawallet", "https://github.com/hazelnutcloud/bobawallet")
                    }),
            )
            .child(div().flex_1())
            .child(
                Button::new("select-account-button")
                    .icon(Icon::empty().path("icons/caret-up-down.svg"))
                    .ghost()
                    .xsmall()
                    .popup_menu(|this, _, _| {
                        this.menu(
                            "0xE9Faf...bE7C8",
                            Box::new(SelectAccount {
                                account: "0xE9Faf59a975BEB99678ACc785c5a901De32bE7C8".into(),
                            }),
                        )
                    }),
            )
            .child(
                Button::new("expand-account-button")
                    .child("0xd2Be6...E1Bc3")
                    .ghost()
                    .small()
                    .compact()
                    .popup_menu(move |this, _, _| {
                        this.menu_element(
                            Box::new(ExpandAccount {
                                account: "0xd2Be6...E1Bc3".into(),
                            }),
                            move |_, _| {
                                div()
                                    .v_flex()
                                    .p_1()
                                    .gap_2()
                                    .items_center()
                                    .text_sm()
                                    .child(
                                        svg()
                                            .path("images/address_qr.svg")
                                            .w_full()
                                            .h(rems(7.5))
                                            .text_color(fg),
                                    )
                                    .child(div().w_full().h(px(1.0)).mt_1().bg(muted_fg))
                                    .child(
                                        div()
                                            .v_flex()
                                            .gap_neg_1()
                                            .child("0xd2be6586c9ed")
                                            .child("3b940396db1893")
                                            .child("c6a0e0d52e1bc3"),
                                    )
                            },
                        )
                    })
                    .anchor(Corner::TopRight),
            )
    }
}
