/*
    Appellation: nasimplex <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::simplex::*;

mod simplex;
pub mod algo {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod barycentric;
    pub mod harmonics;

    pub(crate) mod prelude {
        pub use super::barycentric::*;
        pub use super::harmonics::*;
    }
}

mod impl_simplex;

#[cfg(test)]
mod tests {
    use super::algo::barycentric::dynbary;
    use approx::assert_relative_eq;
    use nalgebra::{Point2, Vector3, convert};

    #[test]
    fn test_nalgebra_barycentric() {
        let simplex = [
            Point2::new(3.0, 2.0),
            Point2::new(5.0, 3.0),
            Point2::new(3.0, 4.0),
        ];

        let position = Point2::new(0.3, 0.3);

        let coords = dynbary::<f64, 2>(&simplex, &position);

        assert_relative_eq!(
            coords,
            convert(Vector3::new(2.53, -1.35, -0.18)),
            epsilon = f64::EPSILON,
            max_relative = 0.1
        )
    }
}
