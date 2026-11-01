//! Same step every time (lineárna funkcia): `y = start + step · x`.
//!
//! Every equal step in `x` adds (or takes away) the same amount of `y`. The
//! start is a separate number: the taxi's start fee, a candle's height before
//! it is lit. Only when the start is zero does the rule of three work; then
//! twice the `x` really is twice the `y`.

/// A straight line `y = start + step · x`. School writes it `y = k·x + q`
/// with `k = step` and `q = start`.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Line {
    /// The value at `x = 0` (school: `q`).
    pub start: f64,
    /// What one more unit of `x` adds (school: `k`, the slope).
    pub step: f64,
}

impl Line {
    pub const fn new(start: f64, step: f64) -> Self {
        Self { start, step }
    }

    pub fn at(&self, x: f64) -> f64 {
        self.start + self.step * x
    }

    /// The line through two points, or `None` when they share an `x`.
    pub fn through(a: (f64, f64), b: (f64, f64)) -> Option<Line> {
        let dx = b.0 - a.0;
        if dx.abs() < 1e-12 {
            return None;
        }
        let step = (b.1 - a.1) / dx;
        Some(Line::new(a.1 - step * a.0, step))
    }

    /// Where the line reaches zero (the candle is gone), if it ever does.
    pub fn zero_at(&self) -> Option<f64> {
        (self.step != 0.0).then(|| -self.start / self.step)
    }

    /// Where two lines meet (two offers cost the same), if they do.
    pub fn crossing(&self, other: &Line) -> Option<f64> {
        let dk = self.step - other.step;
        (dk.abs() > 1e-12).then(|| (other.start - self.start) / dk)
    }

    /// Goes through zero: the rule of three works.
    pub fn through_zero(&self) -> bool {
        self.start.abs() < 1e-12
    }
}

/// What the rule of three would say: if `x1` gives `y1`, then `x2` gives
/// `y1 · x2 / x1`. Right only for a line through zero.
pub fn rule_of_three(x1: f64, y1: f64, x2: f64) -> Option<f64> {
    (x1 != 0.0).then(|| y1 * x2 / x1)
}

/// The straight line the visitor's own tries lie on, if they all lie on one.
/// Needs at least `min` tries with different `x`; every try must be within
/// `tol` (in `y`) of the line through the two outermost ones.
pub fn line_of_tries(tries: &[(f64, f64)], min: usize, tol: f64) -> Option<Line> {
    let mut xs: Vec<f64> = tries.iter().map(|t| t.0).collect();
    xs.sort_by(f64::total_cmp);
    xs.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    if xs.len() < min.max(2) {
        return None;
    }
    let lo = tries.iter().min_by(|a, b| a.0.total_cmp(&b.0))?;
    let hi = tries.iter().max_by(|a, b| a.0.total_cmp(&b.0))?;
    let line = Line::through(*lo, *hi)?;
    tries
        .iter()
        .all(|t| (line.at(t.0) - t.1).abs() <= tol)
        .then_some(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TAXI: Line = Line::new(3.0, 1.2);

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn taxi_is_start_plus_steps() {
        assert!(close(TAXI.at(0.0), 3.0));
        assert!(close(TAXI.at(4.0), 7.8));
        assert!(close(TAXI.at(8.0), 12.6));
        // the rule of three ignores the start fee
        assert!(close(rule_of_three(4.0, 7.8, 8.0).unwrap(), 15.6));
        assert!(!TAXI.through_zero());
    }

    #[test]
    fn apples_go_through_zero_so_the_rule_of_three_works() {
        let apples = Line::new(0.0, 1.8);
        assert!(apples.through_zero());
        assert!(close(
            rule_of_three(3.0, apples.at(3.0), 5.0).unwrap(),
            apples.at(5.0)
        ));
    }

    #[test]
    fn candle_burns_down_and_runs_out() {
        let candle = Line::new(20.0, -2.0);
        assert!(close(candle.at(3.0), 14.0));
        assert!(close(candle.at(6.0), 8.0));
        assert!(close(candle.zero_at().unwrap(), 10.0));
        assert_eq!(Line::new(5.0, 0.0).zero_at(), None);
    }

    #[test]
    fn line_through_two_points() {
        let l = Line::through((4.0, 7.8), (8.0, 12.6)).unwrap();
        assert!(close(l.start, 3.0) && close(l.step, 1.2));
        assert_eq!(Line::through((2.0, 1.0), (2.0, 5.0)), None);
    }

    #[test]
    fn two_offers_cost_the_same_at_the_crossing() {
        let (a, b) = (Line::new(6.0, 1.0), Line::new(2.0, 1.5));
        let x = a.crossing(&b).unwrap();
        assert!(close(x, 8.0) && close(a.at(x), 14.0) && close(b.at(x), 14.0));
        assert_eq!(a.crossing(&Line::new(0.0, 1.0)), None);
    }

    #[test]
    fn tries_show_a_line_only_when_there_are_enough_and_they_fit() {
        let on = |x: f64| (x, TAXI.at(x));
        assert_eq!(line_of_tries(&[on(1.0), on(5.0)], 3, 0.01), None);
        assert_eq!(line_of_tries(&[on(1.0), on(1.0), on(5.0)], 3, 0.01), None);
        let l = line_of_tries(&[on(5.0), on(1.0), on(3.0)], 3, 0.01).unwrap();
        assert!(close(l.start, 3.0) && close(l.step, 1.2));
        // a burnt-out candle stays at 0: the tries no longer lie on one line
        let candle = |x: f64| (x, (20.0 - 2.0 * x).max(0.0));
        assert!(line_of_tries(&[candle(1.0), candle(4.0), candle(8.0)], 3, 0.01).is_some());
        assert_eq!(
            line_of_tries(&[candle(1.0), candle(4.0), candle(12.0)], 3, 0.01),
            None
        );
    }
}
