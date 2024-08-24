/*
    Appellation: simplex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Simplex
//!
//! A simplex is defined to be the smallest polytope in any dimension, consisting of the convex hull of its vertices.
//!
//! ## Regular Simplex
//!
//! A regular simplex is one which is also a regular polytope; listed below are several
//! exampels of regular simplices:
//!
//! - Point: 0-simplex
//! - Line segment: 1-simplex
//! - Triangle: 2-simplex
//! - Tetrahedron: 3-simplex
//!
#[doc(inline)]
pub use self::object::Simplex;

mod object;

pub(crate) mod prelude {
    pub use super::object::Simplex;
}

pub trait Simplicial {}
