/*
    Appellation: ndsimplex <module>
    Contrib: @FL03
*/

/// The `NdSimplex` struct represents an n-dimensional simplex constructed using the ndarray
/// crate. The vertices, or nodes, of the simplex are stored in a 2-dimensional array where
/// each row represents an individual vertex and each column represents a dimension. Each
/// simplex contains N + 1 n-dimensional points, where N is the dimensionality of the simplex.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NdSimplex<A = f64> {
    pub(crate) nodes: ndarray::Array2<A>,
}


