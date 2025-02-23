/*
    Appellation: complex <test>
    Contrib: @FL03
*/
use smplx::{NdSimplex, SimplexComplex};

#[test]
fn test_simplex_complex() {
    let mut complex = SimplexComplex::<f64, ()>::new();
    let simplex = NdSimplex::from_ndarray(ndarray::arr2(&[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]));
    complex.add_simplex(simplex);
    assert_eq!(complex.node_count(), 1);
    complex.remove_simplex(0.into());
    assert_eq!(complex.node_count(), 0);
}
