/*
    Appellation: ndsimplex <test>
    Contrib: @FL03
*/
use approx::assert_relative_eq;
use smplx::ndsimplex::NdSimplex;
    
#[test]
fn test_barycentric_coordinates() {
    use ndarray::array;
    let vertices = array![[3.0, 2.0], [5.0, 3.0], [3.0, 4.0]];
    let simplex = NdSimplex::from_ndarray(vertices);
    let point = array![0.3, 0.3];
    let barycentric = simplex.barycentric(point);
    let expected = array![2.53, -1.35, -0.18];
    assert_relative_eq!(
        barycentric,
        expected,
        epsilon = f64::EPSILON,
        max_relative = 0.01
    )
}