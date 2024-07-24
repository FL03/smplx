/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::TriadId;
use crate::state::State;
use rstmt::neo::triad::{Major, Triad};

#[derive(Clone)]
pub struct TopoVenv<T, Q, K = Major> {
    pub(crate) id: TriadId,
    pub(crate) triad: Triad<K>,
    pub(crate) state: State<T, Q>,
}

impl<T, Q, K> TopoVenv<T, Q, K> {
    
}
