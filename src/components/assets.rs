use gpui::{Hsla, Render, Styled, div, prelude::*, rems};
use gpui_component::{
    ActiveTheme, IconName, Sizable, StyledExt,
    avatar::{Avatar, AvatarGroup},
    button::{Button, ButtonVariants},
};

pub struct AssetsView;

impl AssetsView {
    pub fn new(_window: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> Self {
        Self {}
    }
}

impl Render for AssetsView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let theme = cx.theme();
        div()
            .v_flex()
            .w_full()
            .bg(theme.accent)
            .child(
                div()
                    .p_2()
                    .h_flex()
                    .items_center()
                    .bg(Hsla {
                        a: 0.2,
                        ..theme.foreground
                    })
                    .gap_1()
                    .child(
                        Button::new("filter-assets-button")
                            .icon(IconName::Settings2)
                            .ghost()
                            .small(),
                    )
                    .child(div().font_bold().child("assets"))
                    .child(div().flex_1())
                    .child(
                        Button::new("add-asset-button")
                            .icon(IconName::Plus)
                            .ghost()
                            .small()
                            .compact()
                            .child("add asset"),
                    ),
            )
            .child(
                div()
                    .h_flex()
                    .justify_between()
                    .items_center()
                    .p_2()
                    .child("TOTAL")
                    .child(div().child("$3,891.00").text_color(theme.green)),
            )
            .child(
                div()
                    .h_flex()
                    .items_center()
                    .p_2()
                    .border_t_1()
                    .border_color(Hsla {
                        a: 0.7,
                        ..theme.foreground
                    })
                    .text_color(Hsla {
                        a: 0.7,
                        ..theme.foreground
                    })
                    .child("ETH")
                    .child(
                        AvatarGroup::new()
                            .xsmall()
                            .child(Avatar::new().src("images/ethereum_logo.jpg"))
                            .child(Avatar::new().src("images/base_logo.jpg"))
                            .child(Avatar::new().src("images/arbitrum_logo.jpg"))
                            .w(rems(3.25)),
                    )
                    .child(div().flex_1())
                    .child(div().child("0.1258 ($235.78)").text_color(Hsla {
                        a: 0.7,
                        ..theme.green
                    })),
            )
            .child(
                div()
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .p_2()
                    .border_t_1()
                    .border_color(Hsla {
                        a: 0.7,
                        ..theme.foreground
                    })
                    .text_color(Hsla {
                        a: 0.7,
                        ..theme.foreground
                    })
                    .child("USDC")
                    .child(
                        AvatarGroup::new()
                            .xsmall()
                            .child(Avatar::new().src("images/ethereum_logo.jpg"))
                            .child(Avatar::new().src("images/base_logo.jpg"))
                            .child(Avatar::new().src("images/arbitrum_logo.jpg"))
                            .w(rems(3.25)),
                    )
                    .child(div().flex_1())
                    .child(div().child("1123.45 ($1,124.01)").text_color(Hsla {
                        a: 0.7,
                        ..theme.green
                    })),
            )
            .child(
                div()
                    .h_flex()
                    .items_center()
                    .justify_center()
                    .p_2()
                    .border_t_1()
                    .border_color(Hsla {
                        a: 0.7,
                        ..theme.foreground
                    })
                    .text_color(Hsla {
                        a: 0.7,
                        ..theme.foreground
                    })
                    .child("USDT")
                    .child(
                        AvatarGroup::new()
                            .xsmall()
                            .child(Avatar::new().src("images/ethereum_logo.jpg"))
                            .child(Avatar::new().src("images/base_logo.jpg"))
                            .child(Avatar::new().src("images/arbitrum_logo.jpg"))
                            .w(rems(3.25)),
                    )
                    .child(div().flex_1())
                    .child(div().child("2531.93 ($2,531.21)").text_color(Hsla {
                        a: 0.7,
                        ..theme.green
                    })),
            )
    }
}
