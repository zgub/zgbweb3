//! The timeline of everything (`timeline`): from the first moment physics can
//! talk about to the heat death of the universe, on one zoomable axis.
//!
//! # The axis
//!
//! Plain linear time hides everything: on a line from the Big Bang to 10¹⁰⁰
//! years, all of the past is one pixel. Plain log of "time since the Big Bang"
//! gives the first second plenty of room but squeezes all of human history into
//! the last pixel before now. So the axis coordinate `u` is built in two parts:
//!
//! - **Past** (Big Bang → now): `u = log₁₀ t − log₁₀ (ago + ε)`, where `t` is the
//!   time since the Big Bang and `ago = T − t` the time before now (`T` = age of
//!   the universe). Near the Big Bang it behaves like `log t`, so the first
//!   fractions of a second spread out; near now it behaves like `−log ago`, so
//!   the last thousand years spread out too. `ε` (one year) keeps "now" at a
//!   finite place instead of infinitely far to the right.
//! - **Future** (now → heat death): `u = u_now + log₁₀ (1 + ahead / ε)`.
//!   Both halves have the same slope at now, so the join is smooth.
//!
//! The page draws `u` linearly; the mouse wheel zooms in `u`. Zoomed far in,
//! `u` is locally almost linear in time, so a zoomed view reads like an
//! ordinary timeline.
//!
//! # Precision
//!
//! Dates are kept in the form they are known in ([`When`]): "380 000 years
//! after the Big Bang", "4.54 billion years ago", "in 5 billion years". The age
//! of the universe itself is uncertain by tens of millions of years; that error
//! must not leak into "when did Rome fall", and an `f64` holding 13.8 × 10⁹
//! could not resolve a day anyway.

/// Age of the universe in years: 13.787 ± 0.020 × 10⁹ (Planck 2018, ΛCDM).
pub const AGE_YR: f64 = 13.787e9;
/// Seconds in a Julian year (365.25 days), the astronomers' year.
pub const SEC_PER_YR: f64 = 365.25 * 86_400.0;
/// The Planck time, 5.39 × 10⁻⁴⁴ s: the earliest moment current physics can
/// say anything about. The axis starts here, not at zero.
pub const PLANCK_S: f64 = 5.39e-44;
/// Softening at "now" (years): how close to now the past half stops stretching.
pub const EPS_YR: f64 = 1.0;
/// Right end of the axis, in years from now. The largest black holes are gone
/// by about 10¹⁰⁰ years; beyond that nothing much happens, ever.
pub const END_AHEAD_YR: f64 = 1e110;

/// Where the axis stops counting "after the Big Bang" and starts counting
/// "ago": one billion years after the start. Earlier moments are known that
/// way (first stars, first galaxies); later ones by their age.
pub const SWITCH_AFTER_YR: f64 = 1e9;

const LN10: f64 = std::f64::consts::LN_10;

/// A moment, in the form it is known in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum When {
    /// Years after the Big Bang (early-universe events).
    After(f64),
    /// Years before now (Earth, life, history).
    Ago(f64),
    /// Years from now (the future).
    Ahead(f64),
}

impl When {
    /// `After`, given in seconds.
    pub fn after_s(s: f64) -> When {
        When::After(s / SEC_PER_YR)
    }

    /// Position on the axis.
    pub fn u(self) -> f64 {
        match self {
            When::After(t) if t >= AGE_YR => When::Ahead(t - AGE_YR).u(),
            When::After(t) => past_u(t, AGE_YR - t),
            When::Ago(a) if a < 0.0 => When::Ahead(-a).u(),
            When::Ago(a) => {
                let a = a.min(AGE_YR);
                past_u(AGE_YR - a, a)
            }
            When::Ahead(y) if y < 0.0 => When::Ago(-y).u(),
            When::Ahead(y) => u_now() + (y / EPS_YR).ln_1p() / LN10,
        }
    }

    /// The moment at axis position `u`, in the natural form for that part of the
    /// axis: `After` up to [`SWITCH_AFTER_YR`], `Ago` from there to now,
    /// `Ahead` in the future.
    pub fn at(u: f64) -> When {
        let now = u_now();
        if u > now {
            // ε (10^(u−u_now) − 1), written to stay exact near now
            return When::Ahead(EPS_YR * ((u - now) * LN10).exp_m1());
        }
        // u = log t − log(T + ε − t)  ⇒  t = r (T + ε) / (1 + r), r = 10^u
        let full = AGE_YR + EPS_YR;
        if u <= u_switch() {
            let r = 10f64.powf(u);
            When::After(r * full / (1.0 + r))
        } else {
            let r_inv = 10f64.powf(-u);
            // ago = T − t = (T + ε)/(1 + r) − ε, computed without cancelling t
            When::Ago((full * r_inv / (1.0 + r_inv) - EPS_YR).max(0.0))
        }
    }

    /// Signed years from now (negative = past). Loses precision for early
    /// moments; use [`years_between`] for spans.
    pub fn from_now(self) -> f64 {
        match self {
            When::After(t) => t - AGE_YR,
            When::Ago(a) => -a,
            When::Ahead(y) => y,
        }
    }

    /// Years per unit of `u` at this moment: the local scale of the axis.
    pub fn years_per_u(self) -> f64 {
        match self {
            When::Ahead(y) => LN10 * (y.max(0.0) + EPS_YR),
            When::After(t) => LN10 * t * (AGE_YR - t + EPS_YR) / (AGE_YR + EPS_YR),
            When::Ago(a) => LN10 * (AGE_YR - a) * (a + EPS_YR) / (AGE_YR + EPS_YR),
        }
    }
}

fn past_u(t: f64, ago: f64) -> f64 {
    t.log10() - (ago.max(0.0) + EPS_YR).log10()
}

/// Where ticks and dates switch from "after the Big Bang" to "ago".
pub fn u_switch() -> f64 {
    When::After(SWITCH_AFTER_YR).u()
}

/// Axis position of now.
pub fn u_now() -> f64 {
    AGE_YR.log10() - EPS_YR.log10()
}

/// Left end of the axis: the Planck time.
pub fn u_start() -> f64 {
    When::after_s(PLANCK_S).u()
}

/// Right end of the axis.
pub fn u_end() -> f64 {
    When::Ahead(END_AHEAD_YR).u()
}

/// Years between two moments, exact when both are in the same form.
pub fn years_between(a: When, b: When) -> f64 {
    match (a, b) {
        (When::After(x), When::After(y)) => (y - x).abs(),
        (When::Ago(x), When::Ago(y)) => (y - x).abs(),
        (When::Ahead(x), When::Ahead(y)) => (y - x).abs(),
        _ => (b.from_now() - a.from_now()).abs(),
    }
}

// --- How long: numbers for people ------------------------------------------

/// The unit a duration is told in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Second,
    Minute,
    Hour,
    Day,
    Year,
    /// 10⁶ years
    MillionYears,
    /// 10⁹ years (a "miliarda" in Slovak)
    BillionYears,
    /// 10¹² years (a "bilión" in Slovak)
    TrillionYears,
}

/// A duration ready to be put into words: `value` is the number as text (dot
/// as decimal point, thin-space thousands, or "10⁻³²"), `count` the number
/// for choosing plural forms.
#[derive(Debug, Clone, PartialEq)]
pub struct Amount {
    pub value: String,
    pub count: f64,
    pub unit: Unit,
}

/// Tell a duration in the largest sensible unit, with `sig` significant digits.
pub fn amount(years: f64, sig: usize) -> Amount {
    let years = years.abs();
    let s = years * SEC_PER_YR;
    let make = |v: f64, unit| Amount {
        value: number(v, sig),
        count: crate::fmt::sci_count(v, sig),
        unit,
    };
    if s < 60.0 {
        make(s, Unit::Second)
    } else if s < 3_600.0 {
        make(s / 60.0, Unit::Minute)
    } else if s < 86_400.0 {
        make(s / 3_600.0, Unit::Hour)
    } else if years < 1.0 {
        make(s / 86_400.0, Unit::Day)
    } else if years < 1e6 {
        make(years, Unit::Year)
    } else if years < 1e9 {
        make(years / 1e6, Unit::MillionYears)
    } else if years < 1e12 {
        make(years / 1e9, Unit::BillionYears)
    } else if years < 1e15 {
        make(years / 1e12, Unit::TrillionYears)
    } else {
        make(years, Unit::Year)
    }
}

/// 380000 → "380 000", 0.5 → "0.5", 1e-32 → "10⁻³²", 3e19 → "3 × 10¹⁹".
fn number(v: f64, sig: usize) -> String {
    if v != 0.0 && !(0.01..1e6).contains(&v) {
        let s = crate::fmt::sci(v, sig);
        return s.strip_prefix("1 × ").map(str::to_owned).unwrap_or(s);
    }
    crate::fmt::group_thousands(&crate::fmt::sig_digits(v, sig))
}

// --- Axis ticks --------------------------------------------------------------

/// A labelled tick on the axis.
#[derive(Debug, Clone, PartialEq)]
pub struct Tick {
    pub u: f64,
    pub when: When,
}

/// Ticks for the view `[u0, u1]` drawn `width_px` wide, at least `gap_px`
/// apart. Round values (1, 5, 2 × 10ⁿ first) in the form the axis uses there
/// (see [`When::at`]). Deep in a zoom, where no power-of-ten
/// value is in sight, falls back to evenly spaced round values.
pub fn ticks(u0: f64, u1: f64, width_px: f64, gap_px: f64) -> Vec<Tick> {
    if u0.is_nan() || u1.is_nan() || u1 <= u0 || width_px <= 0.0 {
        return Vec::new();
    }
    let px = |u: f64| (u - u0) / (u1 - u0) * width_px;
    // (rank, tick): lower rank wins when ticks crowd each other
    let mut cand: Vec<(u8, Tick)> = Vec::new();
    let mut push = |w: When, rank: u8| {
        let u = w.u();
        if u >= u0 && u <= u1 && family(u) == form(w) {
            cand.push((rank, Tick { u, when: w }));
        }
    };
    // seconds after the Big Bang (under a minute), round minutes, hours,
    // days, then years after it
    for k in -44..=1 {
        for m in 1..=9 {
            let s = m as f64 * 10f64.powi(k);
            if s < 60.0 {
                push(When::after_s(s), rank(m));
            }
        }
    }
    for s in [60.0, 600.0, 3_600.0, 86_400.0, 30.0 * 86_400.0] {
        push(When::after_s(s), 1);
    }
    for k in 0..=10 {
        for m in 1..=9 {
            let y = m as f64 * 10f64.powi(k);
            if y < SWITCH_AFTER_YR {
                push(When::After(y), rank(m));
            }
            push(When::Ago(y), rank(m));
        }
    }
    for k in 0..=110 {
        for m in 1..=9 {
            push(When::Ahead(m as f64 * 10f64.powi(k)), rank(m));
        }
    }
    if u0 <= u_now() && u_now() <= u1 {
        cand.push((
            0,
            Tick {
                u: u_now(),
                when: When::Ago(0.0),
            },
        ));
    }
    let in_view = cand.len();
    if in_view < 2 {
        cand.extend(linear_ticks(u0, u1).into_iter().map(|t| (4, t)));
    }
    cand.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.u.total_cmp(&b.1.u)));
    let mut kept: Vec<Tick> = Vec::new();
    for (_, t) in cand {
        let x = px(t.u);
        if kept.iter().all(|k| (px(k.u) - x).abs() >= gap_px) {
            kept.push(t);
        }
    }
    kept.sort_by(|a, b| a.u.total_cmp(&b.u));
    kept
}

/// Evenly spaced round values across a narrow view, in the view's own form.
fn linear_ticks(u0: f64, u1: f64) -> Vec<Tick> {
    let (a, b) = (When::at(u0), When::at(u1));
    let (va, vb) = (value(a), value(b));
    let (lo, hi) = (va.min(vb), va.max(vb));
    let step = nice_step((hi - lo) / 4.0);
    if step.is_nan() || step <= 0.0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut v = (lo / step).ceil() * step;
    while v <= hi && out.len() < 20 {
        let w = with_value(a, v);
        out.push(Tick { u: w.u(), when: w });
        v += step;
    }
    out
}

fn value(w: When) -> f64 {
    match w {
        When::After(v) | When::Ago(v) | When::Ahead(v) => v,
    }
}

fn with_value(like: When, v: f64) -> When {
    match like {
        When::After(_) => When::After(v),
        When::Ago(_) => When::Ago(v),
        When::Ahead(_) => When::Ahead(v),
    }
}

/// 1, 2, 2.5 or 5 × 10ⁿ, the largest not above `x`.
fn nice_step(x: f64) -> f64 {
    if x.is_nan() || x <= 0.0 {
        return 0.0;
    }
    let p = 10f64.powf(x.log10().floor());
    [5.0, 2.5, 2.0, 1.0]
        .into_iter()
        .map(|m| m * p)
        .find(|s| *s <= x)
        .unwrap_or(p)
}

fn rank(m: i32) -> u8 {
    match m {
        1 => 0,
        5 => 1,
        2 => 2,
        _ => 3,
    }
}

/// Which form the axis tells time in at `u` (0 = after, 1 = ago, 2 = ahead).
fn family(u: f64) -> u8 {
    form(When::at(u))
}

fn form(w: When) -> u8 {
    match w {
        When::After(_) => 0,
        When::Ago(_) => 1,
        When::Ahead(_) => 2,
    }
}

// --- Events ------------------------------------------------------------------

/// Which row of the timeline an event sits in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lane {
    /// The universe as a whole: its stuff, its light, its end.
    Cosmos,
    /// Stars and galaxies.
    Stars,
    /// The Sun, the Earth, life and people.
    Earth,
}

impl Lane {
    pub const ALL: [Lane; 3] = [Lane::Cosmos, Lane::Stars, Lane::Earth];
}

/// One event on the timeline. Its texts are `tl-ev-<id>` (title) and
/// `tl-ev-<id>.text` (the card) in both catalogs. Values and sources:
/// docs/TIMELINE.md.
#[derive(Debug, Clone, Copy)]
pub struct Event {
    /// Permanent: catalog key and deep link (`#/timeline/<id>`).
    pub id: &'static str,
    pub when: When,
    pub lane: Lane,
    /// 1 = a turning point of everything, 2 = big, 3 = detail. Drives the
    /// density stripe and which labels survive when they crowd.
    pub importance: u8,
    /// The date is a model's forecast or a rough estimate, not a measurement.
    pub uncertain: bool,
}

const fn ev(id: &'static str, when: When, lane: Lane, importance: u8, uncertain: bool) -> Event {
    Event {
        id,
        when,
        lane,
        importance,
        uncertain,
    }
}

/// The starter set (v0.7): one end to the other, sparse on purpose.
/// Every value is in docs/TIMELINE.md with its source.
pub static EVENTS: &[Event] = &[
    ev(
        "start",
        When::After(PLANCK_S / SEC_PER_YR),
        Lane::Cosmos,
        1,
        false,
    ),
    ev(
        "transparent",
        When::After(380_000.0),
        Lane::Cosmos,
        1,
        false,
    ),
    ev("first-stars", When::After(1.5e8), Lane::Stars, 1, true),
    ev("first-galaxies", When::After(3.0e8), Lane::Stars, 2, true),
    ev("second-stars", When::After(5.0e8), Lane::Stars, 2, true),
    ev("sun", When::Ago(4.567e9), Lane::Earth, 2, false),
    ev("earth", When::Ago(4.54e9), Lane::Earth, 1, false),
    ev("now", When::Ago(0.0), Lane::Earth, 1, false),
    ev("andromeda", When::Ahead(4.5e9), Lane::Stars, 2, true),
    ev("red-giant", When::Ahead(5.0e9), Lane::Earth, 1, true),
    ev("white-dwarf", When::Ahead(8.0e9), Lane::Earth, 2, true),
    ev("last-stars", When::Ahead(1e14), Lane::Stars, 1, true),
    ev("bh-stellar", When::Ahead(1e67), Lane::Cosmos, 2, true),
    ev("bh-biggest", When::Ahead(1e100), Lane::Cosmos, 2, true),
    ev("heat-death", When::Ahead(1e106), Lane::Cosmos, 1, true),
];

pub fn event(id: &str) -> Option<&'static Event> {
    EVENTS.iter().find(|e| e.id == id)
}

/// How much happens around each point of the axis: `n` samples evenly from
/// `u0` to `u1`, each the importance-weighted sum of Gaussian bumps (width
/// `sigma` in `u`) around the events, scaled so the largest sample is 1.
/// Drawn as the colour of the overview stripe: it tells where to zoom.
pub fn density(events: &[Event], u0: f64, u1: f64, n: usize, sigma: f64) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    let weight = |imp: u8| match imp {
        1 => 3.0,
        2 => 2.0,
        _ => 1.0,
    };
    let us: Vec<(f64, f64)> = events
        .iter()
        .map(|e| (e.when.u(), weight(e.importance)))
        .collect();
    let mut out: Vec<f64> = (0..n)
        .map(|i| {
            let u = u0 + (u1 - u0) * (i as f64 + 0.5) / n as f64;
            us.iter()
                .map(|(c, w)| w * (-0.5 * ((u - c) / sigma).powi(2)).exp())
                .sum()
        })
        .collect();
    let max = out.iter().cloned().fold(0.0, f64::max);
    if max > 0.0 {
        out.iter_mut().for_each(|v| *v /= max);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, rel: f64) -> bool {
        (a - b).abs() <= rel * a.abs().max(b.abs()).max(1e-300)
    }

    #[test]
    fn axis_runs_left_to_right_through_now() {
        let order = [
            When::after_s(PLANCK_S),
            When::after_s(1e-36),
            When::after_s(1.0),
            When::After(380_000.0),
            When::After(1e9),
            When::Ago(4.54e9),
            When::Ago(66e6),
            When::Ago(5_000.0),
            When::Ago(100.0),
            When::Ago(1.0),
            When::Ago(0.0),
            When::Ahead(0.5),
            When::Ahead(1e9),
            When::Ahead(1e100),
        ];
        for w in order.windows(2) {
            assert!(w[0].u() < w[1].u(), "{:?} !< {:?}", w[0], w[1]);
        }
        assert!(close(When::Ago(0.0).u(), u_now(), 1e-12));
        assert!(close(When::Ahead(0.0).u(), u_now(), 1e-12));
        assert!(u_start() < -60.0 && u_end() > 115.0);
    }

    #[test]
    fn the_join_at_now_is_smooth() {
        // slope just before and just after now, in u per year
        let h = 1e-3;
        let before = (When::Ago(0.0).u() - When::Ago(h).u()) / h;
        let after = (When::Ahead(h).u() - When::Ahead(0.0).u()) / h;
        assert!(close(before, after, 1e-2), "{before} vs {after}");
    }

    #[test]
    fn inverse_round_trips_in_every_part() {
        for w in [
            When::after_s(1e-40),
            When::after_s(1e-6),
            When::After(380_000.0),
            When::After(3e8),
            When::Ago(4.54e9),
            When::Ago(2_000.0),
            When::Ago(3.0),
            When::Ago(0.25),
            When::Ahead(0.25),
            When::Ahead(5e9),
            When::Ahead(1e100),
        ] {
            let back = When::at(w.u());
            assert_eq!(form(back), form(w), "{w:?} came back as {back:?}");
            assert!(close(value(back), value(w), 1e-9), "{w:?} → {back:?}");
        }
    }

    #[test]
    fn local_scale_matches_the_slope() {
        for w in [When::After(1e5), When::Ago(1e3), When::Ahead(1e6)] {
            let u = w.u();
            let h = 1e-6;
            let span = years_between(When::at(u - h), When::at(u + h)) / (2.0 * h);
            assert!(close(span, w.years_per_u(), 1e-4), "{w:?}: {span}");
        }
    }

    #[test]
    fn human_history_gets_room() {
        // 1000 BCE to now takes a visible share of the past half
        let past = u_now() - u_start();
        let history = u_now() - When::Ago(3_000.0).u();
        assert!(history / past > 0.04, "{}", history / past);
    }

    #[test]
    fn amounts() {
        let a = amount(1e-32 / SEC_PER_YR, 2);
        assert_eq!(a.unit, Unit::Second);
        assert_eq!(a.value, "10⁻³²");
        let a = amount(380_000.0, 2);
        assert_eq!((a.value.as_str(), a.unit), ("380\u{2009}000", Unit::Year));
        let a = amount(4.54e9, 3);
        assert_eq!((a.value.as_str(), a.unit), ("4.54", Unit::BillionYears));
        assert_eq!(a.count, 4.54);
        let a = amount(1e100, 2);
        assert_eq!((a.value.as_str(), a.unit), ("10¹⁰⁰", Unit::Year));
        assert_eq!(amount(180.0 / SEC_PER_YR, 2).unit, Unit::Minute);
        assert_eq!(amount(1.0, 2).value, "1");
    }

    #[test]
    fn ticks_spread_and_stay_in_view() {
        let (u0, u1) = (u_start(), u_end());
        let t = ticks(u0, u1, 1000.0, 90.0);
        assert!(t.len() >= 5, "{}", t.len());
        for w in t.windows(2) {
            assert!((w[1].u - w[0].u) / (u1 - u0) * 1000.0 >= 90.0 - 1e-9);
        }
        // no odd units: every tick reads as a round number
        for k in &t {
            let a = amount(value(k.when), 3);
            assert!(!a.value.contains('.'), "{:?} → {:?}", k.when, a);
        }
        // around the Earth, everything counts back from now
        let t = ticks(When::Ago(9e9).u(), When::Ahead(1e9).u(), 1200.0, 100.0);
        assert!(t.iter().all(|k| !matches!(k.when, When::After(_))), "{t:?}");
        // zoomed deep into history: 2000 to 2100 years ago
        let (a, b) = (When::Ago(2_100.0).u(), When::Ago(2_000.0).u());
        let t = ticks(a, b, 800.0, 80.0);
        assert!(t.len() >= 2, "{t:?}");
        assert!(t.iter().all(|k| k.u >= a && k.u <= b));
        assert!(t.iter().all(|k| matches!(k.when, When::Ago(_))));
    }

    #[test]
    fn events_are_unique_and_in_order_within_reach() {
        let mut ids: Vec<&str> = EVENTS.iter().map(|e| e.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), EVENTS.len());
        for e in EVENTS {
            let u = e.when.u();
            assert!(u >= u_start() - 1e-9 && u <= u_end(), "{}", e.id);
            assert!((1..=3).contains(&e.importance));
            assert!(
                e.id.chars().all(|c| c.is_ascii_lowercase() || c == '-'),
                "{}",
                e.id
            );
        }
        // the Sun is a little older than the Earth
        assert!(event("sun").unwrap().when.u() < event("earth").unwrap().when.u());
    }

    #[test]
    fn density_peaks_where_events_are() {
        let (u0, u1) = (u_start(), u_end());
        let d = density(EVENTS, u0, u1, 400, 1.0);
        assert_eq!(d.len(), 400);
        assert!(d.iter().all(|v| (0.0..=1.0).contains(v)));
        let at = |u: f64| d[(((u - u0) / (u1 - u0)) * 400.0) as usize];
        // around now is busier than the empty stretch between 10²⁰ and 10⁶⁰ years
        assert!(at(u_now()) > 0.3);
        assert!(at(When::Ahead(1e40).u()) < 0.01);
    }
}
