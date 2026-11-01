//! Percentages and the tricks around them. Pure numbers, no UI.
//!
//! "Per cent" means "per hundred": `p %` of `x` is `x · p / 100`. Everything
//! here is that one line, applied in ways that surprise people: swapping the two
//! numbers, going up and down by the same percent, stacking discounts, taking
//! VAT back out, and percent versus percentage points.

/// `p %` of `x`.
pub fn of(p: f64, x: f64) -> f64 {
    x * p / 100.0
}

/// Change `x` by `p %` (negative `p` goes down).
pub fn change(x: f64, p: f64) -> f64 {
    x * (1.0 + p / 100.0)
}

/// Apply several percent changes one after the other; returns every step,
/// starting with `x`.
pub fn chain(x: f64, steps: &[f64]) -> Vec<f64> {
    let mut out = Vec::with_capacity(steps.len() + 1);
    let mut v = x;
    out.push(v);
    for &p in steps {
        v = change(v, p);
        out.push(v);
    }
    out
}

/// The single percent change that does the same as `steps` together.
/// +20 % then −20 % is −4 %; −30 % then −20 % is −44 %.
pub fn combined(steps: &[f64]) -> f64 {
    let factor: f64 = steps.iter().map(|p| 1.0 + p / 100.0).product();
    (factor - 1.0) * 100.0
}

/// The change needed to get back to where you started after `p %`.
/// After −20 % you need +25 %; after +25 % you need −20 %.
pub fn undo(p: f64) -> Option<f64> {
    let f = 1.0 + p / 100.0;
    (f > 0.0).then(|| (1.0 / f - 1.0) * 100.0)
}

/// A price with VAT at `rate %` → the price without VAT.
pub fn without_vat(gross: f64, rate: f64) -> f64 {
    gross / (1.0 + rate / 100.0)
}

/// Going from `from %` to `to %`: the change in percentage points and in percent
/// of the old value. 2 % → 3 % is +1 point, but +50 %.
pub fn points_and_percent(from: f64, to: f64) -> (f64, Option<f64>) {
    let points = to - from;
    let rel = (from != 0.0).then(|| (to - from) / from * 100.0);
    (points, rel)
}

/// The quick ways to get a percent of a number in your head. Each is a known
/// fraction of the number, or a sum of those.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quick {
    /// 50 %: half.
    Half,
    /// 25 %: half of half.
    Quarter,
    /// 10 %: move the decimal point one place left.
    Tenth,
    /// 5 %: half of 10 %.
    HalfTenth,
    /// 1 %: move the decimal point two places left.
    Hundredth,
    /// 20 %: 10 % twice.
    TwoTenths,
    /// 15 %: 10 % plus 5 %.
    TenPlusFive,
}

impl Quick {
    pub const ALL: [Quick; 7] = [
        Quick::Half,
        Quick::Quarter,
        Quick::Tenth,
        Quick::HalfTenth,
        Quick::Hundredth,
        Quick::TwoTenths,
        Quick::TenPlusFive,
    ];

    pub fn percent(self) -> f64 {
        match self {
            Quick::Half => 50.0,
            Quick::Quarter => 25.0,
            Quick::Tenth => 10.0,
            Quick::HalfTenth => 5.0,
            Quick::Hundredth => 1.0,
            Quick::TwoTenths => 20.0,
            Quick::TenPlusFive => 15.0,
        }
    }

    /// The value the way you work it out in your head (same number as [`of`]).
    pub fn by_hand(self, x: f64) -> f64 {
        let tenth = x / 10.0;
        match self {
            Quick::Half => x / 2.0,
            Quick::Quarter => x / 2.0 / 2.0,
            Quick::Tenth => tenth,
            Quick::HalfTenth => tenth / 2.0,
            Quick::Hundredth => x / 100.0,
            Quick::TwoTenths => tenth * 2.0,
            Quick::TenPlusFive => tenth + tenth / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{a} != {b}");
    }

    #[test]
    fn swap_trick() {
        // 8 % of 50 = 50 % of 8 = 4; 4 % of 75 = 75 % of 4 = 3
        near(of(8.0, 50.0), 4.0);
        near(of(50.0, 8.0), 4.0);
        near(of(4.0, 75.0), of(75.0, 4.0));
        near(of(4.0, 75.0), 3.0);
    }

    #[test]
    fn up_and_down_is_not_back() {
        let c = chain(100.0, &[20.0, -20.0]);
        near(c[1], 120.0);
        near(c[2], 96.0);
        near(combined(&[20.0, -20.0]), -4.0);
        // the order does not matter
        near(combined(&[-20.0, 20.0]), -4.0);
        near(undo(-20.0).unwrap(), 25.0);
        near(undo(25.0).unwrap(), -20.0);
        assert_eq!(undo(-100.0), None);
    }

    #[test]
    fn discounts_do_not_add() {
        near(combined(&[-30.0, -20.0]), -44.0);
        near(chain(100.0, &[-30.0, -20.0])[2], 56.0);
    }

    #[test]
    fn vat_comes_out_by_dividing() {
        near(without_vat(123.0, 23.0), 100.0);
        // the wrong way: minus 23 % of the gross price
        near(change(123.0, -23.0), 94.71);
    }

    #[test]
    fn points_versus_percent() {
        let (pts, rel) = points_and_percent(2.0, 3.0);
        near(pts, 1.0);
        near(rel.unwrap(), 50.0);
        // risk: 1 in 10 000 → 2 in 10 000 is "+100 %", but 0.01 points
        let (pts, rel) = points_and_percent(0.01, 0.02);
        near(pts, 0.01);
        near(rel.unwrap(), 100.0);
        assert_eq!(points_and_percent(0.0, 1.0).1, None);
    }

    #[test]
    fn quick_ways_agree_with_the_definition() {
        for x in [240.0, 37.5, 1999.0] {
            for q in Quick::ALL {
                near(q.by_hand(x), of(q.percent(), x));
            }
        }
    }
}
