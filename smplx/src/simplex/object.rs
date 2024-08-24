/*
    Appellation: object <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct Simplex<V, E> {
    /// The set of edges in the simplex.
    pub(crate) edges: Vec<E>,
    /// The set of vertices in the simplex.
    pub(crate) nodes: Vec<V>,
}

impl<V, E> Simplex<V, E> {
    pub fn new() -> Self {
        Self { edges: Vec::new(), nodes: Vec::new() }
    }

    pub fn from_edges(edges: Vec<E>) -> Self {
        Self { edges, nodes: Vec::new() }
    }
}