/*
    Appellation: point <module>
    Contrib: @FL03
*/

mod impl_point;

pub trait RawPoint {
    type Ctx;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Point<S>
where
    S: RawPoint,
{
    data: S,
    _ctx: core::marker::PhantomData<S::Ctx>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        let p = Point::new(42u8);
        
        assert_eq!(*p.data(), 42u8);
        assert_eq!(*p.map(|i| i + 1).data(), 43u8);

        let p = Point::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(*p.data(), vec![1.0, 2.0, 3.0]);
    }
}
