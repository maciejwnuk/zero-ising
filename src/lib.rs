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

#[cfg(test)]
mod tests {
    use super::*;

    use rand::prelude::*;

    #[test]
    fn test_probability_zero() {
        assert_eq!(probability(0.0), 0.5);
    }

    #[test]
    fn test_probability_large_positive() {
        assert!(probability(100.0) < 1e-10);
    }

    #[test]
    fn test_probability_large_negative() {
        assert!((probability(-100.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_probability_range() {
        for _ in 0..100 {
            let exp: i64 = rand::random();

            let p = probability(exp as f64);

            assert!(p >= 0.0 && p <= 1.0);
        }
    }

    #[test]
    fn test_linspace_zero() {
        assert_eq!(linspace(0.0, 10.0, 0), vec![0.0]);
    }

    #[test]
    fn test_linspace_one() {
        assert_eq!(linspace(0.0, 10.0, 1), vec![10.0]);
    }

    #[test]
    fn test_linspace_two() {
        assert_eq!(linspace(0.0, 10.0, 2), vec![0.0, 10.0]);
    }

    #[test]
    fn test_linspace_five() {
        assert_eq!(linspace(0.0, 1.0, 5), vec![0.0, 0.25, 0.5, 0.75, 1.0]);
    }

    #[test]
    fn test_linspace_eleven() {
        let result = linspace(0.0, 1.0, 11);

        assert_eq!(result[0], 0.0);
        assert_eq!(result[10], 1.0);
    }

    #[test]
    fn test_linspace_descending() {
        assert_eq!(linspace(10.0, 0.0, 3), vec![10.0, 5.0, 0.0]);
    }

    #[test]
    fn test_linspace_equal_start_stop() {
        assert_eq!(linspace(5.0, 5.0, 4), vec![5.0, 5.0, 5.0, 5.0]);
    }
}
