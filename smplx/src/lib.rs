/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # smplx
//!
//! smplx is a research project supporting the Flow protocol
#![crate_name = "smplx"]


pub mod space;
pub mod state;

pub mod prelude {
    pub use super::space::prelude::*;
}
