/*
    Appellation: n_simplex <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code)]

pub struct Node<T: ?Sized>(pub T);

pub struct NSimplex<T> {
    pub(crate) dim: usize,
    pub(crate) nodes: Vec<Node<T>>,
}

impl<T> NSimplex<T> {
    pub fn from_nodes<I>(iter: I) -> Self where I: IntoIterator<Item = Node<T>> {
        let nodes = Vec::from_iter(iter);
        Self { dim: nodes.len(), nodes }
    }
}
