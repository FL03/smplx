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

    fn as_ptr(&self) -> *const Self::Elem;

    fn as_mut_ptr(&mut self) -> *mut Self::Elem;

    fn as_slice(&self) -> &[Self::Elem];

    fn as_mut_slice(&mut self) -> &mut [Self::Elem];

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize;
}

pub trait Point: RawNode {}
