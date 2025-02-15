/*
    Appellation: barycentric <module>
    Contrib: @FL03
*/

use nalgebra::{DMatrix, DVector, Point};
use nalgebra::{Point2, RealField, Scalar, Vector3};

use crate::NSimplex;

/// [dynbary] computes the barycentric coordinates of a point in an n-simplex.
pub fn dynbary<T, const D: usize>(simplex: &[Point<T, D>; D + 1], point: &Point<T, D>) -> DVector<T>
where
    T: Copy + RealField + Scalar,
{
    assert_eq!(D + 1, simplex.len(), "The number of vertices must be N+1.");

    let mut matrix = DMatrix::zeros(D + 1, D + 1);
    let mut rhs = DVector::zeros(D + 1);

    // Fill matrix with simplex points (homogeneous form)
    for i in 0..=D {
        for j in 0..D {
            matrix[(j, i)] = simplex[i][j];
        }
        matrix[(D, i)] = T::one(); // Homogeneous coordinate row
    }

    // Right-hand side vector (homogeneous point)
    for j in 0..D {
        rhs[j] = point[j];
    }
    rhs[D] = T::one(); // Homogeneous coordinate

    // Solve for barycentric coordinates
    let bary_coords = matrix.lu().solve(&rhs).unwrap_or(DVector::zeros(D + 1));

    bary_coords
}

pub fn barycentric<T>(simplex: &[Point2<T>], point: &Point2<T>) -> Vector3<T>
where
    T: Copy + RealField + Scalar + num::Num,
{
    // define the coordinate vectors
    let v0 = simplex[1] - simplex[0];
    let v1 = simplex[2] - simplex[0];
    let v2 = point - simplex[0];
    // calculate the denominator
    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);
    // calculate the denominator
    let denom = d00 * d11 - d01 * d01;
    // use the results above to find the barycentric coordinats (u, v, w)
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = T::one() - v - w;

    Vector3::new(u, v, w)
}
// pub struct Barycentric <'a, T, const N: usize> {
//     simplex: &'a NSimplex<T, N>
// }
