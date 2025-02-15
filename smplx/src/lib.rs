/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # smplx
//!
//! smplx is a research project supporting the Flow protocol
#![crate_name = "smplx"]
#![crate_type = "lib"]

#![feature(generic_const_exprs)]
#[cfg(feature = "alloc")]
extern crate alloc;

extern crate nalgebra as na;

#[doc(inline)]
pub use self::{algo::prelude::*, error::*, point::Point, simplex::Simplex, traits::prelude::*};

pub mod error;
pub mod point;
pub mod simplex;

#[doc(hidden)]
pub mod state;

pub mod algo {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod barycentric;

    pub(crate) mod prelude {
        pub use super::barycentric::*;
    }
}

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod hkt;
    pub mod math;

    pub(crate) mod prelude {
        pub use super::math::*;
    }
}

pub mod prelude {
    pub use crate::error::*;
    pub use crate::point::*;
    pub use crate::simplex::*;

    pub use crate::algo::prelude::*;
    pub use crate::traits::prelude::*;
}
