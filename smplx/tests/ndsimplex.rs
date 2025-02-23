/*
    Appellation: ndsimplex <test>
    Contrib: @FL03
*/
use approx::assert_relative_eq;
use ndarray::array;
use smplx::NdSimplex;

#[test]
fn test_simplex_from_iter() {
    let vertices = vec![array![3.0, 2.0], array![5.0, 3.0], array![3.0, 4.0]];
    let simplex = NdSimplex::from_iter(vertices);
    assert_eq!(simplex.dim(), 2);
    assert_eq!(simplex.nodes().shape(), &[3, 2]);
    assert_eq!(simplex.nodes().get((0, 0)), Some(&3.0));
    assert_eq!(simplex.nodes().get((0, 1)), Some(&2.0));
    assert_eq!(simplex.nodes().get((1, 0)), Some(&5.0));
    assert_eq!(simplex.nodes().get((1, 1)), Some(&3.0));
    assert_eq!(simplex.nodes().get((2, 0)), Some(&3.0));
    assert_eq!(simplex.nodes().get((2, 1)), Some(&4.0));
}

#[test]
fn test_barycentric_coordinates() {
    let vertices = array![[3.0, 2.0], [5.0, 3.0], [3.0, 4.0]];
    let simplex = NdSimplex::from_ndarray(vertices);
    let point = array![0.3, 0.3];
    let barycentric = simplex.barycentric(point).expect("Barycentric coordinates");
    let expected = array![2.525, -1.35, -0.175];
    assert_relative_eq!(
        barycentric,
        expected,
        epsilon = f64::EPSILON,
        max_relative = 0.01
    )
}
