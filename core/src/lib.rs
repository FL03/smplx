/*
    Appellation: smplx-core <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # smplx-core
//!
//! smplx is a research project supporting the Flow protocol
#![crate_name = "smplx_core"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{error::Error, pt::Point};

pub mod error;
pub mod ops;
pub mod pt;
pub mod traits;
pub mod types;

pub mod prelude {

    pub use super::pt::prelude::*;
}
