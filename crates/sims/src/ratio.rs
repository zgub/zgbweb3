//! Rule of three (trojčlenka): "if `a` is `b`, what is `c`?"
//!
//! Direct proportion: twice the one means twice the other, `b / a` stays the
//! same, so the answer is `b · c / a`. Inverse proportion: twice the one means
//! half the other, `a · b` stays the same, so the answer is `a · b / c`.
//!
//! The pictures on the page go "through one": from the known pair to a small
//! step that fits both numbers, then out to the asked one. [`Steps`] finds
//! that step (the greatest common divisor), so 30 % → 100 % goes ÷ 3 to 10 %,
//! then × 10.

/// Twice the one means twice the other, or half of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Direct,
    Inverse,
}

/// "If `a` is `b`, what is `c`?"
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Question {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub kind: Kind,
}

impl Question {
    pub const fn direct(a: f64, b: f64, c: f64) -> Self {
        Self {
            a,
            b,
            c,
            kind: Kind::Direct,
        }
    }

    pub const fn inverse(a: f64, b: f64, c: f64) -> Self {
        Self {
            a,
            b,
            c,
            kind: Kind::Inverse,
        }
    }

    /// The answer, or `None` when it has none (dividing by zero, negative amounts).
    pub fn answer(&self) -> Option<f64> {
        if !(self.a > 0.0 && self.c >= 0.0 && self.b >= 0.0) {
            return None;
        }
        match self.kind {
            Kind::Direct => Some(self.b * self.c / self.a),
            Kind::Inverse if self.c > 0.0 => Some(self.a * self.b / self.c),
            Kind::Inverse => None,
        }
    }

    /// For direct proportion: how much of the second thing one of the first is
    /// (6 per percent when 30 % is 180). School calls it the constant `k` in `y = k·x`.
    pub fn per_one(&self) -> Option<f64> {
        (self.kind == Kind::Direct && self.a > 0.0).then(|| self.b / self.a)
    }

    /// The "through one" route, when `a` and `c` share a step (see [`Steps`]).
    pub fn steps(&self) -> Option<Steps> {
        Steps::between(self.a, self.c)
    }
}

/// The route from `a` to `c` through a shared step: `a ÷ down = step`, then
/// `step × up = c`. Both factors are whole numbers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Steps {
    pub step: f64,
    pub down: u64,
    pub up: u64,
}

impl Steps {
    /// Numbers are taken to two decimals (1.5 l, 6.5 l per 100 km). `None` when
    /// a number is not positive, or the route would need more than
    /// [`Steps::MAX_FACTOR`] pieces, which no picture can show.
    pub fn between(a: f64, c: f64) -> Option<Steps> {
        const SCALE: f64 = 100.0;
        let ia = to_hundredths(a)?;
        let ic = to_hundredths(c)?;
        let g = gcd(ia, ic);
        let (down, up) = (ia / g, ic / g);
        (down <= Self::MAX_FACTOR && up <= Self::MAX_FACTOR).then(|| Steps {
            step: g as f64 / SCALE,
            down,
            up,
        })
    }

    pub const MAX_FACTOR: u64 = 100;
}

fn to_hundredths(x: f64) -> Option<u64> {
    let v = (x * 100.0).round();
    (x > 0.0 && (1.0..1e12).contains(&v) && (x * 100.0 - v).abs() < 1e-6).then_some(v as u64)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// How close a guess is, for the "guess first" box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Within 0.5 % (or exactly right when the answer is 0).
    Exact,
    /// Within 10 %.
    Close,
    TooLow,
    TooHigh,
}

pub fn judge(guess: f64, answer: f64) -> Verdict {
    let tol = answer.abs() * 0.005;
    if (guess - answer).abs() <= tol.max(1e-9) {
        Verdict::Exact
    } else if (guess - answer).abs() <= answer.abs() * 0.10 {
        Verdict::Close
    } else if guess < answer {
        Verdict::TooLow
    } else {
        Verdict::TooHigh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(x: Option<f64>, want: f64) {
        let x = x.expect("an answer");
        assert!((x - want).abs() < 1e-9, "{x} != {want}");
    }

    #[test]
    fn percent_example_from_the_page() {
        // If 180 is 30 %, what is 100 %?
        let q = Question::direct(30.0, 180.0, 100.0);
        near(q.answer(), 600.0);
        near(q.per_one(), 6.0);
        let s = q.steps().unwrap();
        assert_eq!((s.step, s.down, s.up), (10.0, 3, 10));
    }

    #[test]
    fn examples_on_the_page() {
        // pancakes: 4 people 250 ml → 6 people
        let q = Question::direct(4.0, 250.0, 6.0);
        near(q.answer(), 375.0);
        let s = q.steps().unwrap();
        assert_eq!((s.step, s.down, s.up), (2.0, 2, 3));
        // fuel: 100 km 6.5 l → 340 km
        let q = Question::direct(100.0, 6.5, 340.0);
        near(q.answer(), 22.1);
        let s = q.steps().unwrap();
        assert_eq!((s.step, s.down, s.up), (20.0, 5, 17));
        // sale: you pay 70 % = 42 € → 100 %
        near(Question::direct(70.0, 42.0, 100.0).answer(), 60.0);
        // painters: 4 painters 6 days → 3 painters
        near(Question::inverse(4.0, 6.0, 3.0).answer(), 8.0);
    }

    #[test]
    fn no_answer_without_a_starting_amount() {
        assert_eq!(Question::direct(0.0, 180.0, 100.0).answer(), None);
        assert_eq!(Question::inverse(4.0, 6.0, 0.0).answer(), None);
        assert_eq!(Question::direct(-3.0, 1.0, 1.0).answer(), None);
    }

    #[test]
    fn steps_handle_decimals_and_refuse_huge_routes() {
        let s = Steps::between(1.5, 4.5).unwrap();
        assert_eq!((s.step, s.down, s.up), (1.5, 1, 3));
        // 37 and 101 share nothing but 0.01: far too many pieces to draw.
        assert!(Steps::between(37.0, 101.0).is_none());
        assert!(Steps::between(0.0, 5.0).is_none());
    }

    #[test]
    fn verdicts() {
        assert_eq!(judge(600.0, 600.0), Verdict::Exact);
        assert_eq!(judge(601.0, 600.0), Verdict::Exact);
        assert_eq!(judge(560.0, 600.0), Verdict::Close);
        assert_eq!(judge(234.0, 600.0), Verdict::TooLow);
        assert_eq!(judge(900.0, 600.0), Verdict::TooHigh);
    }
}
