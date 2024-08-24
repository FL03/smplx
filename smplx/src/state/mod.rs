/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::{kinds::*, state::*};

pub(crate) mod state;

pub(crate) mod kinds {
    pub use self::binary::*;

    mod binary;
}

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::state::*;
}

/// A type alias for a binary state
pub type BinaryState<T = ()> = State<Binary, T>;

/// [Stateful]
pub trait Stateful<T> {
    type State: RawState<Data = T>;
}
/// [RawState]
pub trait RawState {
    type Data;
}

/*
 ************* Implementations *************
*/
impl<Q, T> RawState for State<Q, T> {
    type Data = T;
}

impl<Q, T> Stateful<T> for State<Q, T> {
    type State = State<Q, T>;
}
