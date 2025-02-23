/*
    Appellation: impl_ndsimplex <module>
    Contrib: @FL03
*/
use crate::algo::QuickHull;
use crate::simplex::NdSimplex;
use ndarray::{Array1, Array2, ArrayView1, NdFloat};
use ndarray_linalg::{Lapack, Scalar, Solve};
use num::{One, Zero};

impl<A> NdSimplex<A> {
    /// initializes a new simplex as a single point in 0-dimensional space.
    pub fn new() -> Self
    where
        A: Default,
    {
        Self {
            dim: 0,
            nodes: Array2::default((1, 0)),
        }
    }
    /// constructs a new simplex from an iterator of _points_. By definition, each vertex or
    /// point within a simplex is a one-dimensional set of coordinates. Therefore, we can
    /// construct a simplex from an iterator of one-dimensional arrays.
    pub fn from_iter<I>(iter: I) -> Self
    where
        A: Clone + Default,
        I: IntoIterator<Item = Array1<A>>,
    {
        let iter = iter.into_iter();
        let len = iter.size_hint().0;
        let dim = len - 1;
        let mut nodes = Array2::default((len, dim));
        nodes
            .outer_iter_mut()
            .zip(iter)
            .for_each(|(mut row, point)| {
                row.assign(&point);
            });

        Self { dim, nodes }
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
        A: Clone + One,
    {
        let nodes = Array2::ones((dim + 1, dim));
        Self { dim, nodes }
    }
    /// creates a simplex with the given dimension and all zeros
    pub fn zeros(dim: usize) -> Self
    where
        A: Clone + Zero,
    {
        let nodes = Array2::zeros((dim + 1, dim));
        Self { dim, nodes }
    }
    /// get the dimension of the simplex
    pub fn dim(&self) -> usize {
        self.dim
    }
    /// get an immutable reference to the nodes of the simplex
    pub fn nodes(&self) -> &Array2<A> {
        &self.nodes
    }
    /// get a mutable reference to the nodes of the simplex
    pub fn nodes_mut(&mut self) -> &mut Array2<A> {
        &mut self.nodes
    }
    /// a method for checking if the simplex is valid
    pub fn is_valid(&self) -> bool {
        self.nodes.nrows() == self.dim + 1 && self.nodes.ncols() == self.dim
    }
    /// get the vertex at the given index
    pub fn get_vertex(&self, index: usize) -> Option<ArrayView1<'_, A>> {
        if index < self.nodes.nrows() {
            Some(self.nodes.row(index))
        } else {
            None
        }
    }
    /// calculate the barycentric coordinates of the given point with respect to the simplex
    pub fn barycentric(&self, point: Array1<A>) -> ndarray_linalg::error::Result<Array1<A>>
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
        matrix.solve(&rhs)
    }
    /// a lazy evaluator for computing the convex hull of the simplex using the QuickHull
    /// algorithm
    pub fn quickhull(&self) -> QuickHull<'_, A> {
        QuickHull::new(self.nodes.view())
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

impl<A> Default for NdSimplex<A>
where
    A: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<A> From<Array2<A>> for NdSimplex<A> {
    fn from(nodes: Array2<A>) -> Self {
        Self::from_ndarray(nodes)
    }
}

impl<A> From<NdSimplex<A>> for Array2<A> {
    fn from(simplex: NdSimplex<A>) -> Self {
        simplex.nodes
    }
}
