/*
    Appellation: glue <module>
    Contrib: @FL03
*/

/// A trait for _gluing_ two types together, a behavior formally discussed within the context 
/// of algebraic topology. For example, the gluing of two simplices is a fundamental operation 
/// in the construction of a simplicial complex.
pub trait Glue<Rhs> {
    type Output;

    fn glue(self, rhs: Rhs) -> Self::Output;
}