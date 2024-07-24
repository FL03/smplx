/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct State<T, Q> {
    pub(crate) data: T,
    pub(crate) _state: PhantomData<Q>,
}

impl<T, Q> State<T, Q> {
    pub fn new(data: T) -> Self {
        Self { data, _state: PhantomData::<Q> }
    }

    pub fn data(&self) -> &T {
        &self.data
    }



    pub fn is_state<Q2: 'static>(&self) -> bool where Q: 'static {
        use core::any::TypeId;
        TypeId::of::<Q>() == TypeId::of::<Q2>()
    }
}
