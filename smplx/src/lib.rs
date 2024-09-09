/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # smplx
//!
//! smplx is a research project supporting the Flow protocol
#![crate_name = "smplx"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::simplex::Simplex;
#[doc(inline)]
pub use smplx_core::*;

pub mod complex;
pub mod set;
pub mod simplex;

pub mod prelude {
    pub use super::complex::prelude::*;
    pub use super::set::prelude::*;
    pub use super::simplex::prelude::*;
    pub use smplx_core::prelude::*;
}
