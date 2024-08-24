/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, state::*};

pub(crate) mod state;

pub(crate) mod kinds {
    pub use self::binary::*;

    mod binary;
}

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::state::*;
}

