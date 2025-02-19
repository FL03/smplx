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
    hull: Vec<usize>,
}

impl<A> QuickHull<A> {
    /// Constructs a new QuickHull instance from a set of points.
    pub fn new(points: Array2<A>) -> Self {
        Self { points, hull: Vec::new() }
    }

    /// Computes the convex hull using the QuickHull algorithm.
    pub fn compute(&mut self) -> Array2<A>
    where
        A: NdFloat,
    {
        if self.points.nrows() <= 3 {
            return self.remove_duplicates(self.points.axis_iter(Axis(0)).map(|row| row.to_owned()).collect());
        }

        let (min_idx, max_idx) = self.find_extremes();

        let (above, below) = self.partition_points(min_idx, max_idx);

        self.quickhull_recursive(min_idx, max_idx, &above);
        self.quickhull_recursive(max_idx, min_idx, &below);

        let hull_points = self.hull.iter().map(|&idx| self.points.row(idx).to_owned()).collect::<Vec<_>>();
        self.remove_duplicates(hull_points)
    }

    fn remove_duplicates(&self, points: Vec<Array1<A>>) -> Array2<A>
    where
        A: NdFloat,
    {
        let mut unique_points = Vec::new();
        for point in points {
            if !unique_points.iter().any(|p| p == &point) {
                unique_points.push(point);
            }
        }
        ndarray::stack(
            Axis(0),
            &unique_points.iter().map(|x| x.view()).collect::<Vec<_>>(),
        )
        .expect("Failed to stack points into an array")
    }

    fn quickhull_recursive(&mut self, a: usize, b: usize, subset: &[usize])
    where
        A: NdFloat,
    {
        if subset.is_empty() {
            self.record_edge(a, b);
            return;
        }

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

        let farthest = match maybe_farthest {
            Some(&idx) => idx,
            None => {
                self.record_edge(a, b);
                return;
            }
        };

        let cross = self.signed_distance(
            &self.points.row(a),
            &self.points.row(b),
            &self.points.row(farthest),
        );
        if cross.abs() < A::epsilon() {
            self.record_edge(a, b);
            return;
        }

        let (above_a, _) = self.partition_subset(a, farthest, subset);
        let (_, above_b) = self.partition_subset(farthest, b, subset);

        self.quickhull_recursive(a, farthest, &above_a);
        self.quickhull_recursive(farthest, b, &above_b);
    }

    fn record_edge(&mut self, a: usize, b: usize) {
        let edge = (a.min(b), a.max(b));
        if !self.hull.contains(&edge.0) {
            self.hull.push(edge.0);
        }
        if !self.hull.contains(&edge.1) {
            self.hull.push(edge.1);
        }
    }

    fn signed_distance(&self, a: &ArrayView1<A>, b: &ArrayView1<A>, p: &ArrayView1<A>) -> A
    where
        A: NdFloat,
    {
        let ab_x = b[0] - a[0];
        let ab_y = b[1] - a[1];
        let ap_x = p[0] - a[0];
        let ap_y = p[1] - a[1];
        ab_x * ap_y - ab_y * ap_x
    }

    fn find_extremes(&self) -> (Ix, Ix)
    where
        A: NdFloat,
    {
        let col_x = self.points.column(0);
        let min_idx = col_x.indexed_iter().min_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
        let max_idx = col_x.indexed_iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
        (min_idx, max_idx)
    }

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