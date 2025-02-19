/*
    Appellation: ndsimplex <module>
    Contrib: @FL03
*/

use ndarray::{Array1, Array2, ScalarOperand};
use ndarray_linalg::{Lapack, Scalar, Solve};

/// The `NdSimplex` struct represents an n-dimensional simplex constructed using the ndarray 
/// crate. The vertices of the simplex are stored in a 2D array, where each row represents a
/// vertex and each column represents a dimension.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NdSimplex<A> {
    vertices: Array2<A>,
}

impl<A> NdSimplex<A> {
    pub fn from_ndarray(vertices: Array2<A>) -> Self {
        assert_eq!(vertices.nrows(), vertices.ncols() + 1, "Simplex must have N+1 vertices.");
        Self { vertices }
    }

    pub fn barycentric(&self, point: Array1<A>) -> Array1<A> where A: Lapack + Scalar + ScalarOperand + num::Num {
        let dim = self.vertices.ncols();
        let npoints = self.vertices.nrows();
        // verify the shape of the simplex
        assert_eq!(
            npoints,
            dim + 1,
            "A simplex contains exactly N+1 n-dimensional points."
        );
        // initialize a matrix for the simplex points
        let mut matrix = Array2::zeros((npoints, npoints));
        // initialize a vector for the right-hand side
        let mut rhs = Array1::zeros((npoints,));
        // Fill matrix with simplex points (homogeneous form)
        for i in 0..=dim {
            for j in 0..dim {
                matrix[(j, i)] = self.vertices[[i, j]];
            }
            matrix[(dim, i)] = A::one(); // Homogeneous coordinate row
        }

        // Right-hand side vector (homogeneous point)
        for j in 0..dim {
            rhs[j] = point[j];
        }
        rhs[dim] = A::one(); // Homogeneous coordinate

        // Solve for barycentric coordinates
        matrix
            .solve(&rhs)
            .unwrap_or(Array1::zeros(npoints))
    }

    pub const fn vertices(&self) -> &Array2<A> {
        &self.vertices
    }
}

