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

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{error::*, ndsimplex::NdSimplex, traits::prelude::*};

pub mod algo;
pub mod error;
pub mod ndsimplex;
// pub mod simplex;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod math;

    pub(crate) mod prelude {
        pub use super::math::*;
    }
}

pub mod prelude {
    pub use crate::algo::prelude::*;
    pub use crate::error::*;
    pub use crate::ndsimplex::NdSimplex;
    pub use crate::traits::prelude::*;
}
