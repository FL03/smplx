/*
    Appellation: simple <module>
    Contrib: @FL03
*/
use super::*;
use crate::Factorial;
use nalgebra::{ComplexField, Const, DMatrix, Point, Scalar};

use nalgebra::base::{Dyn, Matrix, ViewStorage};

type RowView<'a, T> = Matrix<T, Const<1>, Dyn, ViewStorage<'a, T, Const<1>, Dyn, Const<1>, Dyn>>;

impl<T, const D: usize> NSimplex<T, D>
where
    T: Scalar,
    [Point<T, D>; D + 1]: Sized,
{
    /// Constructs a new simplex from an (N+1) x N matrix
    pub fn new(nodes: [Point<T, D>; D + 1]) -> Self {
        assert_eq!(nodes.len(), D + 1, "Simplex must have N+1 vertices.");
        Self { nodes }
    }

    pub fn try_from_iter<I>(iter: I) -> crate::Result<Self>
    where
        I: IntoIterator<Item = Point<T, D>>,
    {
        let iter = iter.into_iter();
        let (n, _) = iter.size_hint();
        Vec::from_iter(iter)
            .try_into()
            .map(|nodes| Self::new(nodes))
            .map_err(|_| crate::SimplexError::IncompatibleDimension {
                expected: D + 1,
                found: n,
            })
    }

    pub const fn dim(&self) -> usize {
        D
    }

    pub fn len(&self) -> usize {
        D + 1
    }

    pub const fn vertices(&self) -> &[Point<T, D>; D + 1] {
        &self.nodes
    }

    pub fn convert_cartesian_to_barycentric(&self, point: Point<T, D>) -> na::SVector<T, { D + 1 }>
    where
        T: Copy + na::RealField,
    {
        crate::algo::dynbary(self.vertices(), &point)
    }
}

impl<T> SimplexDyn<T> {
    /// Constructs a new simplex from an (N+1) x N matrix
    pub fn new(dim: usize, angles: DMatrix<T>, nodes: DMatrix<T>) -> Self {
        assert_eq!(nodes.nrows(), dim + 1, "Simplex must have N+1 vertices.");
        assert_eq!(
            nodes.ncols(),
            dim,
            "Each vertex must be in N-dimensional space."
        );
        Self { dim, angles, nodes }
    }

    pub fn ones(dim: usize) -> Self
    where
        T: Scalar + num::One,
    {
        let angles = DMatrix::from_element(dim + 1, dim, T::one());
        let nodes = DMatrix::from_element(dim + 1, dim, T::one());
        Self::new(dim, angles, nodes)
    }

    pub fn zeros(dim: usize) -> Self
    where
        T: Scalar + num::Zero,
    {
        let angles = DMatrix::zeros(dim + 1, dim);
        let nodes = DMatrix::zeros(dim + 1, dim);
        Self::new(dim, angles, nodes)
    }

    pub fn from_row_iterator<I, J>(n: usize, angles: I, vertices: J) -> Self
    where
        I: IntoIterator<Item = T>,
        J: IntoIterator<Item = T>,
        T: Scalar,
    {
        Self {
            dim: n,
            angles: DMatrix::from_row_iterator(n + 1, n, angles),
            nodes: DMatrix::from_row_iterator(n + 1, n, vertices),
        }
    }

    pub fn get_vertex(&self, i: usize) -> RowView<'_, T> {
        self.nodes.row(i)
    }

    pub fn complex_hull(&self, angles: DMatrix<T>) -> crate::Result<T>
    where
        T: Scalar + num::Num + num::traits::NumAssign + core::iter::Sum + core::fmt::Debug,
    {
        if angles.nrows() != self.dim + 1 || angles.ncols() != self.dim {
            return Err(crate::SimplexError::IncompatibleDimension {
                expected: (self.dim + 1) * self.dim,
                found: angles.len(),
            });
        }
        let total_angle: T = angles.iter().cloned().sum();
        if total_angle != T::zero() {
            return Err(crate::SimplexError::AngleError(format!(
                "Total angle must be zero, found: {total_angle:?}"
            )));
        }
        let res = angles.dot(&self.nodes);
        Ok(res)
    }

    pub fn vertices(&self) -> Vec<RowView<'_, T>>
    where
        T: Scalar,
    {
        self.nodes.row_iter().collect::<Vec<_>>()
    }
    /// Computes the volume of the n-simplex using the cayley-menger determinant
    ///
    /// The volume of an n-simplex is given by the formula:
    ///
    /// $V = \frac{1}{n!} \sqrt{|\Delta|^2}$
    pub fn volume(&self) -> T
    where
        T: ComplexField<RealField = T>,
    {
        self.nodes.determinant().abs() / T::from_usize(self.dim.factorial()).unwrap()
    }
}
