/*
    Appellation: simplex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Simplex
//!
//! A simplex $\Delta^n$ is a fundamental topological objects known for being the smallest
//! (and simplest) polytopes in any dimension. Simplexes are often _glued_ together to form
//! simplicial complexes or sets.
//!
//! ## Definitions
//!
//! "A $k$-simplex, is a $k$-dimensional polytope which is the convex hull $C$ of its $k+1$ vertices."
//! - [Wikipedia](https://en.wikipedia.org/wiki/Simplex)
//!
//! The convex hull is given by the expression:
//!
//! $C = \biggl\{ \sum_{i=0}^{k} \lambda_i v_i \mid\lambda_i\geq{0}\text{ for all } j \text{ and }\sum_{i=0}^{k} \lambda_i = 1 \biggr\}$
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

pub trait Node {
    type Weight;
}

pub trait Link<A, B> {
    type Data;
}

pub trait Simplicial {
    type Dim;
}
