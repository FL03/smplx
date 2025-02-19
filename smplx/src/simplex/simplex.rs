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

use na::Const;
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

pub struct Simplex<T, D>
where
    D: DimName,
    T: Scalar,
    DefaultAllocator: Allocator<D>,
{
    pub(crate) nodes: Vec<OPoint<T, D>>,
}

impl<T, D> Simplex<T, D>
where
    D: DimName,
    T: Scalar,
    DefaultAllocator: Allocator<D>,
{
    pub fn new(nodes: Vec<OPoint<T, D>>) -> Self {
        assert_eq!(nodes.len(), D::dim() + 1, "Simplex must have N+1 vertices.");
        Self { nodes }
    }

    pub fn barycentric(&self, point: OPoint<T, D>) -> DVector<T>
    where
        T: Copy + RealField,
        D: na::DimAdd<na::U1>,
        [(); D::USIZE + 1]: Sized,
    {
        let dim = D::dim();
        let n = D::dim() + 1;
        // verify the shape of the simplex
        assert_eq!(
            n,
            self.len(),
            "A simplex contains exactly N+1 n-dimensional points."
        );
        // initialize a matrix for the simplex points
        let mut matrix = DMatrix::zeros(n, n);
        // initialize a vector for the right-hand side
        let mut rhs = OVector::<T, Const<{D::USIZE + 1}>>::zeros();
        // Fill matrix with simplex points (homogeneous form)
        for i in 0..=dim {
            for j in 0..dim {
                matrix[(j, i)] = self.nodes[i][j];
            }
            matrix[(dim, i)] = T::one(); // Homogeneous coordinate row
        }

        // Right-hand side vector (homogeneous point)
        for j in 0..dim {
            rhs[j] = point[j];
        }
        rhs[dim] = T::one(); // Homogeneous coordinate

        // Solve for barycentric coordinates
        let res = matrix
            .lu()
            .solve(&rhs).unwrap_or(na::SVector::<T, { D::USIZE + 1 }>::zeros());

    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn nodes(&self) -> &[na::OPoint<T, D>] {
        &self.nodes
    }

    pub fn dim(&self) -> usize {
        D::dim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nsimplex() {
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
}
