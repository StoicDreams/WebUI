pub mod drawers;
pub mod navigation;

use crate::prelude::*;
use std::{
    any::Any,
    collections::HashMap,
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex},
};

pub use drawers::*;
pub use navigation::*;

use crate::AppConfig;

pub(crate) trait DynamicContext: Any {
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> DynamicContext for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ToString for dyn DynamicContext {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Debug for dyn DynamicContext {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let me = self.as_any();
        write!(f, "{:?}", me)
    }
}

impl PartialEq for dyn DynamicContext {
    fn eq(&self, other: &dyn DynamicContext) -> bool {
        self.type_id() == other.type_id() && self.to_string() == other.to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Contexts {
    pub config: AppConfig,
    pub page_loaded: UseStateHandle<String>,
    pub data: UseStateHandle<Option<String>>,
    pub nav: UseStateHandle<NavigationMessage>,
    pub drawer: UseStateHandle<DrawerMessage>,
    #[cfg(feature = "myfi")]
    pub user: Arc<UseStateHandle<Option<MyFiUser>>>,
}
