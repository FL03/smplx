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

#[doc(inline)]
pub use self::{algo::prelude::*, error::*, simplex::*, traits::prelude::*};

pub mod error;
pub mod simplex;

pub mod algo {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod quick_hull;

    #[allow(unused_imports)]
    pub(crate) mod prelude {
        pub use super::quick_hull::*;
    }
}

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
    pub use crate::simplex::NdSimplex;
    pub use crate::traits::prelude::*;
}
