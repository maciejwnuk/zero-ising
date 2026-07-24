use clap::ValueEnum;
use rand::prelude::*;

use crate::graph::Graph;

#[derive(Debug, Clone, ValueEnum)]
pub enum States {
    Normal,
    Extended
}

#[derive(Debug, Clone)]
pub struct Fields(pub f64, pub f64);

#[derive(Debug)]
pub struct Ising {
    spins: Vec<i8>,

    graph: Graph,
    fields: Fields,
    states: States
}

impl Ising {
    pub fn new(graph: Graph, states: States, fields: Fields) -> Self {
        let size = graph.size();

        let mut rng = rand::rng();

        let spins: Vec<i8> = match states {
            States::Normal => {
                (0..size).map(|_| *[-1, 1].choose(&mut rng).unwrap()).collect()
            },
            States::Extended => {
                (0..size).map(|_| *[-1, 0, 1].choose(&mut rng).unwrap()).collect()
            },
        };

        Ising { spins, graph, fields, states }
    }

    pub fn step(&mut self, temperature: f64) {
        let mut rng = rand::rng();

        let size = self.graph.size();

        let mut indices: Vec<usize> = (0..size).collect();
        indices.shuffle(&mut rng);

        let beta = if temperature == 0.0 {
            f64::MAX
        } else {
            temperature.recip()
        };

        // Modified Glauber algorithm
        for i in indices {
            let old_spin = self.spins[i];

            let new_spin = match self.states {
                States::Normal => {
                    old_spin * -1
                },
                States::Extended => {
                    [-1, 0, 1].into_iter()
                        .filter(|x| *x != old_spin)
                        .choose(&mut rng)
                        .unwrap()
                },
            };

            let spin_change = new_spin - old_spin;

            // sum_(j in Nbrs(i)) s^j
            let spins_nbrs = {
                self.graph.adj()[i]
                    .iter()
                    .zip(&self.spins)
                    .map(|(a, b)| *a as i8 * b)
                    .map(f64::from)
                    .sum::<f64>()
            };

            let partial_energy_change = -self.fields.0 * spins_nbrs - self.fields.1;
            let energy_change = partial_energy_change * spin_change as f64;

            let exponent = beta * energy_change;

            let probability = (1. + exponent.exp()).recip();

            if probability > rng.random::<f64>() {
                self.spins[i] = new_spin;
            }
        }
    }

    pub fn state(&self) -> &[i8] {
        &self.spins
    }

    pub fn magnetization(&self) -> isize {
        self.spins.iter()
            .copied()
            .map(isize::from)
            .sum::<isize>()
    }

    pub fn energy(&self) -> f64 {
        let spin_product = self.spins.iter()
            .zip(self.graph.adj().iter())
            .map(|(s_i, nbrs)| {
                *s_i as f64 * nbrs.iter()
                    .zip(self.spins.iter())
                    .map(|(m_ij, s_j)| *m_ij as i8 * s_j)
                    .map(f64::from)
                    .sum::<f64>()
            })
            .sum::<f64>();

        let spin_sum = self.spins.iter()
            .copied()
            .map(f64::from)
            .sum::<f64>();

        - self.fields.0 * spin_product / 2. - self.fields.1 * spin_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{Graph, Topology};

    #[test]
    fn test_new_normal() {
        let graph = Graph::new(10, Topology::Chain);
        let ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

        assert_eq!(ising.state().len(), 10);

        for &spin in ising.state() {
            assert!(spin == -1 || spin == 1);
        }
    }

    #[test]
    fn test_new_extended() {
        let mut has_minus_one = false;
        let mut has_zero = false;
        let mut has_one = false;

        for _ in 0..100 {
            let graph = Graph::new(10, Topology::Chain);
            let ising = Ising::new(graph, States::Extended, Fields(1.0, 0.0));

            assert_eq!(ising.state().len(), 10);

            for &spin in ising.state() {
                match spin {
                    -1 => has_minus_one = true,
                    0 => has_zero = true,
                    1 => has_one = true,
                    _ => panic!("Invalid spin value"),
                }
            }

            if has_minus_one && has_zero && has_one {
                break;
            }
        }

        assert!(has_minus_one && has_zero && has_one);
    }

    #[test]
    fn test_magnetization() {
        let graph = Graph::new(4, Topology::Chain);
        let ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

        let mag = ising.magnetization();

        let expected_mag: isize = ising.state()
            .iter()
            .copied()
            .map(isize::from)
            .sum();

        assert_eq!(mag, expected_mag);

        assert!(mag >= -4 && mag <= 4);
    }

    #[test]
    fn test_energy() {
        let graph = Graph::new(2, Topology::Chain);
        let ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

        let energy = ising.energy();

        assert!(energy.is_finite());

        let adj = ising.graph.adj();
        let spins = ising.state();

        let mut expected_energy = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                expected_energy += spins[i] as f64 * adj[i][j] as f64 * spins[j] as f64;
            }
        }

        expected_energy = -1.0 * expected_energy / 2.0;

        // Since spins are valid values, expected_energy must match
        assert_eq!(energy, expected_energy);
    }

    #[test]
    fn test_step_ising_normal_t0() {
        let graph = Graph::new(10, Topology::Chain);
        let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

        let energy_before = ising.energy();

        ising.step(0.0);

        let energy_after = ising.energy();

        assert!(energy_after <= energy_before);
    }

    #[test]
    fn test_step_ising_extended_t0() {
        let graph = Graph::new(10, Topology::Chain);
        let mut ising = Ising::new(graph, States::Extended, Fields(1.0, 0.0));
        ising.step(0.0);
        for &spin in ising.state() {
            assert!(spin == -1 || spin == 0 || spin == 1);
        }
    }

    #[test]
    fn test_step_t_low() {
        let mut avg_mag = 0.0;

        let iters = 10;

        for _ in 0..iters {
            let graph = Graph::new(10, Topology::Chain);
            let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

            for _ in 0..100 {
                ising.step(0.01);
            }

            avg_mag += ising.magnetization().abs() as f64;
        }

        avg_mag /= iters as f64;

        // At low T, J > 0, |magnetization| tends to be large (ordered)
        assert!(avg_mag > 5.0);
    }

    #[test]
    fn test_step_t_high() {
        let mut avg_mag = 0.0;

        let iters = 10;

        for _ in 0..iters {
            let graph = Graph::new(10, Topology::Chain);
            let mut ising = Ising::new(graph, States::Normal, Fields(1.0, 0.0));

            for _ in 0..100 {
                ising.step(100.0);
            }

            avg_mag += ising.magnetization().abs() as f64;
        }

        avg_mag /= iters as f64;

        // At very high T, magnetization should average out around 0
        assert!(avg_mag < 5.0);
    }
}
