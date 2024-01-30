use crate::AppDrawerOptions;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum DrawerMessage {
    #[default]
    None,
    Close,
    ToggleDrawer(AppDrawerOptions),
    Setup(AppDrawerOptions),
}
