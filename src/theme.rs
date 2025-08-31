use gpui::{App, Global, Pixels, Rems, Rgba, SharedString, rems, rgb};

pub struct ThemeVariables {
    pub base_100: Rgba,
    pub base_200: Rgba,
    pub base_300: Rgba,
    pub base_content: Rgba,
    pub primary: Rgba,
    pub primary_content: Rgba,
    pub secondary: Rgba,
    pub secondary_content: Rgba,
    pub accent: Rgba,
    pub accent_content: Rgba,
    pub neutral: Rgba,
    pub neutral_content: Rgba,
    pub info: Rgba,
    pub info_content: Rgba,
    pub success: Rgba,
    pub success_content: Rgba,
    pub warning: Rgba,
    pub warning_content: Rgba,
    pub error: Rgba,
    pub error_content: Rgba,
    pub radius_selector: Rems,
    pub radius_field: Rems,
    pub radius_box: Rems,
    pub size_selector: Rems,
    pub size_field: Rems,
    pub border_width: Pixels,
}

pub struct Theme {
    pub name: SharedString,
    pub variables: ThemeVariables,
}

impl Theme {
    pub fn init(cx: &mut App) {
        cx.set_global(Theme::default())
    }
}

impl Global for Theme {}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Dim".into(),
            variables: ThemeVariables {
                base_100: rgb(0x2A303C),
                base_200: rgb(0x242933),
                base_300: rgb(0x20252E),
                base_content: rgb(0xB2CCD6),
                primary: rgb(0x9FE88D),
                primary_content: rgb(0x091307),
                secondary: rgb(0xFF7D5D),
                secondary_content: rgb(0x160503),
                accent: rgb(0xC792E9),
                accent_content: rgb(0x0E0813),
                neutral: rgb(0x1C212B),
                neutral_content: rgb(0xB2CCD6),
                info: rgb(0x28EBFF),
                info_content: rgb(0x011316),
                success: rgb(0x62EFBD),
                success_content: rgb(0x03140D),
                warning: rgb(0xEFD057),
                warning_content: rgb(0x141003),
                error: rgb(0xFFAE9B),
                error_content: rgb(0x160B09),
                radius_selector: rems(0.5),
                radius_field: rems(0.5),
                radius_box: rems(0.),
                size_selector: rems(0.25),
                size_field: rems(0.25),
                border_width: Pixels(1.0),
            },
        }
    }
}
