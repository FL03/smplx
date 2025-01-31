/*
    Appellation: math <traits>
    Contrib: @FL03
*/
pub trait Factorial {
    fn factorial(&self) -> Self;
}

/*
    ************* Implementations *************
*/
macro_rules! impl_factorial {
    ($($t:ty),* $(,)?) => {
        $(
            impl Factorial for $t {
                fn factorial(&self) -> Self {
                    (1..=*self).fold(1, |acc, x| acc * x)
                }
            }
        )*
    };
}

impl_factorial!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);