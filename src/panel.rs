use gpui::{AnyView, App, Entity, Focusable, KeyBinding, Window, actions, prelude::*};
use gpui::{Render, Styled, div};
use gpui_component::{ActiveTheme, Root};

use crate::components::{AssetsView, CommandPalette, Footer, Header};

actions!([Focus]);

pub fn init(cx: &mut App) {
    cx.bind_keys(vec![KeyBinding::new("secondary-k", Focus, None)]);
}

pub struct Panel {
    header: Entity<Header>,
    footer: Entity<Footer>,
    command_palette: Entity<CommandPalette>,
    body: AnyView,
}

impl Panel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let header = cx.new(|_| Header);
        let footer = cx.new(|_| Footer);
        let command_palette = cx.new(|cx| CommandPalette::new(window, cx));
        let default_body = cx.new(|cx| AssetsView::new(window, cx));

        Self {
            header,
            footer,
            command_palette,
            body: default_body.into(),
        }
    }

    fn on_action_focus(&mut self, _: &Focus, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.command_palette.focus_handle(cx));
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
            .id("panel")
            .track_focus(&self.command_palette.focus_handle(cx))
            .on_action(cx.listener(Self::on_action_focus))
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
                    .justify_start()
                    .flex_1()
                    .w_full()
                    .child(self.command_palette.clone())
                    .child(self.body.clone()),
            )
            .child(self.footer.clone())
            .children(drawer_layer)
            .children(modal_layer)
            .children(notification_layer)
    }
}
