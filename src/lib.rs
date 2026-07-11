const KB: f64 = 1.380649e-23;

#[inline]
pub fn probability(exponent: f64) -> f64 {
    (1f64 + exponent.exp()).recip()
}

pub mod graph;
pub mod ising;

pub mod cli;
