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

mod impl_simplex;

use nalgebra::{Point, Scalar};

pub struct DSimplex<T, const D: usize> where T: Scalar {
    nodes: Vec<Point<T, D>>
}



/// A simplex is a generalization of the notion of a triangle or tetrahedron to arbitrary
/// dimensions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Simplex<T> {
    dim: usize,
    angles: nalgebra::DMatrix<T>, // (N+1) x N matrix
    nodes: nalgebra::DMatrix<T>,  // (N+1) x N matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplex() {
        // vertices: [0, 0], [0, 1], [1, 0]
        let angles = vec![0f64, 0f64, 0f64, 0f64, 0f64, 0f64];
        let vertices = vec![0f64, 0f64, 0f64, 1f64, 1f64, 0f64];

        let simplex = Simplex::from_row_iterator(2, angles.clone(),  vertices.clone());

        for (i, j) in [(0, 0), (0, 1), (1, 0), (1, 1), (2, 0), (2, 1)]
            .iter()
            .zip(vertices.iter())
        {
            println!("{i:?}: {:?}", simplex.get_vertex(i.0));
            assert_eq!(simplex.get_vertex(i.0)[i.1], *j);
        }
    }
}
