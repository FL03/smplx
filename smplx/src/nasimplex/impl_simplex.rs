/*
    Appellation: simple <module>
    Contrib: @FL03
*/
use super::NSimplex;
use crate::simplex::algo::barycentric::dynbary;
use nalgebra::{Point, Scalar};

impl<T, const N: usize> NSimplex<T, N>
where
    T: Scalar,
    [Point<T, N>; N + 1]: Sized,
{
    /// Constructs a new simplex from an (N+1) x N matrix
    pub fn new(nodes: [Point<T, N>; N + 1]) -> Self {
        assert_eq!(nodes.len(), N + 1, "Simplex must have N+1 vertices.");
        Self { nodes }
    }

    pub fn try_from_iter<I>(iter: I) -> crate::Result<Self>
    where
        I: IntoIterator<Item = Point<T, N>>,
    {
        let iter = iter.into_iter();
        let (n, _) = iter.size_hint();
        Vec::from_iter(iter)
            .try_into()
            .map(|nodes| Self::new(nodes))
            .map_err(|_| crate::SimplexError::IncompatibleDimension {
                expected: N + 1,
                found: n,
            })
    }

    pub const fn dim(&self) -> usize {
        N
    }

    pub fn len(&self) -> usize {
        N + 1
    }
    /// returns an immutable slice of the vertices
    pub const fn vertices(&self) -> &[Point<T, N>; N + 1] {
        &self.nodes
    }

    pub fn convert_cartesian_to_barycentric(&self, point: Point<T, N>) -> na::SVector<T, { N + 1 }>
    where
        T: Copy + na::RealField,
    {
        dynbary(self.vertices(), &point)
    }
}
