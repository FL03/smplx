/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

/// [State] is an abstract object that allows a particular _kind_ of state to be associated
/// with some data.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct State<Q, T> {
    pub(crate) data: T,
    pub(crate) _state: PhantomData<Q>,
}

impl<Q, T> State<Q, T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            _state: PhantomData::<Q>,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn is_state<R: 'static>(&self) -> bool
    where
        Q: 'static,
    {
        use core::any::TypeId;
        TypeId::of::<Q>() == TypeId::of::<R>()
    }
}
