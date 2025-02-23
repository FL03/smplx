/*
    Appellation: impl_ndsimplex <module>
    Contrib: @FL03
*/
use crate::algo::QuickHull;
use crate::simplex::NdSimplex;
use ndarray::{Array1, Array2, NdFloat};
use ndarray_linalg::{Lapack, Scalar, Solve};

impl<A> NdSimplex<A> {
    pub fn new() -> Self
    where
        A: Default,
    {
        Self {
            dim: 0,
            nodes: Array2::default((1, 0)),
        }
    }
    /// constructs a new simplex from an (N+1) x N matrix
    pub fn from_ndarray(nodes: Array2<A>) -> Self {
        assert_eq!(
            nodes.nrows(),
            nodes.ncols() + 1,
            "Simplex must have N+1 vertices."
        );
        let dim = nodes.ncols();
        Self { dim, nodes }
    }
    /// creates a simplex with the given dimension and all ones
    pub fn ones(dim: usize) -> Self
    where
        A: Clone + num::One,
    {
        let nodes = Array2::ones((dim + 1, dim));
        Self { dim, nodes }
    }
    /// creates a simplex with the given dimension and all zeros
    pub fn zeros(dim: usize) -> Self
    where
        A: Clone + num::Zero,
    {
        let nodes = Array2::zeros((dim + 1, dim));
        Self { dim, nodes }
    }

    /// calculate the barycentric coordinates of the given point with respect to the simplex
    pub fn barycentric(&self, point: Array1<A>) -> Array1<A>
    where
        A: Lapack + Scalar + NdFloat,
    {
        let dim: usize = self.dim;
        let npoints: usize = self.nodes.nrows();
        assert_eq!(
            point.len(),
            dim,
            "The point must have the same dimension as the simplex."
        );
        // initialize a matrix for the simplex points
        let mut matrix = Array2::zeros((npoints, npoints));
        // initialize a vector for the right-hand side
        let mut rhs = Array1::zeros((npoints,));
        // Fill matrix with simplex points (homogeneous form)
        for i in 0..=dim {
            for j in 0..dim {
                matrix[(j, i)] = self.nodes[[i, j]];
            }
            matrix[(dim, i)] = A::one(); // Homogeneous coordinate row
        }
        // Right-hand side vector (homogeneous point)
        for j in 0..dim {
            rhs[j] = point[j];
        }
        rhs[dim] = A::one(); // Homogeneous coordinate
        // Solve for barycentric coordinates
        matrix.solve(&rhs).unwrap_or(Array1::zeros(npoints))
    }
    /// a lazy evaluator for computing the convex hull of the simplex using the QuickHull
    /// algorithm
    pub fn quickhull(&self) -> QuickHull<A>
    where
        A: NdFloat,
    {
        QuickHull::new(self.nodes.clone())
    }
}

impl<A> core::ops::Deref for NdSimplex<A> {
    type Target = Array2<A>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

impl<A> core::ops::DerefMut for NdSimplex<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.nodes
    }
}
