use super::*;

pub(super) fn get_option(value: &str) -> Option<&str> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

pub(super) fn get_u8(value: &str) -> u8 {
    u8::from_str(value).unwrap_or(0)
}

pub(super) fn get_u16(value: &str) -> u16 {
    u16::from_str(value).unwrap_or(0)
}

pub(super) fn get_u32(value: &str) -> u32 {
    u32::from_str(value).unwrap_or(0)
}

pub(super) fn get_u64(value: &str) -> u64 {
    u64::from_str(value).unwrap_or(0)
}

pub(super) fn get_loading_variant(variant: &str) -> LoadingVariant {
    match variant {
        "bar" => LoadingVariant::Bar,
        "striped" | "stripedbar" => LoadingVariant::StripedBar,
        _ => LoadingVariant::Circle,
    }
}

pub(super) fn get_theme(theme: &str) -> Theme {
    match theme {
        "active" => Theme::Active,
        "background" => Theme::Background,
        "black" => Theme::Black,
        "white" => Theme::White,
        "secondary" => Theme::Secondary,
        "tertiary" => Theme::Tertiary,
        "info" => Theme::Info,
        "success" => Theme::Success,
        "warning" => Theme::Warning,
        "danger" => Theme::Danger,
        "title" => Theme::Title,
        "inherit" => Theme::None,
        _ => Theme::Primary,
    }
}
