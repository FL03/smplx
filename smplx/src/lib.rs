/*
    Appellation: smplx <lib>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # smplx
//!
//! The `smplx` crate is a collection of algorithms and data structures for working with
//! simplices in n-dimensional space. The crate is designed to be flexible and generic,
//! allowing users to work with simplices in a variety of contexts.
#![crate_name = "smplx"]
#![crate_type = "lib"]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "nalgebra")]
extern crate nalgebra as na;

#[doc(inline)]
pub use self::{error::*, traits::prelude::*};

pub mod error;
#[cfg(feature = "ndarray")]
pub mod ndsimplex;
#[cfg(feature = "nalgebra")]
pub mod simplex;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod math;

    pub(crate) mod prelude {
        pub use super::math::*;
    }
}

pub mod prelude {
    pub use crate::error::*;
    pub use crate::traits::prelude::*;
}
