use clap::ValueEnum;
use rand::{
    RngExt,
    seq::{
        IndexedRandom,
        IteratorRandom,
        SliceRandom
    }
};

use crate::{
    KB,
    graph::Graph
};

#[derive(Debug, Clone, ValueEnum)]
pub enum States {
    Normal,
    Extended
}

#[derive(Debug, Clone)]
pub struct Fields(f64, f64);

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
                (0..size).map(|_| *[-1i8, 1i8].choose(&mut rng).unwrap()).collect()
            },
            States::Extended => {
                (0..size).map(|_| *[-1i8, 0i8, 1i8].choose(&mut rng).unwrap()).collect()
            },
        };

        Ising { spins, graph, fields, states }
    }

    pub fn step(&mut self, temperature: f64) {
        let mut rng = rand::rng();

        let size = self.graph.size();

        let mut indices: Vec<usize> = (0..size).collect();
        indices.shuffle(&mut rng);

        let beta = (KB * temperature).recip();

        // Modified Glauber algorithm
        for i in indices {
            let old_spin = self.spins[i];

            let new_spin = match self.states {
                States::Normal => {
                    old_spin * -1
                },
                States::Extended => {
                    [-1i8, 0i8, 1i8].into_iter()
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

            let probability = (1f64 + exponent.exp()).recip();

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

        -self.fields.0 * spin_product - self.fields.1
    }
}
