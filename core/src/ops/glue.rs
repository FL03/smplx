/*
    Appellation: glue <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Glue] is a trait defining the ability for two simplicial objects to be combined.
pub trait Glue<Rhs = Self> {
    type Output;

    fn glue(self, rhs: Rhs) -> Self::Output;
}
