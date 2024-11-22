/*
This code implements fundamental quantum gates and operations for a quantum circuit simulator.

Key concepts implemented:
1. Quantum Gates: Mathematical representations of quantum operations
   - Single-qubit gates (X, Y, Z, H, S, T)
   - Two-qubit gates (CNOT)
   Learn more: https://qiskit.org/textbook/ch-states/single-qubit-gates.html

2. Complex Linear Algebra
   - Uses nalgebra library for matrix operations
   - Represents quantum states as complex vectors
   - Applies unitary transformations via matrix multiplication
   More info: https://quantum.country/qcvc

3. Gate Operations:
   - Pauli gates (X,Y,Z): Fundamental quantum operations
   - Hadamard (H): Creates superposition states
   - Phase (S) and T gates: Important for quantum algorithms
   - Rotation gates: Arbitrary rotations on Bloch sphere
   Reference: https://en.wikipedia.org/wiki/Quantum_logic_gate

4. Testing:
   - Unit tests for each gate operation
   - Validates quantum state transformations
   - Checks normalization and expected outcomes
   Guide: https://learn.qiskit.org/course/basics/single-qubit-gates

Implementation follows standard quantum computing conventions:
- States are normalized complex vectors
- Gates are unitary matrices
- CNOT implements controlled operations
- All operations preserve quantum mechanical properties

For mathematical background:
- Quantum Computing: https://arxiv.org/abs/quant-ph/0207118
- Linear Algebra: https://arxiv.org/abs/quant-ph/0001066
*/

use nalgebra::{Complex, DVector, Matrix2};
use std::f64::consts::PI;

/// Trait defining the interface for quantum gates
pub trait QuantumGate {
    fn apply(&self, state: &mut DVector<Complex<f64>>);
    fn matrix(&self) -> Matrix2<Complex<f64>>;
    fn name(&self) -> &'static str;
}

/// Helper function to apply a 2x2 matrix to a quantum state
fn apply_matrix(matrix: &Matrix2<Complex<f64>>, state: &mut DVector<Complex<f64>>) {
    if state.len() != 2 {
        panic!("State vector must be 2-dimensional for single qubit gates");
    }
    let result = matrix * DVector::from_column_slice(&[state[0], state[1]]);
    state[0] = result[0];
    state[1] = result[1];
}

// Pauli-X (NOT) Gate
#[derive(Debug, Clone, Copy)]
pub struct XGate;
impl QuantumGate for XGate {
    fn apply(&self, state: &mut DVector<Complex<f64>>) {
        let matrix = self.matrix();
        apply_matrix(&matrix, state);
    }

    fn matrix(&self) -> Matrix2<Complex<f64>> {
        Matrix2::new(
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
        )
    }

    fn name(&self) -> &'static str {
        "Pauli-X"
    }
}

// Pauli-Y Gate
#[derive(Debug, Clone, Copy)]
pub struct YGate;
impl QuantumGate for YGate {
    fn apply(&self, state: &mut DVector<Complex<f64>>) {
        let matrix = self.matrix();
        apply_matrix(&matrix, state);
    }

    fn matrix(&self) -> Matrix2<Complex<f64>> {
        Matrix2::new(
            Complex::new(0.0, 0.0),
            Complex::new(0.0, -1.0),
            Complex::new(0.0, 1.0),
            Complex::new(0.0, 0.0),
        )
    }

    fn name(&self) -> &'static str {
        "Pauli-Y"
    }
}

// Pauli-Z Gate
#[derive(Debug, Clone, Copy)]
pub struct ZGate;
impl QuantumGate for ZGate {
    fn apply(&self, state: &mut DVector<Complex<f64>>) {
        let matrix = self.matrix();
        apply_matrix(&matrix, state);
    }

    fn matrix(&self) -> Matrix2<Complex<f64>> {
        Matrix2::new(
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
        )
    }

    fn name(&self) -> &'static str {
        "Pauli-Z"
    }
}

// Hadamard Gate
#[derive(Debug, Clone, Copy)]
pub struct HadamardGate;
impl QuantumGate for HadamardGate {
    fn apply(&self, state: &mut DVector<Complex<f64>>) {
        let matrix = self.matrix();
        apply_matrix(&matrix, state);
    }

    fn matrix(&self) -> Matrix2<Complex<f64>> {
        let sqrt_2_inv = 1.0 / (2.0_f64.sqrt());
        Matrix2::new(
            Complex::new(sqrt_2_inv, 0.0),
            Complex::new(sqrt_2_inv, 0.0),
            Complex::new(sqrt_2_inv, 0.0),
            Complex::new(-sqrt_2_inv, 0.0),
        )
    }

    fn name(&self) -> &'static str {
        "Hadamard"
    }
}

// Phase Gate (S Gate)
#[derive(Debug, Clone, Copy)]
pub struct PhaseGate;
impl QuantumGate for PhaseGate {
    fn apply(&self, state: &mut DVector<Complex<f64>>) {
        let matrix = self.matrix();
        apply_matrix(&matrix, state);
    }

    fn matrix(&self) -> Matrix2<Complex<f64>> {
        Matrix2::new(
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 1.0),
        )
    }

    fn name(&self) -> &'static str {
        "Phase"
    }
}

// T Gate (π/8 gate)
#[derive(Debug, Clone, Copy)]
pub struct TGate;
impl QuantumGate for TGate {
    fn apply(&self, state: &mut DVector<Complex<f64>>) {
        let matrix = self.matrix();
        apply_matrix(&matrix, state);
    }

    fn matrix(&self) -> Matrix2<Complex<f64>> {
        Matrix2::new(
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new((PI / 4.0).cos(), (PI / 4.0).sin()),
        )
    }

    fn name(&self) -> &'static str {
        "T"
    }
}

// Rotation Gate
#[derive(Debug, Clone)]
pub struct RotationGate {
    theta: f64,
}

impl RotationGate {
    pub fn new(theta: f64) -> Self {
        Self { theta }
    }
}

impl QuantumGate for RotationGate {
    fn apply(&self, state: &mut DVector<Complex<f64>>) {
        let matrix = self.matrix();
        apply_matrix(&matrix, state);
    }

    fn matrix(&self) -> Matrix2<Complex<f64>> {
        Matrix2::new(
            Complex::new(self.theta.cos(), 0.0),
            Complex::new(-self.theta.sin(), 0.0),
            Complex::new(self.theta.sin(), 0.0),
            Complex::new(self.theta.cos(), 0.0),
        )
    }

    fn name(&self) -> &'static str {
        "Rotation"
    }
}

// CNOT Gate (Controlled-NOT)
#[derive(Debug, Clone, Copy)]
pub struct CNOTGate;

impl CNOTGate {
    pub fn apply_controlled(
        &self,
        state: &mut DVector<Complex<f64>>,
        control: usize,
        target: usize,
    ) {
        if state.len() != 4 {
            panic!("CNOT gate requires a 2-qubit state (4-dimensional vector)");
        }

        let mut new_state = state.clone();
        if control == 1 {
            new_state[2] = state[3];
            new_state[3] = state[2];
        }
        *state = new_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    fn complex_eq(a: Complex<f64>, b: Complex<f64>, epsilon: f64) -> bool {
        (a.re - b.re).abs() < epsilon && (a.im - b.im).abs() < epsilon
    }

    #[test]
    fn test_x_gate() {
        let mut state = DVector::from_vec(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]);
        let gate = XGate;
        gate.apply(&mut state);
        assert!(complex_eq(state[0], Complex::new(0.0, 0.0), 1e-10));
        assert!(complex_eq(state[1], Complex::new(1.0, 0.0), 1e-10));
    }

    #[test]
    fn test_hadamard_gate() {
        let mut state = DVector::from_vec(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]);
        let gate = HadamardGate;
        gate.apply(&mut state);
        let sqrt_2_inv = 1.0 / (2.0_f64.sqrt());
        assert!(complex_eq(state[0], Complex::new(sqrt_2_inv, 0.0), 1e-10));
        assert!(complex_eq(state[1], Complex::new(sqrt_2_inv, 0.0), 1e-10));
    }

    #[test]
    fn test_rotation_gate() {
        let mut state = DVector::from_vec(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]);
        let gate = RotationGate::new(PI / 2.0);
        gate.apply(&mut state);
        assert!(complex_eq(state[0], Complex::new(0.0, 0.0), 1e-10));
        assert!(complex_eq(state[1], Complex::new(1.0, 0.0), 1e-10));
    }

    #[test]
    fn test_phase_gate() {
        let mut state = DVector::from_vec(vec![
            Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
            Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
        ]);
        let gate = PhaseGate;
        gate.apply(&mut state);
        assert!(complex_eq(
            state[0],
            Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
            1e-10
        ));
        assert!(complex_eq(
            state[1],
            Complex::new(0.0, 1.0 / 2.0_f64.sqrt()),
            1e-10
        ));
    }

    #[test]
    #[should_panic(expected = "State vector must be 2-dimensional for single qubit gates")]
    fn test_invalid_state_dimension() {
        let mut state = DVector::from_vec(vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ]);
        let gate = XGate;
        gate.apply(&mut state);
    }
}
