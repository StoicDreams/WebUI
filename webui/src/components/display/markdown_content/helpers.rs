use std::collections::HashMap;
use super::*;

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
