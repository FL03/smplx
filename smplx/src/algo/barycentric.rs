/*
    Appellation: barycentric <module>
    Contrib: @FL03
*/

use nalgebra::DMatrix;
use nalgebra::{Point, Point2, RealField, SVector, Scalar, Vector3};

/// [dynbary] computes the barycentric coordinates of a point in an n-simplex.
pub fn dynbary<T, const D: usize>(
    simplex: &[Point<T, D>; D + 1],
    point: &Point<T, D>,
) -> SVector<T, { D + 1 }>
where
    T: Copy + RealField + Scalar,
{
    let n = D + 1;
    // verify the shape of the simplex
    assert_eq!(
        D + 1,
        simplex.len(),
        "A simplex contains exactly N+1 n-dimensional points."
    );
    // initialize a matrix for the simplex points
    let mut matrix = DMatrix::zeros(n, n);
    // initialize a vector for the right-hand side
    let mut rhs = SVector::<T, { D + 1 }>::zeros();
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
    

    matrix
        .lu()
        .solve(&rhs)
        .unwrap_or(SVector::<T, { D + 1 }>::zeros())
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
