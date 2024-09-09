/*
    Appellation: pt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Point (pt)
//!
//!

#[doc(inline)]
pub use self::point::*;

pub mod point;
pub mod repr;

pub(crate) mod prelude {
    pub use super::point::*;
}

pub unsafe trait RawNode {
    type Elem;
}

pub trait Node: RawNode {}
