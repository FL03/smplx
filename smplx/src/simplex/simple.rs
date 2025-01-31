/*
    Appellation: simple <module>
    Contrib: @FL03
*/
use crate::Factorial;
use nalgebra::{ComplexField, Matrix2, Scalar};


/// A simplex is a generalization of the notion of a triangle or tetrahedron to arbitrary 
/// dimensions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Simplex<T> {
    dim: usize,
    vertices: Matrix2<T>, // (N+1) x N matrix
}

impl<T> Simplex<T>
where
    T: Scalar + num::Num + num::traits::FromPrimitive,
{
    /// Constructs a new simplex from an (N+1) x N matrix
    pub fn new(n: usize, vertices: Matrix2<T>) -> Self {
        assert_eq!(vertices.nrows(), n + 1, "Simplex must have N+1 vertices.");
        assert_eq!(
            vertices.ncols(),
            n,
            "Each vertex must be in N-dimensional space."
        );
        Self { dim: n, vertices }
    }
    /// Computes the volume of the n-simplex using determinant
    ///
    pub fn volume(&self) -> T
    where
        T: ComplexField<RealField = T>,
    {
        self.vertices.determinant().abs() / T::from_usize(self.dim.factorial()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simplex() {
        let vertices = Matrix2::from_row_iterator([0.0, 0.0, 0.0, 1.0, -1.0, 0.0]);
        let _simplex = Simplex::new(2, vertices);
    }
}
