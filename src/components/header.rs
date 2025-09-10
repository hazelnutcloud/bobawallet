use gpui::{Action, Corner, SharedString, actions, prelude::*, rems};
use gpui::{Hsla, Render, Window, div, svg};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::popup_menu::PopupMenuExt;
use gpui_component::{ActiveTheme, Icon, Sizable, StyledExt};

#[derive(Action, PartialEq, Clone)]
#[action(no_json)]
pub struct SelectAccount {
    pub account: SharedString,
}

actions!(header, [ExpandAccount]);

pub struct Header;

impl Render for Header {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let fg = theme.foreground;

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
                    .small()
                    .popup_menu(|this, _, _| {
                        this.label("switch account").separator().menu(
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
                    .xsmall()
                    .compact()
                    .popup_menu(move |this, _, _| {
                        this.label("account info")
                            .separator()
                            .menu_element_with_disabled(
                                Box::new(ExpandAccount),
                                true,
                                move |_, _| {
                                    div().p_0p5().w_full().h(rems(7.7)).child(
                                        svg()
                                            .path("images/address_qr.svg")
                                            .size_full()
                                            .text_color(fg),
                                    )
                                },
                            )
                            .separator()
                            .menu_element(Box::new(ExpandAccount), move |_, _| {
                                div()
                                    .v_flex()
                                    .p_0p5()
                                    .gap_neg_1()
                                    .text_sm()
                                    .child("0xd2be6586c9ed")
                                    .child("3b940396db1893")
                                    .child("c6a0e0d52e1bc3")
                            })
                    })
                    .anchor(Corner::TopRight),
            )
    }
}
