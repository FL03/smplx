/*
    Appellation: binary <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::BaseState;

pub enum Unary {}

pub enum Binary {}

pub enum Ternary {}

pub enum Nary<const N: usize> {}

pub type BinaryState<T> = BaseState<Binary, T>;

pub type UnaryState<T> = BaseState<Unary, T>;

pub type NState<T, const N: usize = 4> = BaseState<Nary<N>, T>;
