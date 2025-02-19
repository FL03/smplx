/*
    Appellation: quick_hull <mod>
    Contrib: @FL03
*/
use ndarray::{Array1, Array2, ArrayView1, Axis, Ix, NdFloat};

/// A lazy evaluation of the QuickHull algorithm for computing the convex hull of a set of
/// points within a simplex;
#[derive(Clone, Debug)]
pub struct QuickHull<A> {
    points: Array2<A>,
}
impl<A> QuickHull<A> {
    /// Constructs a new QuickHull instance from a set of points.
    pub fn new(points: Array2<A>) -> Self {
        Self { points }
    }

    /// Computes the convex hull using the QuickHull algorithm.
    pub fn compute(&self) -> Array2<A>
    where
        A: NdFloat,
    {
        let mut hull_indices = Vec::new();

        if self.points.nrows() <= 3 {
            // If we have ≤3 points, they already form a convex hull
            return self.points.clone();
        }

        // Step 1: Find extreme points along an axis (x-axis here)
        let (min_idx, max_idx) = self.find_extremes();

        // Step 2: Partition points into two sets
        let (above, below) = self.partition_points(min_idx, max_idx);

        // Step 3: Recursively build the convex hull
        self.quickhull_recursive(min_idx, max_idx, &above, &mut hull_indices);
        self.quickhull_recursive(max_idx, min_idx, &below, &mut hull_indices);

        // Step 4: Collect unique points forming the hull
        let hull_points = hull_indices
            .into_iter()
            .map(|idx| self.points.row(idx).to_owned())
            .collect::<Vec<_>>();

        let unique_hull_points = self.remove_duplicates(hull_points);

        ndarray::stack(
            Axis(0),
            &unique_hull_points.iter().map(|x| x.view()).collect::<Vec<_>>(),
        )
        .expect("Failed to stack points into an array")
    }

    fn remove_duplicates(&self, points: Vec<Array1<A>>) -> Vec<Array1<A>>
    where
        A: NdFloat,
    {
        let mut unique_points = Vec::new();
        for point in points {
            if !unique_points.iter().any(|p| p == &point) {
                unique_points.push(point);
            }
        }
        unique_points
    }

    fn quickhull_recursive(&self, a: usize, b: usize, subset: &[usize], hull: &mut Vec<usize>)
    where
        A: NdFloat,
    {
        // If no points remain, just record the edge from a to b
        if subset.is_empty() {
            if !hull.contains(&a) {
                hull.push(a);
            }
            if !hull.contains(&b) {
                hull.push(b);
            }
            return;
        }

        // Find the farthest point from baseline (a,b) among subset
        let maybe_farthest = subset.iter().max_by(|&&p1, &&p2| {
            self.signed_distance(
                &self.points.row(a),
                &self.points.row(b),
                &self.points.row(p1),
            )
            .partial_cmp(&self.signed_distance(
                &self.points.row(a),
                &self.points.row(b),
                &self.points.row(p2),
            ))
            .unwrap()
        });

        // If we can't find a strictly "farther" point, all subset points are collinear with (a,b)
        let farthest = match maybe_farthest {
            Some(&idx) => idx,
            None => {
                // Just keep endpoints a, b
                if !hull.contains(&a) {
                    hull.push(a);
                }
                if !hull.contains(&b) {
                    hull.push(b);
                }
                return;
            }
        };

        // Check whether the farthest point is actually collinear with (a,b)
        let cross = self.signed_distance(
            &self.points.row(a),
            &self.points.row(b),
            &self.points.row(farthest),
        );
        if cross.abs() < A::epsilon() {
            // If collinear, just keep (a,b) in the hull
            if !hull.contains(&a) {
                hull.push(a);
            }
            if !hull.contains(&b) {
                hull.push(b);
            }
            return;
        }

        // Partition only the subset on each side
        let (above_a, _) = self.partition_subset(a, farthest, subset);
        let (_, above_b) = self.partition_subset(farthest, b, subset);

        // Recursively process each side
        self.quickhull_recursive(a, farthest, &above_a, hull);
        self.quickhull_recursive(farthest, b, &above_b, hull);
    }

    /// Computes the signed distance of a point from a baseline (a, b).
    fn signed_distance(&self, a: &ArrayView1<A>, b: &ArrayView1<A>, p: &ArrayView1<A>) -> A
    where
        A: NdFloat,
    {
        // For 2D arrays:
        let ab_x = b[0] - a[0];
        let ab_y = b[1] - a[1];
        let ap_x = p[0] - a[0];
        let ap_y = p[1] - a[1];
        // Cross product z-component: ab_x * ap_y - ab_y * ap_x
        ab_x * ap_y - ab_y * ap_x
    }

    /// Finds the indices of the extreme points along a given dimension.
    fn find_extremes(&self) -> (Ix, Ix)
    where
        A: NdFloat,
    {
        let col_x = self.points.column(0);
        let min_idx = col_x
            .indexed_iter()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        let max_idx = col_x
            .indexed_iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        (min_idx, max_idx)
    }

    /// Skip points with cross_product == 0 to avoid keeping all collinear points.
    fn partition_points(&self, min_idx: Ix, max_idx: Ix) -> (Vec<usize>, Vec<usize>)
    where
        A: NdFloat,
    {
        let min_point = self.points.row(min_idx);
        let max_point = self.points.row(max_idx);

        let mut above = Vec::new();
        let mut below = Vec::new();

        for (i, p) in self.points.axis_iter(Axis(0)).enumerate() {
            if i == min_idx || i == max_idx {
                continue;
            }
            let cross_product = self.signed_distance(&min_point, &max_point, &p);
            if cross_product > A::zero() {
                above.push(i);
            } else if cross_product < A::zero() {
                below.push(i);
            }
            // If cross_product == 0, skip to avoid extra collinear points in the hull.
        }
        (above, below)
    }

    fn partition_subset(&self, a: Ix, b: Ix, subset: &[usize]) -> (Vec<usize>, Vec<usize>)
    where
        A: NdFloat,
    {
        let min_point = self.points.row(a);
        let max_point = self.points.row(b);

        let mut above = Vec::new();
        let mut below = Vec::new();

        for &i in subset {
            if i == a || i == b {
                continue;
            }
            let cross_product = self.signed_distance(&min_point, &max_point, &self.points.row(i));
            if cross_product > A::zero() {
                above.push(i);
            } else if cross_product < A::zero() {
                below.push(i);
            }
        }
        (above, below)
    }
}