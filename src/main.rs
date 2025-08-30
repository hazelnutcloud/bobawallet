use std::time::Duration;

use bobawallet::Panel;
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use gpui::prelude::*;
use gpui::{
    Application, Bounds, Edges, Timer, TitlebarOptions, WindowBounds, WindowKind, WindowOptions,
    point, px, size,
};

struct AppState {
    is_hidden: bool,
}

impl AppState {
    fn new() -> Self {
        Self { is_hidden: false }
    }

    fn toggle_hide(&mut self) {
        self.is_hidden = !self.is_hidden;
    }
}

pub fn main() {
    // setup global hotkey
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::ALT), Code::Slash);
    manager.register(hotkey).unwrap();
    let (tx, rx) = crossbeam_channel::unbounded();
    std::thread::spawn(move || {
        loop {
            if let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
                tx.send(event).ok();
            }

            std::thread::sleep(Duration::from_millis(8));
        }
    });

    Application::new().run(move |cx| {
        let app_state = cx.new(|_| AppState::new());

        cx.spawn(async move |cx| {
            loop {
                if let Ok(event) = rx.try_recv()
                    && event.state() == HotKeyState::Released
                {
                    cx.update(|cx| {
                        cx.update_entity(&app_state, |state, cx| {
                            if state.is_hidden {
                                cx.activate(true);
                            } else {
                                cx.hide();
                            }
                            state.toggle_hide();
                        });
                    })
                    .ok();
                }

                Timer::after(Duration::from_millis(8)).await;
            }
        })
        .detach();

        let bounds = match cx.primary_display() {
            Some(display) => display.bounds().extend(Edges {
                left: -(display.bounds().size.width - px(400.0)),
                ..Default::default()
            }),
            None => Bounds::centered(None, size(px(400.0), px(400.0)), cx),
        };

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Maximized(bounds)),
                titlebar: Some(TitlebarOptions {
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(-999.0), px(-999.0))),
                    ..Default::default()
                }),
                is_minimizable: false,
                is_movable: false,
                is_resizable: false,
                kind: WindowKind::PopUp,
                ..Default::default()
            },
            |_, cx| cx.new(|_| Panel::new()),
        )
        .unwrap();

        cx.activate(true);
    });
}
