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

use na::{Const, SVector, U1};
use nalgebra::{DMatrix, DVector, OPoint, OVector, RealField};
use nalgebra::{DimName, Point, Scalar};
use nalgebra::{allocator::Allocator, default_allocator::DefaultAllocator};

pub struct NSimplex<T, const N: usize>
where
    T: Scalar,
    [T; N + 1]: Sized,
{
    pub(crate) nodes: [Point<T, N>; N + 1],
}

pub struct DynSimplex<T> {
    pub(crate) dim: usize,
    pub(crate) nodes: DMatrix<T>,
}

impl<T> DynSimplex<T> {
    pub fn new(dim: usize,) -> Self where T: Scalar + num::Zero {
        let nodes = DMatrix::zeros(dim + 1, dim);
        Self { dim, nodes }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn nodes(&self) -> &DMatrix<T> {
        &self.nodes
    }

    pub fn dim(&self) -> usize {
        self.dim
    }
}

impl<T> core::ops::Deref for DynSimplex<T>
where
    T: Scalar,
{
    type Target = DMatrix<T>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_na_simplex() {
        use nalgebra::Point2;
        // vertices: [0, 0], [0, 1], [1, 0]
        let vertices = [
            Point2::new(0f64, 0f64),
            Point2::new(0f64, 1f64),
            Point2::new(1f64, 0f64),
        ];

        let triangle = NSimplex::new(vertices);

        assert_eq!(triangle.dim(), vertices.len() - 1);
    }

    #[test]
    fn test_dyn_simplex() {
        use nalgebra::{DMatrix, Point2};
        // vertices: [0, 0], [0, 1], [1, 0]
        let vertices = [
            Point2::new(0f64, 0f64),
            Point2::new(0f64, 1f64),
            Point2::new(1f64, 0f64),
        ];

        // let vertices = DMatrix::from_iterator(3, 2, vertices.iter().cloned());

        let simplex = DynSimplex::<f64>::new(2);

        assert_eq!(simplex.dim(), 2);
    }
}
