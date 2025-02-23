/*
    Appellation: ndsimplex <module>
    Contrib: @FL03
*/

mod impl_complex;
mod impl_ndsimplex;

/// The `NdSimplex` struct represents an n-dimensional simplex constructed using the ndarray
/// crate. The vertices, or nodes, of the simplex are stored in a 2-dimensional array where
/// each row represents an individual vertex and each column represents a dimension. Each
/// simplex contains N + 1 n-dimensional points, where N is the dimensionality of the simplex.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
pub struct NdSimplex<A = f64> {
    pub(crate) dim: usize,
    pub(crate) nodes: ndarray::Array2<A>,
}

/// A simplicial complex is a collection of simplicies that are connected in a specific way.
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
pub struct SimplicialComplex<A = f64, E = ()> {
    pub(crate) simplices: petgraph::graph::DiGraph<NdSimplex<A>, E>,
}
