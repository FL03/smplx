/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # State
//!
//! This module contains the stateful types and traits for the library.
#[doc(inline)]
pub use self::{base_state::*, kinds::*};

mod base_state;
mod kinds;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::base_state::*;
    pub use super::kinds::*;
}

/// [Stateful]
pub trait Stateful<T> {
    type State: RawState<Inner = T>;
}
/// [RawState]
pub trait RawState {
    type Inner;
}

/*
 ************* Implementations *************
*/
impl<Q, T> RawState for BaseState<Q, T> {
    type Inner = T;
}

impl<Q, T> Stateful<T> for BaseState<Q, T> {
    type State = BaseState<Q, T>;
}
