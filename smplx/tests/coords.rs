/*
    Appellation: coords <module>
    Contrib: @FL03
*/

use smplx::algo::barycentric::*;

use approx::assert_relative_eq;
use nalgebra::{Point2, Vector3, convert};

#[test]
fn test_barycentric() {
    let simplex = vec![
        Point2::new(3.0, 2.0),
        Point2::new(5.0, 3.0),
        Point2::new(3.0, 4.0),
    ];

    let position = Point2::new(0.3, 0.3);

    let coords = barycentric(&simplex, &position);

    assert_relative_eq!(
        coords,
        Vector3::new(2.53, -1.35, -0.18),
        epsilon = f64::EPSILON,
        max_relative = 0.1
    )
}

// #[ignore = "n-dimensional cases aren't working yet"]
#[test]
fn test_dynamic_barycentric() {
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
