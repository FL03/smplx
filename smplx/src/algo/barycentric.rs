/*
    Appellation: barycentric <module>
    Contrib: @FL03
*/

use nalgebra::{Point2, RealField, Scalar, Vector3};
use nalgebra::{DMatrix, DVector, Point};

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

// #[derive(Debug)]
// pub struct Barycentric<S = f64> {
//     coords: Vector3<S>,
// }

// impl<S> Barycentric<S> {
//     pub fn from_cartesian(simplex: &[Vector2<S>], point: &Vector2<S>) -> Self
//     where
//         S: Copy + RealField + Scalar + num::Num,
//     {
//         Barycentric {
//             coords: barycentric_coordinates(simplex, point),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use nalgebra::{Point2, Vector3};
    
    #[test]
    fn test_barycentric() {
        let simplex = vec![
            Point2::new(3.0, 2.0),
            Point2::new(5.0, 3.0),
            Point2::new(3.0, 4.0),
        ];
    
        let position = Point2::new(0.3, 0.3);
        
        let coords = barycentric(&simplex, &position);
        
        assert_relative_eq!(coords, Vector3::new(2.53, -1.35, -0.18), epsilon = f64::EPSILON, max_relative = 0.1)
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
        
        let coords = dynbary(&simplex, &position);
        
        assert_relative_eq!(coords, na::convert(Vector3::new(2.53, -1.35, -0.18)), epsilon = f64::EPSILON, max_relative = 0.1)
    }
}