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
    pub app_data: HashMap<String, UseStateHandle<Option<String>>>,
    pub nav: UseStateHandle<NavigationMessage>,
    pub drawer: UseStateHandle<DrawerMessage>,
    pub user_roles: UseStateHandle<u32>,
    #[cfg(feature = "myfi")]
    pub user: UseStateHandle<Option<MyFiUser>>,
}

impl Contexts {
    pub fn get_data_handler(&self, name: &str) -> Option<UseStateHandle<Option<String>>> {
        match self.app_data.get_key_value(&name.to_string()) {
            Some(value) => Some(value.1.clone()),
            None => None,
        }
    }
    pub fn init_data_handler(
        &mut self,
        name: &str,
        value: UseStateHandle<Option<String>>,
    ) -> &Self {
        self.app_data.insert(name.to_string(), value);
        self
    }
    pub fn get_app_data(&self, name: &str) -> Option<String> {
        match self.app_data.get_key_value(&name.to_string()) {
            Some(value) => {
                let handler = value.1.clone();
                match &*handler {
                    Some(value) => Some(value.clone()),
                    None => None,
                }
            }
            None => None,
        }
    }
    pub fn set_app_data(&self, name: &str, value: &str) -> &Self {
        if let Some(handler) = self.app_data.get_key_value(&name.to_string()) {
            handler.1.set(Some(value.to_string()));
        }
        self
    }
    pub fn clear_app_data(&self, name: &str) -> &Self {
        if let Some(handler) = self.app_data.get_key_value(&name.to_string()) {
            handler.1.set(None);
        }
        self
    }
}
