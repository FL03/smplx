/*
    Appellation: simplex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Simplex
//!
//! A simplex is a fundamental topological object often used to construct larger abstractions.
//! Simplices are notable for being the smallest possible polytope in any dimension and are
//! generalizations of common geometric objects such as points, line segments, and triangles.
//! 
//! ## Definitions
//! 
//! "A k-simplex, is a k-dimensional polytope which is the convex hull of its k+1 vertices."
//! - [Wikipedia](https://en.wikipedia.org/wiki/Simplex)
//! 
//!
//! ### Regular Simplex
//!
//! A regular simplex is one which is also a regular polytope. Regular simplices may be 
//! constructed from a regular (n-1)-simplex by connecting a new vertex to all original 
//! vertices by common edge length.
//!
//! |        Shape | Regular Simplex |
//! |--------------|-----------------|
//! |     Point    |    0-simplex    |
//! | Line Segment |    1-simplex    |
//! |   Triangle   |    2-simplex    |
//! |  Tetrahedron |    3-simplex    |
//! |-|-|
//!
#[doc(inline)]
pub use self::object::Simplex;

mod object;

pub(crate) mod prelude {
    pub use super::object::Simplex;
}

pub trait Simplicial {}
