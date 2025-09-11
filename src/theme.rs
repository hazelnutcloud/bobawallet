use gpui::{Hsla, Rgba, SharedString};
use gpui_component::ThemeMode;

pub struct ThemeConfig {
    name: SharedString,
    colors: ThemeConfigColors,
    mode: ThemeMode,
    is_default: bool,
}

pub struct ThemeConfigColors {
    base_100: SharedString,
    base_200: SharedString,
    base_300: SharedString,
    base_content: SharedString,
    primary: SharedString,
    primary_content: SharedString,
    secondary: SharedString,
    secondary_content: SharedString,
    accent: SharedString,
    accent_content: SharedString,
    neutral: SharedString,
    neutral_content: SharedString,
    info: SharedString,
    info_content: SharedString,
    success: SharedString,
    success_content: SharedString,
    warning: SharedString,
    warning_content: SharedString,
    error: SharedString,
    error_content: SharedString,
}

impl From<ThemeConfig> for gpui_component::ThemeConfig {
    fn from(value: ThemeConfig) -> Self {
        gpui_component::ThemeConfig {
            is_default: value.is_default,
            name: value.name,
            mode: value.mode,
            colors: (),
            highlight: None,
        }
    }
}

impl From<ThemeConfigColors> for gpui_component::ThemeConfigColors {
    fn from(value: ThemeConfigColors) -> Self {
        let base_100 = Rgba::try_from(value.base_100.as_str()).ok();
        let base_200 = Rgba::try_from(value.base_200.as_str()).ok();
        let base_300 = Rgba::try_from(value.base_300.as_str()).ok();
        let base_content = Rgba::try_from(value.base_content.as_str()).ok();
        let primary = Rgba::try_from(value.primary.as_str()).ok();
        let primary_content = Rgba::try_from(value.primary_content.as_str()).ok();
        let secondary = Rgba::try_from(value.secondary.as_str()).ok();
        let secondary_content = Rgba::try_from(value.secondary_content.as_str()).ok();
        let accent = Rgba::try_from(value.accent.as_str()).ok();
        let accent_content = Rgba::try_from(value.accent_content.as_str()).ok();
        let neutral = Rgba::try_from(value.neutral.as_str()).ok();
        let neutral_content = Rgba::try_from(value.neutral_content.as_str()).ok();
        let info = Rgba::try_from(value.info.as_str()).ok();
        let info_content = Rgba::try_from(value.info_content.as_str()).ok();
        let success = Rgba::try_from(value.success.as_str()).ok();
        let success_content = Rgba::try_from(value.success_content.as_str()).ok();
        let warning = Rgba::try_from(value.warning.as_str()).ok();
        let warning_content = Rgba::try_from(value.warning_content.as_str()).ok();
        let error = Rgba::try_from(value.error.as_str()).ok();
        let error_content = Rgba::try_from(value.error_content.as_str()).ok();

        Self {
            accent: base_content
                .map(|c| serde_json::to_string(&Rgba { a: 0.1, ..c }).unwrap().into()),
            accent_foreground: Some(value.base_content),
            accordion: Some(value.base_100),
            accordion_hover: Some(value.base_100),
            background: Some(value.base_100),
            border: Some(value.base_300),
            caret: Some(value.base_content),
            chart_1: Some(value.primary),
            chart_2: Some(value.secondary),
            chart_3: Some(value.accent),
            chart_4: Some(value.info),
            chart_5: Some(value.success),
            danger: Some(value.warning),
            danger_active: Some(value.warning),
            danger_foreground: Some(value.warning_content),
            danger_hover: Some(value.warning),
        }
    }
}
