/*
    Appellation: impl_complex <module>
    Contrib: @FL03
*/

use crate::simplex::{NdSimplex, SimplexComplex};
use petgraph::graph::{DiGraph, NodeIndex};

impl<A, E> SimplexComplex<A, E> {
    pub fn new() -> Self {
        Self {
            simplices: DiGraph::new(),
        }
    }
    /// inserts a simplex into the complex
    pub fn add_simplex(&mut self, simplex: NdSimplex<A>) -> NodeIndex {
        self.simplices.add_node(simplex)
    }
    /// get the dimension of the complex; the dimension of the complex is defined to be the
    /// maximum dimension of any contained simplices.
    pub fn dimension(&self) -> usize {
        self.simplices
            .node_weights()
            .map(|simplex| simplex.dim())
            .max()
            .unwrap_or(0)
    }
    /// Removes a simplex from the complex
    pub fn remove_simplex(&mut self, simplex: NodeIndex) -> Option<NdSimplex<A>> {
        self.simplices.remove_node(simplex)
    }
    /// Checks if all simplices in this complex are valid
    pub fn validate_complex(&self) -> bool {
        self.simplices
            .node_weights()
            .all(|simplex| simplex.is_valid())
    }
}

impl<A, E> Default for SimplexComplex<A, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A, E> core::ops::Deref for SimplexComplex<A, E> {
    type Target = DiGraph<NdSimplex<A>, E>;

    fn deref(&self) -> &Self::Target {
        &self.simplices
    }
}

impl<A, E> core::ops::DerefMut for SimplexComplex<A, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.simplices
    }
}
