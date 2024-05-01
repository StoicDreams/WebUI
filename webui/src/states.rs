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
    pub page_data: UseStateHandle<String>,
    pub app_data: UseStateHandle<HashMap<String, String>>,
    pub nav: UseStateHandle<NavigationMessage>,
    pub drawer: UseStateHandle<DrawerMessage>,
    pub user_roles: UseStateHandle<i32>,
    #[cfg(feature = "myfi")]
    pub user: UseStateHandle<Option<MyFiUser>>,
}

impl Contexts {
    pub fn get_app_data(&self, key: &str) -> String {
        let app_data = self.app_data.deref();
        match app_data.get(key) {
            Some(value) => value.to_owned(),
            None => String::default(),
        }
    }
    pub fn set_app_data(&self, key: &str, value: &str) -> &Self {
        let mut app_data = self.app_data.deref().clone();
        app_data.insert(key.to_string(), value.to_string());
        self.app_data.set(app_data);
        self
    }
}
