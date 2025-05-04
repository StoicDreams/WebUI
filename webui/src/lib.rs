//! # Web UI
//!
//! `webui` is a website framework for building webassembly SPA websites quickly and easily.
//! Development is just getting started, so we do not recommend using at this point for anything more than experimenting.
#![allow(unused)] // TODO: Remove me when needing to check for dead code / unused methods/variables.
use std::cell::Cell;

/// Constants used throughout the application
pub mod constants;
/// Errors
pub mod errors;
/// Global methods and helpers
pub mod global;
/// Shortcut for all common components
pub mod prelude;

#[macro_use]
extern crate webui_procs;
pub use webui_procs::*;

/// Generalized macros
#[macro_use]
pub mod macros;
/// Javascript interop and related macros
#[macro_use]
pub mod interop;

pub use prelude::*;
