/*
    Appellation: barycentric <module>
    Contrib: @FL03
*/
use nalgebra::{RealField, Scalar, Vector2, Vector3};

pub fn barycentric_coordinates<T>(triangle: &[Vector2<T>], point: &Vector2<T>) -> Vector3<T>
where
    T: Copy + RealField + Scalar + num::Num,
{
    // let point = na::convert::<na::Point2<T>, Vector3<T>>(*point);
    // define the coordinate vectors
    let v0 = triangle[1] - triangle[0];
    let v1 = triangle[2] - triangle[0];
    let v2 = point - triangle[0];
    // precompute elements of the formula
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

#[derive(Debug)]
pub struct Barycentric<S = f64> {
    coords: Vector3<S>,
}

impl<S> Barycentric<S> {
    pub fn from_cartesian(simplex: &[Vector2<S>], point: &Vector2<S>) -> Self
    where
        S: Copy + RealField + Scalar + num::Num,
    {
        Barycentric {
            coords: barycentric_coordinates(simplex, point),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use nalgebra::{Point2, Vector3};
    
    #[test]
    fn test_barycentric() {
        let simplex = vec![
            Vector2::from_vec(vec![3.0, 2.0]),
            Vector2::from_vec(vec![5.0, 3.0]),
            Vector2::from_vec(vec![3.0, 4.0]),
        ];
    
        let position = Vector2::new(0.3, 0.3);
        
        let coords = barycentric_coordinates(&simplex, &position);
        
        assert_relative_eq!(coords, Vector3::new(2.53, -1.35, -0.18), epsilon = f64::EPSILON, max_relative = 0.1)
    }
}