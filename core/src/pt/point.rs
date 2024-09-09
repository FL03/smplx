/*
    Appellation: point <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::RawNode;

/// [Node] represents any point within
pub struct Point<S>
where
    S: RawNode,
{
    pub(crate) data: S,
    
}

impl<A, S> Point<S>
where
    S: RawNode<Elem = A>,
{
    pub(crate) fn new(data: S) -> Self {
        Self { data }
    }
}
