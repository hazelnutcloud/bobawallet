use std::time::Duration;

use bobawallet::{Assets, Panel, panel};
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use gpui::{
    App, Application, Bounds, Edges, Global, Timer, TitlebarOptions, WindowBounds, WindowKind,
    WindowOptions, actions, point, prelude::*, px, size,
};
use gpui_component::Root;

struct AppState {
    is_hidden: bool,
}

impl AppState {
    fn init(cx: &mut App) {
        let state = Self { is_hidden: false };
        cx.set_global(state);
    }

    fn toggle_hide(cx: &mut App) -> bool {
        cx.update_global(|state: &mut Self, cx| {
            state.is_hidden = !state.is_hidden;
            if state.is_hidden {
                cx.hide();
            } else {
                cx.activate(true);
            }
            state.is_hidden
        })
    }
}

impl Global for AppState {}

actions!(window, [ToggleHide]);

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

    Application::new().with_assets(Assets).run(move |cx| {
        gpui_component::init(cx);
        panel::init(cx);

        Assets.load_fonts(cx).unwrap();
        AppState::init(cx);

        cx.spawn(async move |cx| {
            loop {
                if let Ok(event) = rx.try_recv()
                    && event.state() == HotKeyState::Pressed
                {
                    cx.update(|cx| {
                        cx.dispatch_action(&ToggleHide);
                    })
                    .ok();
                }

                Timer::after(Duration::from_millis(8)).await;
            }
        })
        .detach();

        cx.on_action(|_: &ToggleHide, cx| {
            AppState::toggle_hide(cx);
        });

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
            |window, cx| {
                let panel = cx.new(|cx| Panel::new(window, cx));
                cx.new(|cx| Root::new(panel.into(), window, cx))
            },
        )
        .unwrap();

        cx.activate(true);
    });
}
