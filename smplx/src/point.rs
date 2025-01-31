/*
    Appellation: point <module>
    Contrib: @FL03
*/

pub trait RawPoint {
    type Data;
}

pub struct Point<S>
where
    S: RawPoint,
{
    data: S,
    _ctx: core::marker::PhantomData<S::Data>,
}

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

    pub fn data(&self) -> &S {
        &self.data
    }
}

macro_rules! raw_scalar {
    ($($t:ty),*) => {
        $(
            impl RawPoint for $t {
                type Data = ();
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
    type Data = T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p = Point::new(42u8);
        assert_eq!(*p.data(), 42u8);

        let p = Point::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(*p.data(), vec![1.0, 2.0, 3.0]);
    }
}
