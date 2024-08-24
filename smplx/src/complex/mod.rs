/*
    Appellation: complex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Simplicial Complex
//!
//! This module provides the necessary tools to create and manipulate simplicial complexes.
//!
#[doc(inline)]
pub use self::object::SimplicialComplex;

mod object;

pub(crate) mod prelude {
    pub use super::object::SimplicialComplex;
}
