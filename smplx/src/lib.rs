/*
    Appellation: rstopo <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # smplx
//!
//! smplx is a research project supporting the Flow protocol
#![crate_name = "smplx"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use self::{simplex::Simplex, traits::prelude::*};

pub mod point;
pub mod simplex;

#[doc(hidden)]
pub mod state;

pub mod traits {
    pub use self::prelude::*;

    pub mod math;

    pub(crate) mod prelude {
        pub use super::math::*;
    }
}

pub mod prelude {
    pub use crate::simplex::prelude::*;
    pub use crate::traits::prelude::*;
}
