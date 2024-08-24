/*
    Appellation: set <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Simplicial Set
//! 
//! This module provides the necessary tools to create and manipulate simplicial sets.
//! 
#[doc(inline)]
pub use self::object::SimplicialSet;

mod object;

pub(crate) mod prelude {
    pub use super::object::SimplicialSet;
}