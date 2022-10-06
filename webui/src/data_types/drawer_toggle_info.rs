use yew::Html;

/// Struct used for defining details for displaying buttons that toggle drawer content.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawerToggleInfo {
    pub display: fn() -> Html,
    pub class: String,
    pub drawer_content: fn() -> Html,
}
