//! Chance: sums of dice and the central limit theorem.

use rand::{Rng, SeedableRng, rngs::SmallRng};

/// Roll `dice` fair dice with `sides` faces, many times, and count the sums.
///
/// Deterministic for a given seed — useful for tests and for "show me that again".
pub struct DiceSums {
    dice: u32,
    sides: u32,
    /// `counts[k]` = how many rolls summed to `min_sum() + k`.
    counts: Vec<u64>,
    rolls: u64,
    rng: SmallRng,
}

impl DiceSums {
    pub fn new(dice: u32, sides: u32, seed: u64) -> Self {
        let dice = dice.max(1);
        let sides = sides.max(2);
        let bins = (dice * (sides - 1) + 1) as usize;
        Self {
            dice,
            sides,
            counts: vec![0; bins],
            rolls: 0,
            rng: SmallRng::seed_from_u64(seed),
        }
    }

    pub fn dice(&self) -> u32 {
        self.dice
    }
    pub fn sides(&self) -> u32 {
        self.sides
    }
    pub fn rolls(&self) -> u64 {
        self.rolls
    }
    pub fn counts(&self) -> &[u64] {
        &self.counts
    }
    pub fn min_sum(&self) -> u32 {
        self.dice
    }
    pub fn max_sum(&self) -> u32 {
        self.dice * self.sides
    }

    pub fn roll(&mut self, times: u64) {
        for _ in 0..times {
            let sum: u32 = (0..self.dice)
                .map(|_| self.rng.random_range(1..=self.sides))
                .sum();
            self.counts[(sum - self.dice) as usize] += 1;
        }
        self.rolls += times;
    }

    /// Theoretical mean of the sum: n·(s+1)/2.
    pub fn mean(&self) -> f64 {
        self.dice as f64 * (self.sides as f64 + 1.0) / 2.0
    }

    /// Theoretical standard deviation of the sum: √(n·(s²−1)/12).
    pub fn std_dev(&self) -> f64 {
        let s = self.sides as f64;
        (self.dice as f64 * (s * s - 1.0) / 12.0).sqrt()
    }

    /// Mean of what was actually rolled.
    pub fn observed_mean(&self) -> Option<f64> {
        (self.rolls > 0).then(|| {
            let total: f64 = self
                .counts
                .iter()
                .enumerate()
                .map(|(k, &c)| (k as f64 + self.dice as f64) * c as f64)
                .sum();
            total / self.rolls as f64
        })
    }
}

/// Normal (Gauss) probability density.
pub fn normal_pdf(x: f64, mean: f64, std_dev: f64) -> f64 {
    let z = (x - mean) / std_dev;
    (-0.5 * z * z).exp() / (std_dev * (2.0 * std::f64::consts::PI).sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_d6_basics() {
        let d = DiceSums::new(2, 6, 1);
        assert_eq!(d.counts().len(), 11);
        assert_eq!((d.min_sum(), d.max_sum()), (2, 12));
        assert!((d.mean() - 7.0).abs() < 1e-12);
        assert!((d.std_dev() - (35.0f64 / 6.0).sqrt()).abs() < 1e-12);
    }

    #[test]
    fn many_rolls_land_near_theory() {
        let mut d = DiceSums::new(5, 6, 42);
        d.roll(200_000);
        assert_eq!(d.counts().iter().sum::<u64>(), 200_000);
        let m = d.observed_mean().unwrap();
        assert!((m - d.mean()).abs() < 0.05, "observed mean {m}");
    }

    #[test]
    fn same_seed_same_story() {
        let mut a = DiceSums::new(3, 6, 7);
        let mut b = DiceSums::new(3, 6, 7);
        a.roll(1000);
        b.roll(1000);
        assert_eq!(a.counts(), b.counts());
    }

    #[test]
    fn pdf_peaks_at_mean() {
        assert!(normal_pdf(0.0, 0.0, 1.0) > normal_pdf(0.5, 0.0, 1.0));
        assert!((normal_pdf(0.0, 0.0, 1.0) - 0.398_942_28).abs() < 1e-6);
    }
}
