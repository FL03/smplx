/*
    Appellation: object <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// # [Simplex]
/// 
/// A simplex is defined to be the smallest polytope in any dimension, consisting of the convex hull of its vertices.
/// 
/// A regular simplex is one which is also a regular polytope; listed below are several 
/// exampels of regular simplices:
/// 
/// - Point: 0-simplex
/// - Line segment: 1-simplex
/// - Triangle: 2-simplex
/// - Tetrahedron: 3-simplex
/// 
/// 
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Simplex<V, E> {
    pub(crate) dim: usize,
    /// The set of edges in the simplex.
    pub(crate) edges: Vec<E>,
    /// The set of vertices in the simplex.
    pub(crate) nodes: Vec<V>,
}

impl<V, E> Simplex<V, E> {
    pub fn new(dim: usize) -> Self {
        Self { dim, edges: Vec::new(), nodes: Vec::new() }
    }

    pub fn with_edges<I>(self, edges: I) -> Self where I: IntoIterator<Item = E> {
        Self { edges: Vec::from_iter(edges), ..self }
    }

    pub fn with_nodes<I>(self, nodes: I) -> Self where I: IntoIterator<Item = V> {
        Self { nodes: Vec::from_iter(nodes), ..self }
    }
}