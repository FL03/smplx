/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # smplx
//!
//! smplx is a research project supporting the Flow protocol
#![crate_name = "smplx"]
#![crate_type = "lib"]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate nalgebra as na;

#[doc(inline)]
pub use self::{algo::prelude::*, error::*, simplex::*, traits::prelude::*};

pub mod error;
#[doc(hidden)]
pub mod harmonics;
#[cfg(feature = "ndarray")]
pub mod ndsimplex;
#[cfg(feature = "nalgebra")]
pub mod simplex;

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

    pub mod math;

    pub(crate) mod prelude {
        pub use super::math::*;
    }
}

pub mod prelude {
    pub use crate::error::*;
    pub use crate::simplex::*;

    pub use crate::algo::prelude::*;
    pub use crate::traits::prelude::*;
}
