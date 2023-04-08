pub const CLASSES_PAGE_SECTION: &str = "page-segment-standard";
pub const CLASSES_CARD_CONTAINER: &str = "page-segment-cards";

pub const CLASSES_SIDE_BY_SIDE: &str = "side-by-side";
pub const CLASSES_AUTO_MAXCONTENT: &str = "auto-maxcontent";
pub const CLASSES_MAXCONTENT_AUTO: &str = "maxcontent-auto";

pub const CLASSES_READABLE_CONTENT: &str = "readable-content";
pub const CLASSES_FLEX_READABLE_CENTERED: &str =
    "d-flex flex-column justify-center align-center readable-content";

pub const CLASSES_THEME_ACTIVE: &str = "theme-active";
pub const CLASSES_THEME_BACKGROUND: &str = "theme-background";
pub const CLASSES_THEME_BLACK: &str = "theme-black";
pub const CLASSES_THEME_WHITE: &str = "theme-white";
pub const CLASSES_THEME_PRIMARY: &str = "theme-primary";
pub const CLASSES_THEME_SECONDARY: &str = "theme-secondary";
pub const CLASSES_THEME_TERTIARY: &str = "theme-tertiary";
pub const CLASSES_THEME_INFO: &str = "theme-info";
pub const CLASSES_THEME_SUCCESS: &str = "theme-success";
pub const CLASSES_THEME_WARNING: &str = "theme-warning";
pub const CLASSES_THEME_DANGER: &str = "theme-danger";
pub const CLASSES_THEME_TITLE: &str = "theme-title";
pub const CLASSES_THEME_INHERIT: &str = "theme-inherit";

pub const CLASSES_SHOW_AT_MOBILE: &str = "show-at-mobile";
pub const CLASSES_SHOW_AT_TABLET: &str = "show-at-tablet";
pub const CLASSES_SHOW_AT_HD: &str = "show-at-hd";
pub const CLASSES_SHOW_AT_HDPLUS: &str = "show-at-hdplus";
pub const CLASSES_SHOW_AT_FULLHD: &str = "show-at-fullhd";
pub const CLASSES_SHOW_AT_QUADHD: &str = "show-at-quadhd";
pub const CLASSES_SHOW_AT_WIDEQUADHD: &str = "show-at-widequadhd";
pub const CLASSES_SHOW_AT_4K: &str = "show-at-4k";

pub const CLASSES_HIDE_AT_MOBILE: &str = "hide-at-mobile";
pub const CLASSES_HIDE_AT_TABLET: &str = "hide-at-tablet";
pub const CLASSES_HIDE_AT_HD: &str = "hide-at-hd";
pub const CLASSES_HIDE_AT_HDPLUS: &str = "hide-at-hdplus";
pub const CLASSES_HIDE_AT_FULLHD: &str = "hide-at-fullhd";
pub const CLASSES_HIDE_AT_QUADHD: &str = "hide-at-quadhd";
pub const CLASSES_HIDE_AT_WIDEQUADHD: &str = "hide-at-widequadhd";
pub const CLASSES_HIDE_AT_4K: &str = "hide-at-4k";

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    pub(crate) fn hello_world() -> Html {
        html! (
            <Paper class={CLASSES_PAGE_SECTION}>
                {"Hello World"}
            </Paper>
        )
    }

    #[test]
    fn test_hello_world() {
        _ = hello_world();
    }
}
