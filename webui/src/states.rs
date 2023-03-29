pub mod drawers;
pub mod navigation;

pub use drawers::*;
pub use navigation::*;
use yew::UseStateHandle;

use crate::AppConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct Contexts {
    pub config: AppConfig,
    pub nav: UseStateHandle<NavigationMessage>,
    pub drawer: UseStateHandle<DrawerMessage>,
}
