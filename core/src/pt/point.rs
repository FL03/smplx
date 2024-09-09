/*
    Appellation: point <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::RawNode;

pub struct Container<S>
where
    S: RawNode,
{
    pub(crate) data: S,
}

impl<A, S> Container<S>
where
    S: RawNode<Elem = A>,
{
    pub(crate) fn new(data: S) -> Self {
        Self { data }
    }

    pub fn as_ptr(&self) -> *const A {
        self.data.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut A {
        self.data.as_mut_ptr()
    }
}
