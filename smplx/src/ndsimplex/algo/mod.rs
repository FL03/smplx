/*
    Appellation: algo <module>
    Contrib: @FL03
*/
//! The module implements the algorithms for the simplex.
#[allow(unused_imports)]
#[doc(inline)]
pub use self::prelude::*;

// pub mod harmonic;
// #[cfg(feature = "rayon")]
pub mod quick_hull;

#[allow(unused_imports)]
pub(crate) mod prelude {
    // pub use super::harmonic::*;
    // #[cfg(feature = "rayon")]
    pub use super::quick_hull::*;
}
