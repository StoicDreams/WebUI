use crate::{Direction, Html};

/// Struct used for defining details for displaying buttons that toggle drawer content.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawerToggleInfo {
    pub display: fn() -> Html,
    pub title: String,
    pub class: String,
    pub drawer: Direction,
    pub drawer_content: fn() -> Html,
}
