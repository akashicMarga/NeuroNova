/*
This file implements a quantum circuit simulator that models quantum computation operations.
The code simulates quantum states and gates following the principles of quantum mechanics.

Key concepts and implementations:

1. Quantum States:
   - Represents quantum states using complex-valued state vectors
   - Uses the bra-ket notation (|0⟩, |1⟩) from quantum mechanics
   - Learn more: https://en.wikipedia.org/wiki/Quantum_state

2. Quantum Gates:
   - Implements basic quantum gates (X, Hadamard, etc.)
   - Gates are represented as unitary matrices
   - Learn more: https://en.wikipedia.org/wiki/Quantum_logic_gate

3. Measurement:
   - Implements quantum measurement following Born's rule
   - Causes state collapse according to quantum mechanics principles
   - Learn more: https://en.wikipedia.org/wiki/Measurement_in_quantum_mechanics

4. Circuit Operations:
   - apply_gate(): Applies single-qubit gates
   - apply_controlled_gate(): Implements controlled operations like CNOT
   - measure(): Performs quantum measurements
   - verify_state(): Ensures quantum state normalization

5. State Management:
   - Handles state initialization
   - Maintains state vector normalization
   - Implements state reset functionality

Further Reading:
- Quantum Computing Basics: https://quantum.country/qcvc
- Circuit Model: https://en.wikipedia.org/wiki/Quantum_circuit

The implementation uses:
- nalgebra for linear algebra operations
- Complex numbers for quantum amplitudes
- Random number generation for measurement outcomes
*/

use crate::gates::QuantumGate;
use nalgebra::{Complex, DVector};
use rand::Rng;
use std::f64;

#[derive(Debug)]
pub struct QuantumCircuit {
    state: DVector<Complex<f64>>,
    n_qubits: usize,
}

impl QuantumCircuit {
    /// Creates a new quantum circuit with the specified number of qubits
    /// All qubits are initialized to |0⟩ state
    pub fn new(n_qubits: usize) -> Self {
        if n_qubits == 0 {
            panic!("Number of qubits must be greater than 0");
        }

        // Create a zero vector of size 2^n_qubits
        let mut state = DVector::from_element(1 << n_qubits, Complex::new(0.0, 0.0));
        // Initialize to |00...0⟩ state
        state[0] = Complex::new(1.0, 0.0);

        QuantumCircuit { state, n_qubits }
    }

    /// Applies a quantum gate to the specified target qubit
    pub fn apply_gate<G: QuantumGate>(&mut self, gate: G, target: usize) -> Result<(), String> {
        if target >= self.n_qubits {
            return Err(format!(
                "Target qubit {} is out of range for circuit with {} qubits",
                target, self.n_qubits
            ));
        }

        let n = self.state.len();
        let mut new_state = DVector::from_element(n, Complex::new(0.0, 0.0));

        for i in 0..n {
            if (i & (1 << target)) != 0 {
                continue;
            }

            let i1 = i | (1 << target);

            // Create the 2D state vector for the target qubit
            let mut target_state = DVector::from_vec(vec![self.state[i], self.state[i1]]);

            // Apply the gate
            gate.apply(&mut target_state);

            // Update the new state
            new_state[i] = target_state[0];
            new_state[i1] = target_state[1];
        }

        self.state = new_state;
        Ok(())
    }

    /// Applies a controlled gate with one control qubit and one target qubit
    pub fn apply_controlled_gate<G: QuantumGate>(
        &mut self,
        gate: G,
        control: usize,
        target: usize,
    ) -> Result<(), String> {
        if control >= self.n_qubits || target >= self.n_qubits {
            return Err(format!(
                "Qubit indices out of range for circuit with {} qubits",
                self.n_qubits
            ));
        }
        if control == target {
            return Err("Control and target qubits must be different".to_string());
        }

        let n = self.state.len();
        let mut new_state = self.state.clone();

        for i in 0..n {
            if (i & (1 << control)) != 0 {
                let mut target_state = DVector::from_vec(vec![
                    self.state[i & !(1 << target)],
                    self.state[i | (1 << target)],
                ]);

                gate.apply(&mut target_state);

                new_state[i & !(1 << target)] = target_state[0];
                new_state[i | (1 << target)] = target_state[1];
            }
        }

        self.state = new_state;
        Ok(())
    }

    /// Measures the specified qubit and returns the result (0 or 1)
    pub fn measure(&mut self, target: usize) -> Result<bool, String> {
        if target >= self.n_qubits {
            return Err(format!(
                "Target qubit {} is out of range for circuit with {} qubits",
                target, self.n_qubits
            ));
        }

        let n = self.state.len();
        let mut prob_one = 0.0;

        // Calculate probability of measuring |1⟩
        for i in 0..n {
            if (i & (1 << target)) != 0 {
                prob_one += self.state[i].norm_sqr();
            }
        }

        // Generate random number and collapse the state
        let mut rng = rand::thread_rng();
        let random: f64 = rng.gen();
        let result = random < prob_one;

        // Collapse the state vector
        let mut new_state = DVector::from_element(n, Complex::new(0.0, 0.0));
        let norm = if result {
            prob_one.sqrt()
        } else {
            (1.0 - prob_one).sqrt()
        };

        for i in 0..n {
            if (i & (1 << target) != 0) == result {
                new_state[i] = self.state[i] / Complex::new(norm, 0.0);
            }
        }

        self.state = new_state;
        Ok(result)
    }

    /// Returns the current state vector
    pub fn get_state(&self) -> &DVector<Complex<f64>> {
        &self.state
    }

    /// Verifies that the state vector is normalized
    pub fn verify_state(&self) -> bool {
        let sum: f64 = self.state.iter().map(|x| x.norm_sqr()).sum();
        (sum - 1.0).abs() < 1e-10
    }

    /// Returns the number of qubits in the circuit
    pub fn n_qubits(&self) -> usize {
        self.n_qubits
    }

    /// Resets the circuit to the initial state |00...0⟩
    pub fn reset(&mut self) {
        self.state.fill(Complex::new(0.0, 0.0));
        self.state[0] = Complex::new(1.0, 0.0);
    }

    /// Returns the probability of measuring a specific basis state
    pub fn get_probability(&self, basis_state: usize) -> Result<f64, String> {
        if basis_state >= self.state.len() {
            return Err(format!("Basis state {} is out of range", basis_state));
        }
        Ok(self.state[basis_state].norm_sqr())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gates::{HadamardGate, XGate};
    use approx::assert_relative_eq;

    #[test]
    fn test_new_circuit() {
        let circuit = QuantumCircuit::new(2);
        assert_eq!(circuit.state.len(), 4);
        assert_eq!(circuit.state[0], Complex::new(1.0, 0.0));
        assert!(circuit.verify_state());
    }

    #[test]
    fn test_apply_x_gate() {
        let mut circuit = QuantumCircuit::new(1);
        circuit.apply_gate(XGate, 0).unwrap();
        assert_eq!(circuit.state[0], Complex::new(0.0, 0.0));
        assert_eq!(circuit.state[1], Complex::new(1.0, 0.0));
    }

    #[test]
    fn test_apply_hadamard() {
        let mut circuit = QuantumCircuit::new(1);
        circuit.apply_gate(HadamardGate, 0).unwrap();
        let sqrt_2_inv = 1.0 / (2.0_f64.sqrt());
        assert_relative_eq!(circuit.state[0].re, sqrt_2_inv, epsilon = 1e-10);
        assert_relative_eq!(circuit.state[1].re, sqrt_2_inv, epsilon = 1e-10);
    }

    #[test]
    fn test_measurement() {
        let mut circuit = QuantumCircuit::new(1);
        circuit.apply_gate(HadamardGate, 0).unwrap();
        let result = circuit.measure(0).unwrap();
        assert!(circuit.verify_state());
        assert!(result == true || result == false);
    }

    #[test]
    #[should_panic]
    fn test_invalid_qubit_count() {
        QuantumCircuit::new(0);
    }

    #[test]
    fn test_reset() {
        let mut circuit = QuantumCircuit::new(1);
        circuit.apply_gate(XGate, 0).unwrap();
        circuit.reset();
        assert_eq!(circuit.state[0], Complex::new(1.0, 0.0));
        assert_eq!(circuit.state[1], Complex::new(0.0, 0.0));
    }
}
