/*
    Appellation: simple <module>
    Contrib: @FL03
*/

use nalgebra::{ComplexField, Matrix2, Scalar};

pub trait Factorial {
    fn factorial(&self) -> Self;
}

macro_rules! impl_factorial {
    ($($t:ty),* $(,)?) => {
        $(
            impl Factorial for $t {
                fn factorial(&self) -> Self {
                    (1..=*self).fold(1, |acc, x| acc * x)
                }
            }
        )*
    };
}

impl_factorial!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// Represents an n-simplex in an n-dimensional space.
#[derive(Debug, Clone, Eq, PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Simplex<T> {
    dim: usize,
    vertices: Matrix2<T>, // (N+1) x N matrix
}

impl<T> Simplex<T>
where
    T: core::fmt::Debug + Scalar + num::Num + num::traits::FromPrimitive,
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
