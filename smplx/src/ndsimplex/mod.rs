
#[doc(inline)]
pub use self::{algo::prelude::*, simplex::NdSimplex};

pub mod algo;
pub mod simplex;

mod impls {
    mod impl_ndsimplex;
}