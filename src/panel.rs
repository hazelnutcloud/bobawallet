use gpui::{Entity, Window, prelude::*};
use gpui::{Render, Styled, div};
use gpui_component::{ActiveTheme, Root};

use crate::components::{CommandPalette, Footer, Header};

pub struct Panel {
    header: Entity<Header>,
    footer: Entity<Footer>,
    command_palette: Entity<CommandPalette>,
}

impl Panel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let header = cx.new(|_| Header);
        let footer = cx.new(|_| Footer);
        let command_palette = cx.new(|cx| CommandPalette::new(window, cx));

        Self {
            header,
            footer,
            command_palette,
        }
    }
}

impl Render for Panel {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let drawer_layer = Root::render_drawer_layer(window, cx);
        let modal_layer = Root::render_modal_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        let theme = cx.theme();

        div()
            .size_full()
            .flex()
            .flex_col()
            .justify_start()
            .bg(theme.background)
            .font_family("Berkeley Mono")
            .child(self.header.clone())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .p_2()
                    .justify_start()
                    .flex_1()
                    .w_full()
                    .child(self.command_palette.clone()),
            )
            .child(self.footer.clone())
            .children(drawer_layer)
            .children(modal_layer)
            .children(notification_layer)
    }
}
