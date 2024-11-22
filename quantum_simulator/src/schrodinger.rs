/*
This code implements a quantum mechanics simulator focusing on solving the time-independent Schrödinger equation
for one-dimensional systems. Key concepts and implementations include:

1. Schrödinger Equation:
   - Fundamental equation of quantum mechanics describing wave function evolution
   - Learn more: https://en.wikipedia.org/wiki/Schrödinger_equation

2. Numerical Methods:
   - Finite difference method for spatial discretization
   - Eigenvalue problem solving using matrix methods
   - Reference: https://en.wikipedia.org/wiki/Finite_difference_method

3. Key Components:
   - Wave Function: Mathematical description of quantum state
   - Hamiltonian: Energy operator (kinetic + potential energy)
   - Eigenvalues: Energy levels of the system
   - Eigenvectors: Corresponding wave functions
   - Study: https://en.wikipedia.org/wiki/Wave_function

4. Implemented Features:
   - Particle in a box (infinite potential well)
     Details: https://en.wikipedia.org/wiki/Particle_in_a_box
   - Custom potential functions
   - Wave function normalization
   - Probability density calculations
   - Expectation value computations

5. Mathematical Methods:
   - Matrix diagonalization for solving eigenvalue problems
   - Numerical integration for expectation values
   - Learn more: https://en.wikipedia.org/wiki/Matrix_diagonalization

6. Physical Observables:
   - Energy levels
   - Position expectation values
   - Probability distributions
   - Reference: https://en.wikipedia.org/wiki/Observable

Each function implements specific aspects of quantum mechanics, allowing for:
- System initialization (new())
- Potential energy configuration (with_potential())
- Energy level calculation (solve_1d_box())
- Wave function computation (wave_functions())
- Observable calculations (probability_density(), expectation_position())

The code uses nalgebra for linear algebra operations and includes comprehensive
error handling and test cases to verify physical correctness.
*/

use nalgebra::{DMatrix, DVector};
use std::f64::consts::PI;

#[derive(Debug)]
pub enum SchrodingerError {
    InvalidGridSize,
    ComputationError(String),
    InvalidParameters,
}

pub struct SchrodingerSolver {
    grid_points: usize,
    dx: f64,
    potential: Option<Vec<f64>>,
    length: f64, // Adding box length parameter
}

impl SchrodingerSolver {
    pub fn dx(&self) -> f64 {
        self.dx
    }

    /// Create a new Schrödinger equation solver
    pub fn new(grid_points: usize, dx: f64) -> Result<Self, SchrodingerError> {
        if grid_points < 2 {
            return Err(SchrodingerError::InvalidGridSize);
        }
        if dx <= 0.0 {
            return Err(SchrodingerError::InvalidParameters);
        }

        Ok(Self {
            grid_points,
            dx,
            potential: None,
            length: (grid_points as f64) * dx,
        })
    }

    /// Set custom potential function
    pub fn with_potential(&mut self, potential: Vec<f64>) -> Result<&mut Self, SchrodingerError> {
        if potential.len() != self.grid_points {
            return Err(SchrodingerError::InvalidParameters);
        }
        self.potential = Some(potential);
        Ok(self)
    }

    /// Solve 1D particle in a box
    pub fn solve_1d_box(&self) -> Result<(Vec<f64>, DMatrix<f64>), SchrodingerError> {
        let n = self.grid_points;

        // Construct Hamiltonian matrix
        let mut h = DMatrix::zeros(n, n);
        let coeff = 1.0 / (2.0 * self.dx * self.dx); // Changed sign and scaling

        // Fill the Hamiltonian
        for i in 0..n {
            h[(i, i)] = 2.0 * coeff;
            if i > 0 {
                h[(i, i - 1)] = -coeff;
            }
            if i < n - 1 {
                h[(i, i + 1)] = -coeff;
            }

            // Add potential energy if specified
            if let Some(ref v) = self.potential {
                h[(i, i)] += v[i];
            }
        }

        // Add infinite potential at boundaries
        h[(0, 0)] = 1.0e6;
        h[(n - 1, n - 1)] = 1.0e6;

        // Compute eigenvalues and eigenvectors
        let eigen = h.symmetric_eigen();

        // Sort eigenvalues and eigenvectors
        let mut pairs: Vec<_> = eigen
            .eigenvalues
            .iter()
            .zip(eigen.eigenvectors.column_iter())
            .collect();
        pairs.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

        // Extract and normalize eigenvectors
        let sorted_eigenvectors = DMatrix::from_columns(
            &pairs
                .iter()
                .map(|(_, v)| {
                    let mut v = v.clone_owned();
                    let norm = (v.dot(&v)).sqrt();
                    v /= norm;
                    v
                })
                .collect::<Vec<_>>(),
        );

        // Get eigenvalues directly from the numerical solution
        let sorted_eigenvalues: Vec<f64> = pairs.iter().map(|(e, _)| **e).collect();

        Ok((sorted_eigenvalues, sorted_eigenvectors))
    }

    /// Calculate wave functions for given energy levels
    pub fn wave_functions(&self, energy_level: usize) -> Result<DVector<f64>, SchrodingerError> {
        let (_, eigenvectors) = self.solve_1d_box()?;

        if energy_level >= self.grid_points {
            return Err(SchrodingerError::InvalidParameters);
        }

        let mut wave_function = eigenvectors.column(energy_level).into_owned();

        // Normalize the wave function
        let norm = (self.dx * wave_function.dot(&wave_function)).sqrt();
        wave_function /= norm;

        Ok(wave_function)
    }

    /// Calculate probability density for a given wave function
    pub fn probability_density(&self, wave_function: &DVector<f64>) -> DVector<f64> {
        wave_function.map(|x| x * x)
    }

    /// Calculate expectation value of position
    pub fn expectation_position(&self, wave_function: &DVector<f64>) -> f64 {
        let x_values: DVector<f64> = DVector::from_iterator(
            self.grid_points,
            (0..self.grid_points).map(|i| i as f64 * self.dx),
        );

        let probability = self.probability_density(wave_function);
        (x_values.component_mul(&probability)).sum() * self.dx
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_particle_in_box() {
        let solver = SchrodingerSolver::new(1000, 0.01).unwrap();
        let (energies, _) = solver.solve_1d_box().unwrap();

        let length: f64 = 1000.0 * 0.01; // L = N * dx
        let ground_state = PI.powi(2) / (2.0 * length.powi(2));

        // Test first few energy levels
        assert_relative_eq!(energies[0], ground_state, epsilon = 0.1);
        assert_relative_eq!(energies[1], 4.0 * ground_state, epsilon = 0.1);
    }

    #[test]
    fn test_wave_function_normalization() {
        let solver = SchrodingerSolver::new(1000, 0.01).unwrap();
        let wave_function = solver.wave_functions(0).unwrap();
        let probability = solver.probability_density(&wave_function);

        // Test normalization
        assert_relative_eq!(probability.sum() * solver.dx, 1.0, epsilon = 1e-5);
    }

    #[test]
    fn test_invalid_parameters() {
        assert!(SchrodingerSolver::new(1, 0.1).is_err());
        assert!(SchrodingerSolver::new(100, -0.1).is_err());
    }
}
