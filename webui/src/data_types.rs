/// Module holding data types for app configuration
pub mod app_config;

/// Module holding data types for direction values
pub mod direction;

/// Module for DynHtml type
pub mod dyn_html;

/// Module holding custom error types
pub mod errors;

/// Format methods
pub mod format;

/// Module holding data types for navigation routing
pub mod nav_route;

/// Constants for default role assignments
pub mod roles;

/// Module for holding Theme enum
pub mod theme;

/// Module for POC work
mod poc;

pub use crate::data_types::app_config::*;
pub use crate::data_types::direction::*;
pub use crate::data_types::dyn_html::*;
pub use crate::data_types::errors::*;
pub use crate::data_types::format::*;
pub use crate::data_types::nav_route::*;
pub use crate::data_types::roles::*;
pub use crate::data_types::theme::*;
