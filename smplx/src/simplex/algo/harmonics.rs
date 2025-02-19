/*
    Appellation: harmonics <module>
    Contrib: @FL03
*/
use nalgebra::{Const, OVector, Point, RealField, Scalar};

/// Struct representing a vertex in the harmonic space.
/// Each vertex has a position (in some space), a phase angle, and an assigned frequency.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct HarmonicNode<T = f64, const N: usize = 2>
where
    T: RealField + Scalar,
{
    frequency: T,          // Assigned frequency from scale
    phase: T,              // Phase angle in radians
    position: Point<T, N>, // The position within n-dimensional space
}

/// A lazy evaluator for calculating the convex hull of a set of harmonic vertices.
pub struct HarmonicHull<T = f64, const N: usize = 2>
where
    T: RealField + Scalar,
    [HarmonicNode<T, N>; N + 1]: Sized,
{
    nodes: [HarmonicNode<T, N>; N + 1],
    weights: OVector<T, Const<{ N + 1 }>>,
}

mod impl_vertex {
    use super::HarmonicNode;
    use nalgebra::{Const, OPoint, Point, RealField, Scalar};

    impl<T, const N: usize> HarmonicNode<T, N>
    where
        T: RealField + Scalar,
    {
        pub fn new(frequency: T, phase: T, position: Point<T, N>) -> Self {
            Self {
                frequency,
                phase,
                position,
            }
        }

        pub const fn frequency(&self) -> &T {
            &self.frequency
        }

        pub const fn phase_angle(&self) -> &T {
            &self.phase
        }

        pub fn position(&self) -> &OPoint<T, Const<N>> {
            &self.position
        }
    }

    impl<T, const N: usize> core::fmt::Display for HarmonicNode<T, N>
    where
        T: core::fmt::Display + RealField + Scalar,
    {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            let Self {
                frequency,
                phase,
                position,
            } = &self;
            write!(f, "({frequency}, {phase}, {position:?})")
        }
    }
}

mod impl_hull {
    use super::{HarmonicHull, HarmonicNode};
    use nalgebra::{Const, OVector, Point, RealField, Scalar};

    impl<T, const N: usize> HarmonicHull<T, N>
    where
        T: RealField + Scalar,
        [T; N + 1]: Sized,
    {
        pub fn new(
            nodes: [HarmonicNode<T, N>; N + 1],
            weights: OVector<T, Const<{ N + 1 }>>,
        ) -> Self {
            Self { nodes, weights }
        }

        pub fn convex_hull(&self) -> HarmonicNode<T, N>
        where
            T: Copy + core::iter::Sum,
        {
            let HarmonicHull { nodes, weights } = &self;
            assert!(nodes.len() == weights.len());
            let sum_weights: T = weights.iter().copied().sum();
            approx::assert_abs_diff_eq!(sum_weights, T::one());
            let mut weighted_position = Point::<T, N>::origin();
            for n in 0..N {
                for (i, &vertex) in nodes.iter().enumerate() {
                    weighted_position[n] += vertex.position[n] * weights[i];
                }
            }

            let mut phase_real = T::zero();
            let mut phase_imag = T::zero();
            for (i, &vertex) in nodes.iter().enumerate() {
                let angle = vertex.phase;
                phase_real += weights[i] * angle.cos();
                phase_imag += weights[i] * angle.sin();
            }
            let interpolated_phase = phase_imag.atan2(phase_real);

            HarmonicNode::new(
                nodes[0].frequency, // Assuming same scale frequency for all
                interpolated_phase,
                weighted_position,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Point2, Vector3};

    /// Computes the harmonic hull using weighted phase interpolation.
    /// The output phase is the argument of the weighted sum of unit phasors.
    fn _harmonic_hull(
        vertices: &[HarmonicNode<f64, 2>],
        weights: &OVector<f64, Const<3>>,
    ) -> HarmonicNode<f64, 2> {
        assert!(vertices.len() == weights.len());
        let sum_weights: f64 = weights.iter().sum();
        assert!((sum_weights - 1.0).abs() < 1e-6, "Weights must sum to 1");

        // Compute the weighted sum of the positions
        let mut weighted_position = na::Point2::new(0.0, 0.0);
        for (i, vertex) in vertices.iter().enumerate() {
            weighted_position[0] += weights[i] * vertex.position[0];
            weighted_position[1] += weights[i] * vertex.position[1];
        }

        // Compute the weighted sum of phasors (complex representation of phase angles)
        let mut phase_real = 0.0;
        let mut phase_imag = 0.0;
        for (i, vertex) in vertices.iter().enumerate() {
            let angle = vertex.phase;
            phase_real += weights[i] * angle.cos();
            phase_imag += weights[i] * angle.sin();
        }
        let interpolated_phase = phase_imag.atan2(phase_real); // Phase interpolation

        // Return the harmonic hull point
        HarmonicNode {
            position: weighted_position,
            phase: interpolated_phase,
            frequency: vertices[0].frequency, // Assuming same scale frequency for all
        }
    }

    #[test]
    fn test_harmonic_hull() {
        let vertices = [
            HarmonicNode::new(1.0, 0.0, Point2::new(0.0, 0.0)),
            HarmonicNode::new(1.0, std::f64::consts::FRAC_PI_2, Point2::new(1.0, 0.0)),
            HarmonicNode::new(1.0, std::f64::consts::PI, Point2::new(1.0, 1.0)),
        ];
        let weights = Vector3::new(0.2, 0.5, 0.3);
        let exp = _harmonic_hull(&vertices, &weights);
        let hull = HarmonicHull::new(vertices, weights).convex_hull();
        assert_eq!(exp.frequency(), hull.frequency());
        assert_eq!(exp.phase_angle(), hull.phase_angle());
        #[cfg(feature = "approx")]
        approx::assert_abs_diff_eq!(exp.position(), hull.position());
    }
}
