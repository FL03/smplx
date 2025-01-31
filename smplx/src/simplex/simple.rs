/*
    Appellation: simple <module>
    Contrib: @FL03
*/
use crate::Factorial;
use nalgebra::{ComplexField, DMatrix, Scalar};

/// A simplex is a generalization of the notion of a triangle or tetrahedron to arbitrary
/// dimensions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Simplex<T> {
    dim: usize,
    pointset: DMatrix<T>, // (N+1) x N matrix
}

impl<T> Simplex<T>
where
    T: Scalar + num::Num + num::traits::FromPrimitive,
{
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

    pub fn from_iterator<I: IntoIterator<Item = T>>(n: usize, vertices: I) -> Self {
        let vertices = DMatrix::from_iterator(n + 1, n, vertices);
        Self::new(n, vertices)
    }
    /// Computes the volume of the n-simplex using determinant
    ///
    pub fn volume(&self) -> T
    where
        T: ComplexField<RealField = T>,
    {
        self.pointset.determinant().abs() / T::from_usize(self.dim.factorial()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplex() {
        let vertices = [0.0, 0.0, 0.0, 1.0, -1.0, 0.0];
        let _simplex = Simplex::from_iterator(2, vertices);
    }
}
