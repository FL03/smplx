/*
    Appellation: impl_point <module>
    Contrib: @FL03
*/
use super::{Point, RawPoint};

type ScalarContainer = ();

impl<S> Point<S>
where
    S: RawPoint,
{
    pub fn new(data: S) -> Self {
        Self {
            data,
            _ctx: core::marker::PhantomData,
        }
    }
    /// returns an owned reference to the stored data
    pub fn data(&self) -> &S {
        &self.data
    }

    pub fn map<F, T>(self, f: F) -> Point<T>
    where
        F: FnOnce(S) -> T,
        T: RawPoint<Ctx = S::Ctx>,
    {
        Point::new(f(self.data))
    }
}

impl<S> Point<S> where S: RawPoint {}

macro_rules! raw_scalar {
    ($($t:ty),*) => {
        $(
            impl RawPoint for $t {
                type Ctx = ScalarContainer;
            }
        )*
    }
}

raw_scalar!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl<T> RawPoint for Vec<T>
where
    T: RawPoint,
{
    type Ctx = T;
}
