/*
    Appellation: harmonics <module>
    Contrib: @FL03
    Converted to use ndarray and NdSimplex
*/

use crate::NdSimplex;
use ndarray::{Array1, Array2, ScalarOperand};
use ndarray_linalg::{Lapack, Scalar};
use num::{Float, Zero};

/// A structure representing a single harmonic node with a frequency, a phase angle,
/// and a position vector (now using ndarray).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HarmonicNode<A = f64> {
    pub frequency: A,
    pub phase: A,
    pub position: Array1<A>,
}

impl<A> HarmonicNode<A> {
    pub fn new(frequency: A, phase: A, position: Array1<A>) -> Self {
        Self {
            frequency,
            phase,
            position,
        }
    }
}

/// A structure that leverages `NdSimplex` for the position data and stores
/// per-vertex phase and frequency in parallel arrays.
#[derive(Clone, Debug, PartialEq)]
pub struct HarmonicHull<A = f64> {
    /// Simplex holding the spatial positions of the vertices.
    pub simplex: NdSimplex<A>,
    /// Phase angle for each vertex.
    pub phases: Array1<A>,
    /// Frequency for each vertex.
    pub freqs: Array1<A>,
    /// Weights for the convex combination.
    pub weights: Array1<A>,
}

impl<A> HarmonicHull<A>
where
    A: Copy + Lapack + Scalar + ScalarOperand,
{
    /// Creates a new `HarmonicHull` from a list of nodes and a set of weights.
    /// Expects `nodes.len() == weights.len() == N+1`.
    pub fn new(nodes: &[HarmonicNode<A>], weights: Array1<A>) -> Self {
        let nverts = nodes.len();
        assert_eq!(
            nverts,
            weights.len(),
            "Number of nodes must match the number of weights."
        );
        // Dimension of each position vector
        let dim = nodes[0].position.len();
        // Create a matrix of shape (N+1, N) for NdSimplex
        let mut arr = Array2::<A>::zeros((nverts, dim));
        let mut phases = Array1::<A>::zeros(nverts);
        let mut freqs = Array1::<A>::zeros(nverts);

        // Fill the arrays from the harmonic nodes
        for (i, node) in nodes.iter().enumerate() {
            assert_eq!(
                dim,
                node.position.len(),
                "All nodes must have positions of the same dimension."
            );
            for d in 0..dim {
                arr[[i, d]] = node.position[d];
            }
            phases[i] = node.phase;
            freqs[i] = node.frequency;
        }

        let simplex = NdSimplex::from_ndarray(arr);
        Self {
            simplex,
            phases,
            freqs,
            weights,
        }
    }

    /// Computes a single interpolated `HarmonicNode` by combining phases and positions.
    pub fn convex_combination(&self) -> HarmonicNode<A>
    where
        A: Float,
    {
        let nverts = self.weights.len();
        let sum_weights: A = self.weights.iter().copied().sum();
        assert!(
            (sum_weights - A::one()).abs() < A::epsilon(),
            "Weights must sum to 1."
        );
        // Weighted sum of positions (using the simplex data directly)
        let dim = self.simplex.ncols();
        let mut weighted_pos = Array1::<A>::zeros(dim);
        for i in 0..nverts {
            for d in 0..dim {
                weighted_pos[d] = weighted_pos[d] + self.simplex[[i, d]] * self.weights[i];
            }
        }
        // Weighted sum of phases (convert each phase to a phasor)
        let mut phase_real = A::zero();
        let mut phase_imag = A::zero();
        for i in 0..nverts {
            let angle = self.phases[i];
            phase_real = phase_real + self.weights[i] * Float::cos(angle);
            phase_imag = phase_imag + self.weights[i] * Float::sin(angle);
        }
        let interpolated_phase = phase_imag.atan2(phase_real);

        // For consistency, assume frequency is the same across all nodes (take index 0)
        HarmonicNode::new(self.freqs[0], interpolated_phase, weighted_pos)
    }
}
