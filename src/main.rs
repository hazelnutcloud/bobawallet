use std::{fs, path::PathBuf, time::Duration};

use anyhow::Result;
use bobawallet::{Panel, theme::Theme};
use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
    hotkey::{Code, HotKey, Modifiers},
};
use gpui::{App, AssetSource, Global, SharedString, prelude::*};
use gpui::{
    Application, Bounds, Edges, Timer, TitlebarOptions, WindowBounds, WindowKind, WindowOptions,
    point, px, size,
};

struct Assets {
    base: PathBuf,
}

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<std::borrow::Cow<'static, [u8]>>> {
        fs::read(self.base.join(path))
            .map(|data| Some(std::borrow::Cow::Owned(data)))
            .map_err(|e| e.into())
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        fs::read_dir(self.base.join(path))
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        entry
                            .ok()
                            .and_then(|entry| entry.file_name().into_string().ok())
                            .map(SharedString::from)
                    })
                    .collect()
            })
            .map_err(|e| e.into())
    }
}

struct AppState {
    is_hidden: bool,
}

impl AppState {
    fn init(cx: &mut App) {
        let state = Self { is_hidden: false };
        cx.set_global(state);
    }

    fn toggle_hide(cx: &mut App) {
        cx.update_global(|state: &mut Self, cx| {
            state.is_hidden = !state.is_hidden;
            if state.is_hidden {
                cx.hide();
            } else {
                cx.activate(true);
            }
        });
    }
}

impl Global for AppState {}

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

    Application::new()
        .with_assets(Assets {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets"),
        })
        .run(move |cx| {
            AppState::init(cx);
            Theme::init(cx).unwrap();

            cx.spawn(async move |cx| {
                loop {
                    if let Ok(event) = rx.try_recv()
                        && event.state() == HotKeyState::Pressed
                    {
                        cx.update(|cx| {
                            AppState::toggle_hide(cx);
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
