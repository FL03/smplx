/*
    Appellation: barycentric <module>
    Contrib: @FL03
*/

use nalgebra::DMatrix;
use nalgebra::{Const, DimName, OPoint, Point, Point2, RealField, SVector, Scalar, Vector3};

pub fn bary<T, D>(
    simplex: &DMatrix<T>,
    point: &Point<T, {D::USIZE}>,
) -> SVector<T, { D::USIZE + 1 }>
where
    D: DimName,
    T: Copy + RealField + Scalar,
{
    let dim = D::USIZE;
    let n = dim + 1;
    // verify the shape of the simplex
    assert_eq!(
        dim + 1,
        simplex.len(),
        "A simplex contains exactly N+1 n-dimensional points."
    );
    // initialize a matrix for the simplex points
    let mut matrix = DMatrix::zeros(n, n);
    // initialize a vector for the right-hand side
    let mut rhs = SVector::<T, { D::USIZE + 1 }>::zeros();
    // Fill matrix with simplex points (homogeneous form)
    for i in 0..=dim {
        for j in 0..dim {
            matrix[(j, i)] = simplex[(i, j)];
        }
        matrix[(dim, i)] = T::one(); // Homogeneous coordinate row
    }

    // Right-hand side vector (homogeneous point)
    for j in 0..dim {
        rhs[j] = point[j];
    }
    rhs[dim] = T::one(); // Homogeneous coordinate

    // Solve for barycentric coordinates
    matrix
        .lu()
        .solve(&rhs)
        .unwrap_or(SVector::<T, { D::USIZE + 1 }>::zeros())
}

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
