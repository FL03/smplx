/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Overloadable Operations
//!
//! This module implements the operational mechanisms for the library.
#[doc(inline)]
pub use self::glue::*;

mod glue;

pub(crate) mod prelude {}
