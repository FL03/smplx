/*
    Appellation: quick_hull <test>
    Contrib: @FL03
*/
use ndarray::{Axis, arr2, array};
use smplx::ndsimplex::{NdSimplex, QuickHull};

#[test]
fn test_small_input() {
    let points = arr2(&[[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]]);
    let hull = NdSimplex::from_ndarray(points.clone()).quickhull().compute();
    // For three points, the hull should just match the original set
    assert_eq!(hull, points, "Hull should match all points for small input");
}

#[test]
fn test_square_points() {
    let points = arr2(&[
        [0.0, 0.0],
        [2.0, 0.0],
        [2.0, 2.0],
        [0.0, 2.0],
        [1.0, 1.0], // inside the square
    ]);
    let hull = QuickHull::new(points).compute();
    // Expected hull: corners of the square
    // Check that it contains (0,0), (2,0), (2,2), (0,2)
    let expected_corners = vec![
        array![0.0, 0.0],
        array![2.0, 0.0],
        array![2.0, 2.0],
        array![0.0, 2.0],
    ];
    for corner in expected_corners {
        assert!(
            hull.axis_iter(Axis(0)).any(|row| row == corner),
            "Hull should contain square corner {:?}",
            corner
        );
    }
}

#[test]
fn test_collinear_points() {
    // Points on a line at x=1, y from 0 to 3
    let points = arr2(&[[1.0, 0.0], [1.0, 1.0], [1.0, 2.0], [1.0, 3.0]]);
    let hull = QuickHull::new(points).compute();
    // For collinear points, the hull should be the endpoints
    assert_eq!(
        hull.nrows(),
        2,
        "Hull for collinear points should have 2 endpoints"
    );
}

#[test]
fn test_repeated_points() {
    let points = arr2(&[[0.0, 0.0], [0.0, 0.0], [1.0, 1.0], [1.0, 1.0]]);
    let hull = QuickHull::new(points.clone()).compute();
    // Even with duplicates, the hull is the line's endpoints
    assert_eq!(hull.nrows(), 2, "Hull should be the two unique endpoints.");
}

#[ignore = "internal points are not handled correctly"]
#[test]
fn test_triangle_points() {
    let points = arr2(&[
        [1.0, 0.0],
        [3.0, 0.0],
        [2.0, 2.0],
        [2.0, 1.0], // inside the triangle
    ]);
    let hull = QuickHull::new(points).compute();
    assert_eq!(
        hull.nrows(),
        3,
        "A triangle hull should have exactly three vertices."
    );
}

#[test]
fn test_offset_rectangle_points() {
    // Rectangle corners at (-3, -2), (2, -2), (2, 2), (-3, 2)
    let points = arr2(&[
        [-3.0, -2.0],
        [2.0, -2.0],
        [2.0, 2.0],
        [-3.0, 2.0],
        [0.0, 0.0],  // inside the rectangle
        [1.0, 1.0],  // inside
        [-2.0, 1.0], // inside
    ]);
    let hull = QuickHull::new(points).compute();
    let expected_corners = vec![
        array![-3.0, -2.0],
        array![2.0, -2.0],
        array![2.0, 2.0],
        array![-3.0, 2.0],
    ];
    for corner in expected_corners {
        assert!(
            hull.axis_iter(Axis(0)).any(|row| row == corner),
            "Hull should contain corner {:?}",
            corner
        );
    }
}

#[test]
fn test_negative_coords() {
    // Points forming a small polygon in negative space
    let points = arr2(&[
        [-2.0, -2.0],
        [-4.0, -1.0],
        [-3.0, -3.0], // inside
        [-1.0, -3.0],
        [-2.5, -1.5], // inside
    ]);
    let hull = QuickHull::new(points).compute();
    // Expect 4 corners
    assert_eq!(hull.nrows(), 4, "Hull should consist of 4 vertices.");
}
