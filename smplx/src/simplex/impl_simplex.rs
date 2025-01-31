/*
    Appellation: simple <module>
    Contrib: @FL03
*/
use super::Simplex;
use crate::Factorial;
use nalgebra::{ComplexField, DMatrix, Scalar};



impl<T> Simplex<T> {
    /// Constructs a new simplex from an (N+1) x N matrix
    pub fn new(n: usize, vertices: DMatrix<T>) -> Self {
        assert_eq!(vertices.nrows(), n + 1, "Simplex must have N+1 vertices.");
        assert_eq!(
            vertices.ncols(),
            n,
            "Each vertex must be in N-dimensional space."
        );
        Self {
            dim: n,
            pointset: vertices,
        }
    }

    pub fn from_iterator<I>(n: usize, vertices: I) -> Self where I: IntoIterator<Item = T>, T: Scalar {
        let vertices = DMatrix::from_iterator(n + 1, n, vertices);
        Self::new(n, vertices)
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
        self.pointset.determinant().abs() / T::from_usize(self.dim.factorial()).unwrap()
    }
}

