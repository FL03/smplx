/*
    Appellation: simplex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Simplex
//! 
//! This module provides the necessary tools to create and manipulate simplexes.
//! 
#[doc(inline)]
pub use self::object::Simplex;

mod object;

pub(crate) mod prelude {
    pub use super::object::Simplex;
}