/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # space
//! 
//! The `space` module considers abstract topological computing environments. 
//! Using the Wolfram [2, 3] UTM as justification for the assertion that triads
//! are objects of computation, the `space` module provides a set of tools for
//! proving the theorems.
//! 
//! 
//! unit of computability is considered to be a triad 
#[doc(inline)]
pub use self::{
    state::BinaryState,
    venv::TopoVenv,
};

pub mod state;
pub mod venv;

#[cfg(feature = "petgraph")]
pub type TriadGraph = petgraph::Graph<String, u8, petgraph::Directed>;
///
pub type TriadId = String;

pub(crate) mod prelude {
    pub use super::state::BinaryState;
    pub use super::venv::TopoVenv;
    #[cfg(feature = "petgraph")]
    pub use super::TriadGraph;
    pub use super::TriadId;
}
