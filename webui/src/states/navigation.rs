#[derive(Clone, Debug, PartialEq, Default)]
pub enum NavigationMessage {
    #[default]
    None,
    Refresh,
    PathUpdate(String),
}
