#[inline]
pub fn probability(exponent: f64) -> f64 {
    (1f64 + exponent.exp()).recip()
}

pub fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
    if num == 0 {
        return vec![start];
    }

    if num == 1 {
        return vec![stop];
    }

    let step = (stop - start) / (num - 1) as f64;

    let mut result: Vec<f64> = {
        (0..(num - 1))
            .map(|i| start + (i as f64) * step)
            .collect()
    };

    result.push(stop);

    result
}

pub mod graph;
pub mod ising;

pub mod cli;
