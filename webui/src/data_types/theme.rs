#[derive(PartialEq, Default)]
pub enum Theme {
    Active,
    Background,
    Black,
    White,
    #[default]
    Primary,
    Secondary,
    Tertiary,
    Success,
    Info,
    Warning,
    Danger,
    Title,
    None,
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Active => write!(f, "theme-active"),
            Theme::Background => write!(f, "theme-background"),
            Theme::Black => write!(f, "theme-black"),
            Theme::White => write!(f, "theme-white"),
            Theme::Primary => write!(f, "theme-primary"),
            Theme::Secondary => write!(f, "theme-secondary"),
            Theme::Tertiary => write!(f, "theme-tertiary"),
            Theme::Success => write!(f, "theme-success"),
            Theme::Info => write!(f, "theme-info"),
            Theme::Warning => write!(f, "theme-warning"),
            Theme::Danger => write!(f, "theme-danger"),
            Theme::Title => write!(f, "theme-title"),
            Theme::None => write!(f, "theme-inherit"),
        }
    }
}
