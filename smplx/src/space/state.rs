/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::binary::BinaryState;

pub mod binary;

use rstmt::neo::triad::Triads;

pub trait BinState {
    fn is(&self) -> bool;
}

