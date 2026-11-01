//! Same step (lineárna funkcia): a start, then the same step every time.
//!
//! Three examples (apples through zero, a taxi with a start fee, a candle
//! burning down), a guess before the answer, a slider whose settled positions
//! leave dots so the straight line appears from the visitor's own tries
//! (`kit::Tries`), and two taxi offers that cross. The maths is in
//! `sims::linear`; this file only draws. Plan: docs/WHAT-CHANGES-WHAT.md.

use egui::{Align2, Rect, RichText, Sense, Stroke, StrokeKind, pos2, vec2};
use fluent_bundle::FluentArgs;
use serde::{Deserialize, Serialize};
use sims::linear::{Line, line_of_tries, rule_of_three};
use sims::ratio::{Verdict, judge};

use super::{Cx, Topic, heading, storage_key};
use crate::draw::{self, Axis};
use crate::i18n::I18n;
use crate::kit::{self, Tries, TriesPlot};
use crate::theme::{Palette, lerp};

pub const ID: &str = "linear";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
enum Example {
    /// 1.80 € per kilo: a line through zero.
    Apples,
    /// 3 € to get in, then 1.20 € per km.
    #[default]
    Taxi,
    /// 20 cm tall, 2 cm less every hour; gone after 10 hours.
    Candle,
}

impl Example {
    const ALL: [Example; 3] = [Example::Apples, Example::Taxi, Example::Candle];

    fn index(self) -> usize {
        self as usize
    }

    fn param(self) -> &'static str {
        match self {
            Example::Apples => "apples",
            Example::Taxi => "taxi",
            Example::Candle => "candle",
        }
    }

    fn glyph(self) -> &'static str {
        match self {
            Example::Apples => "🍎",
            Example::Taxi => "🚕",
            Example::Candle => "🕯",
        }
    }

    fn line(self) -> Line {
        match self {
            Example::Apples => Line::new(0.0, 1.8),
            Example::Taxi => Line::new(3.0, 1.2),
            Example::Candle => Line::new(20.0, -2.0),
        }
    }

    /// The real value: a candle can't get shorter than nothing.
    fn value(self, x: f64) -> f64 {
        let y = self.line().at(x);
        if self == Example::Candle {
            y.max(0.0)
        } else {
            y
        }
    }

    /// Where the line holds (the candle only until it is gone).
    fn domain_end(self) -> f64 {
        match self {
            Example::Candle => self.line().zero_at().unwrap_or(self.x_max()),
            _ => self.x_max(),
        }
    }

    /// End of the slider and of the x axis; the candle's goes past its end.
    fn x_max(self) -> f64 {
        match self {
            Example::Apples => 10.0,
            Example::Taxi | Example::Candle => 12.0,
        }
    }

    fn y_max(self) -> f64 {
        20.0
    }

    /// The guess: "x1 gives y1, what does x2 give?"
    fn guess_xs(self) -> (f64, f64) {
        match self {
            Example::Apples => (3.0, 5.0),
            Example::Taxi => (4.0, 8.0),
            Example::Candle => (3.0, 6.0),
        }
    }

    fn key(self) -> String {
        format!("ln-ex-{}", self.param())
    }
}

/// Everything the visitor can change; saved with consent.
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
struct State {
    example: Example,
    /// The slider, per example.
    x: [f64; 3],
    tries: [Tries; 3],
    guess: f64,
    a: Line,
    b: Line,
    trip: f64,
    cross_guess: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            example: Example::default(),
            x: [0.0; 3],
            tries: Default::default(),
            guess: 0.0,
            a: OFFER_A,
            b: OFFER_B,
            trip: 5.0,
            cross_guess: 0.0,
        }
    }
}

const OFFER_A: Line = Line::new(6.0, 1.0);
const OFFER_B: Line = Line::new(2.0, 1.5);
const TRIP_MAX: f64 = 20.0;

#[derive(Default)]
pub struct Linear {
    s: State,
    verdict: Option<Verdict>,
    revealed: bool,
    cross_verdict: Option<Verdict>,
    cross_revealed: bool,
}

impl Linear {
    fn pick(&mut self, ex: Example) {
        self.s.example = ex;
        self.verdict = None;
        self.revealed = false;
    }
}

impl Topic for Linear {
    fn id(&self) -> &'static str {
        ID
    }

    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Cx) {
        let (tr, pal) = (cx.tr, cx.pal);
        let meta = content::page(ID).expect("linear is on the site map");
        heading(ui, tr, pal, meta);
        ui.add_space(4.0);
        kit::section(ui, pal, &format!("📏 {}", tr.t("ln-step-title")), |ui| {
            self.examples_ui(ui, tr, pal);
            ui.add_space(8.0);
            self.guess_ui(ui, tr, pal);
            ui.add_space(12.0);
            self.try_ui(ui, tr, pal);
            ui.add_space(4.0);
            self.school_ui(ui, tr, pal);
        });
        kit::section(ui, pal, &format!("🚕 {}", tr.t("ln-offers-title")), |ui| {
            self.offers_ui(ui, tr, pal);
        });
        kit::takeaway(ui, pal, tr.t("ln-rule"));
        ui.add_space(8.0);
        kit::grown_ups(ui, pal, tr, ID, |ui| {
            ui.label(tr.t("ln-grown-formula"));
            ui.label(tr.t("ln-grown-cross"));
            ui.label(tr.t("ln-grown-tries"));
            ui.label(RichText::new(tr.t("ln-grown-sources")).color(pal.weak));
        });
    }

    fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &storage_key(ID), &self.s);
    }

    fn load(&mut self, storage: &dyn eframe::Storage) {
        if let Some(s) = eframe::get_value(storage, &storage_key(ID)) {
            self.s = s;
        }
    }

    /// `#/linear/taxi` opens that example; `/dots` adds four tries, `/answer`
    /// shows the answer, `/crossing` the answer of the two offers.
    fn set_param(&mut self, param: &str) {
        for p in param.split('/') {
            if let Some(ex) = Example::ALL.into_iter().find(|e| e.param() == p) {
                self.pick(ex);
            }
            let ex = self.s.example;
            match p {
                "answer" => self.revealed = true,
                "crossing" => self.cross_revealed = true,
                "dots" => {
                    let xs: &[f64] = if ex == Example::Candle {
                        &[1.0, 4.0, 7.0, 11.0]
                    } else {
                        &[1.0, 3.0, 6.0, 9.0]
                    };
                    for &x in xs {
                        self.s.tries[ex.index()].add(x, ex.value(x));
                    }
                    self.s.x[ex.index()] = xs[xs.len() - 1];
                }
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------- numbers and words

/// Up to two decimals, no trailing zeros, decimal comma in Slovak.
fn num(tr: &I18n, x: f64) -> String {
    let s = format!("{x:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    tr.num(if s == "-0" { "0" } else { s })
}

/// A number with its unit: `which` is "x", "xq" (inside the question) or "y".
/// Prices always with cents ("7,80 €"), like on a price tag.
fn amount(tr: &I18n, ex: Example, which: &str, v: f64) -> String {
    if which == "y" && ex != Example::Candle {
        return euros(tr, v);
    }
    let mut args = FluentArgs::new();
    args.set("n", v);
    args.set("num", num(tr, v));
    tr.ta(&format!("ln-unit-{}-{which}", ex.param()), &args)
}

/// Money with two decimals ("1,20 €"), as on a price tag.
fn euros(tr: &I18n, v: f64) -> String {
    format!("{} €", tr.num(&format!("{v:.2}")))
}

fn verdict_line(ui: &mut egui::Ui, tr: &I18n, pal: &Palette, v: Verdict) {
    let (key, color) = match v {
        Verdict::Exact => ("ln-verdict-exact", pal.series[1]),
        Verdict::Close => ("ln-verdict-close", pal.note),
        Verdict::TooLow => ("ln-verdict-low", pal.series[2]),
        Verdict::TooHigh => ("ln-verdict-high", pal.series[2]),
    };
    ui.label(RichText::new(tr.t(key)).color(color));
}

// ---------------------------------------------------------------- section 1

impl Linear {
    fn examples_ui(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let choices: Vec<kit::Choice<Example>> = Example::ALL
            .iter()
            .map(|e| kit::Choice {
                value: *e,
                label: format!("{} {}", e.glyph(), tr.t(&e.key())),
                disabled: None,
            })
            .collect();
        let mut picked = self.s.example;
        if kit::choice_bar(ui, pal, &tr.t("ln-examples"), &choices, &mut picked) {
            self.pick(picked);
        }
        ui.add_space(6.0);
        ui.label(tr.attr(&self.s.example.key(), "story"));
    }

    fn guess_ui(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let ex = self.s.example;
        let (x1, x2) = ex.guess_xs();
        let (y1, answer) = (ex.value(x1), ex.value(x2));
        let mut args = FluentArgs::new();
        args.set("x1", amount(tr, ex, "xq", x1));
        args.set("y1", amount(tr, ex, "y", y1));
        args.set("x2", amount(tr, ex, "xq", x2));
        ui.label(
            RichText::new(tr.ta(&format!("ln-q-{}", ex.param()), &args))
                .size(24.0)
                .color(pal.strong),
        );
        ui.add_space(4.0);
        kit::inset(pal).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(tr.t("ln-guess-first"))
                    .size(kit::CAPTION)
                    .color(pal.weak),
            );
            ui.horizontal_wrapped(|ui| {
                ui.label(tr.t("ln-your-guess"));
                ui.label(RichText::new(tr.t(&format!("ln-qty-{}-y", ex.param()))).color(pal.weak));
                let r = ui.add(
                    egui::DragValue::new(&mut self.s.guess)
                        .speed(0.1)
                        .max_decimals(2),
                );
                ui.label(tr.t(&format!("ln-sym-{}-y", ex.param())));
                if r.changed() {
                    self.verdict = None;
                }
                if ui.button(tr.t("ln-check")).clicked() {
                    let v = judge(self.s.guess, answer);
                    self.verdict = Some(v);
                    self.revealed |= v == Verdict::Exact;
                }
                if ui.button(format!("👁 {}", tr.t("ln-show"))).clicked() {
                    self.revealed = true;
                }
            });
            if let Some(v) = self.verdict {
                verdict_line(ui, tr, pal, v);
                // the rule-of-three guess: say why it is wrong
                let r3 = rule_of_three(x1, y1, x2).unwrap_or(f64::NAN);
                if v != Verdict::Exact && judge(self.s.guess, r3) == Verdict::Exact {
                    ui.label(RichText::new(tr.attr(&ex.key(), "trap")).color(pal.series[2]));
                }
            }
            if self.revealed {
                let mut args = FluentArgs::new();
                args.set("x", amount(tr, ex, "x", x2));
                args.set("y", amount(tr, ex, "y", answer));
                ui.label(
                    RichText::new(tr.ta("ln-answer", &args))
                        .size(20.0)
                        .color(pal.series[1]),
                );
                ui.label(RichText::new(working(tr, ex, x2)).color(pal.text));
            }
        });
    }

    fn try_ui(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let ex = self.s.example;
        let i = ex.index();
        ui.label(
            RichText::new(tr.t("ln-try"))
                .size(kit::CAPTION)
                .color(pal.weak),
        );
        ui.label(tr.t("ln-try-hint"));
        ui.add_space(2.0);
        let settled = kit::try_slider(
            ui,
            tr,
            &tr.t(&format!("ln-qty-{}-x", ex.param())),
            &mut self.s.x[i],
            0.0..=ex.x_max(),
            0.5,
            &tr.t(&format!("ln-sym-{}-x", ex.param())),
        );
        let x = self.s.x[i];
        let y = ex.value(x);
        if settled {
            self.s.tries[i].add(x, y);
        }
        let mut args = FluentArgs::new();
        args.set("x", amount(tr, ex, "x", x));
        args.set("y", amount(tr, ex, "y", y));
        ui.label(
            RichText::new(tr.ta("ln-now", &args))
                .size(20.0)
                .color(pal.strong),
        );
        ui.add_space(4.0);

        // scene beside the tries plot; stacked on a phone (DESIGN.md section 7)
        let avail = ui.available_width();
        let narrow = avail < 560.0;
        let (scene_w, plot_w) = if narrow {
            (avail, avail)
        } else {
            (230.0, avail - 230.0 - 8.0)
        };
        let tries = self.s.tries[i].clone();
        let tries = &tries;
        let shape = line_of_tries(tries.points(), 3, 0.01);
        let dir = if narrow {
            egui::Layout::top_down(egui::Align::Min)
        } else {
            egui::Layout::left_to_right(egui::Align::Min)
        };
        ui.with_layout(dir, |ui| {
            let scene_h = if narrow { 190.0 } else { 260.0 };
            let (resp, p) = ui.allocate_painter(vec2(scene_w, scene_h), Sense::hover());
            scene(&p, pal, tr, resp.rect, ex, x);
            let (resp, p) = ui.allocate_painter(vec2(plot_w, 260.0), Sense::hover());
            // the axis shows the symbol, never a plural word
            let sym_x = tr.t(&format!("ln-sym-{}-x", ex.param()));
            let xl = |v: f64| format!("{} {sym_x}", num(tr, v));
            let sym_y = tr.t(&format!("ln-sym-{}-y", ex.param()));
            let yl = |v: f64| format!("{} {sym_y}", num(tr, v));
            let f = |v: f64| shape.map_or(0.0, |l| l.at(v));
            kit::tries_plot(
                &p,
                pal,
                resp.rect,
                &TriesPlot {
                    x: Axis {
                        max: ex.x_max(),
                        label: &xl,
                    },
                    y: Axis {
                        max: ex.y_max(),
                        label: &yl,
                    },
                    tries,
                    now: Some((x, y)),
                    shape: shape.map(|_| (&f as &dyn Fn(f64) -> f64, 0.0, ex.domain_end())),
                    tries_label: tr.t("ln-tries-label"),
                    shape_label: tr.t("ln-shape-label"),
                },
            );
        });
        // what the dots say
        ui.horizontal_wrapped(|ui| {
            let distinct = {
                let mut xs: Vec<i64> = tries
                    .points()
                    .iter()
                    .map(|t| (t.0 * 1000.0) as i64)
                    .collect();
                xs.sort_unstable();
                xs.dedup();
                xs.len()
            };
            let (text, color) = match shape {
                Some(l) => {
                    let mut args = FluentArgs::new();
                    args.set("x", amount(tr, ex, "x", 1.0));
                    let sign = if l.step < 0.0 { "−" } else { "+" };
                    args.set("y", format!("{sign}{}", amount(tr, ex, "y", l.step.abs())));
                    (tr.ta("ln-tries-line", &args), pal.series[1])
                }
                None if distinct < 3 => (tr.t("ln-tries-more"), pal.weak),
                None if ex == Example::Candle => (tr.attr(&ex.key(), "broken"), pal.series[2]),
                None => (tr.t("ln-tries-off"), pal.series[2]),
            };
            ui.label(RichText::new(text).color(color));
            if !self.s.tries[i].is_empty() && ui.button(tr.t("ln-start-over")).clicked() {
                self.s.tries[i].clear();
            }
        });
    }

    fn school_ui(&self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let ex = self.s.example;
        let l = ex.line();
        egui::CollapsingHeader::new(format!("🎓 {}", tr.t("ln-school-title")))
            .id_salt("ln-school")
            .default_open(false)
            .show(ui, |ui| {
                ui.label(tr.t("ln-school-what"));
                let mut args = FluentArgs::new();
                args.set("k", num(tr, l.step).replace('-', "−"));
                args.set("q", num(tr, l.start));
                args.set("kwhat", tr.t(&format!("ln-k-{}", ex.param())));
                ui.label(RichText::new(tr.ta("ln-school-formula", &args)).color(pal.accent));
                ui.label(tr.t("ln-school-zero"));
                ui.label(RichText::new(tr.attr(&ex.key(), "domain")).color(pal.weak));
            });
    }
}

/// "3 € + 8 × 1,20 € = 12,60 €", with the right sign and no "0 € +" for apples.
fn working(tr: &I18n, ex: Example, x: f64) -> String {
    let l = ex.line();
    let y = |v: f64| amount(tr, ex, "y", v);
    let mut args = FluentArgs::new();
    args.set("start", y(l.start));
    args.set("n", num(tr, x));
    args.set("step", y(l.step.abs()));
    args.set("y", y(ex.value(x)));
    let key = if l.through_zero() {
        "ln-why-zero"
    } else if l.step < 0.0 {
        "ln-why-down"
    } else {
        "ln-why-up"
    };
    tr.ta(key, &args)
}

/// The scene: one column, the start at the bottom and one brick per step on
/// top of it. A candle loses its bricks: the burnt ones stay as outlines.
fn scene(p: &egui::Painter, pal: &Palette, tr: &I18n, r: Rect, ex: Example, x: f64) {
    draw::canvas(p, pal, r);
    let l = ex.line();
    let y = ex.value(x);
    let inner = r.shrink(12.0);
    let (bottom, top) = (inner.bottom() - 4.0, inner.top() + 24.0);
    let h = |v: f64| bottom - (v / ex.y_max()) as f32 * (bottom - top);
    let (x0, x1) = (inner.left() + 8.0, inner.left() + 64.0);
    let col = |a: f64, b: f64| Rect::from_min_max(pos2(x0, h(a.max(b))), pos2(x1, h(a.min(b))));
    let label_x = x1 + 10.0;

    // the start
    if !l.through_zero() && l.step > 0.0 {
        p.rect_filled(col(0.0, l.start).shrink2(vec2(0.0, 0.8)), 2.0, pal.strong);
        let mut a = FluentArgs::new();
        a.set("v", euros(tr, l.start));
        draw::text(
            p,
            pos2(label_x, h(l.start / 2.0)),
            Align2::LEFT_CENTER,
            tr.ta("ln-scene-start", &a),
            13.0,
            pal.strong,
        );
    }
    let step = l.step.abs();
    let mut a = FluentArgs::new();
    a.set("v", amount(tr, ex, "y", step));
    let step_label = tr.ta(&format!("ln-scene-step-{}", ex.param()), &a);
    if l.step > 0.0 {
        // bricks from the start up to y, one per unit of x
        let n = x.ceil() as usize;
        for k in 0..n {
            let lo = l.start + step * k as f64;
            let hi = (lo + step).min(y);
            let c = if k % 2 == 0 {
                pal.accent
            } else {
                lerp(pal.accent, pal.deep, 0.22)
            };
            p.rect_filled(col(lo, hi).shrink2(vec2(0.0, 0.8)), 2.0, c);
        }
        if n > 0 {
            let first = l.start + step / 2.0;
            draw::text(
                p,
                pos2(label_x, h(first)),
                Align2::LEFT_CENTER,
                step_label,
                13.0,
                pal.accent,
            );
        }
    } else {
        // what is left, and the burnt bricks as outlines above it
        p.rect_filled(col(0.0, y), 2.0, lerp(pal.strong, pal.note, 0.25));
        let burnt = (x.min(ex.domain_end())).ceil() as usize;
        for k in 0..burnt {
            let hi = l.start - step * k as f64;
            let lo = (hi - step).max(y);
            if hi - lo > 1e-9 {
                p.rect_stroke(
                    col(lo, hi).shrink(1.0),
                    2.0,
                    Stroke::new(1.0, lerp(pal.deep, pal.weak, 0.6)),
                    StrokeKind::Inside,
                );
            }
        }
        if y > 0.0 {
            // the flame
            let c = pos2((x0 + x1) / 2.0, h(y) - 9.0);
            p.circle_filled(c + vec2(0.0, 2.0), 5.5, pal.note);
            p.add(egui::Shape::convex_polygon(
                vec![
                    c + vec2(-5.2, 1.0),
                    c + vec2(0.0, -11.0),
                    c + vec2(5.2, 1.0),
                ],
                pal.note,
                Stroke::NONE,
            ));
        }
        draw::text(
            p,
            pos2(label_x, h(l.start - step / 2.0)),
            Align2::LEFT_CENTER,
            step_label,
            13.0,
            pal.weak,
        );
    }
    // the value now, at the top of the column
    draw::tag(
        p,
        pal,
        pos2(
            (x0 + x1) / 2.0,
            h(y) - if l.step < 0.0 && y > 0.0 { 22.0 } else { 4.0 },
        ),
        Align2::CENTER_BOTTOM,
        amount(tr, ex, "y", y),
        14.0,
        pal.strong,
    );
    p.line_segment(
        [pos2(inner.left(), bottom), pos2(inner.right(), bottom)],
        Stroke::new(1.0, pal.line),
    );
}

// ---------------------------------------------------------------- section 2

impl Linear {
    fn offers_ui(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("ln-offers-story"));
        ui.add_space(4.0);
        let colors = [pal.series[3], pal.series[4]];
        let mut changed = false;
        for (k, (name, offer)) in [("ln-offer-a", &mut self.s.a), ("ln-offer-b", &mut self.s.b)]
            .into_iter()
            .enumerate()
        {
            ui.horizontal_wrapped(|ui| {
                ui.add_sized(
                    [64.0, 20.0],
                    egui::Label::new(RichText::new(tr.t(name)).strong().color(colors[k])),
                );
                ui.add(egui::Label::new(tr.t("ln-offer-start")).extend());
                changed |= ui
                    .add(
                        egui::DragValue::new(&mut offer.start)
                            .range(0.0..=20.0)
                            .speed(0.1)
                            .max_decimals(2),
                    )
                    .changed();
                ui.label("€");
                ui.add_space(10.0);
                ui.add(egui::Label::new(tr.t("ln-offer-step")).extend());
                changed |= ui
                    .add(
                        egui::DragValue::new(&mut offer.step)
                            .range(0.0..=5.0)
                            .speed(0.05)
                            .max_decimals(2),
                    )
                    .changed();
                ui.label("€");
            });
        }
        if changed {
            self.cross_verdict = None;
            self.cross_revealed = false;
        }
        let (a, b) = (self.s.a, self.s.b);
        let cross = a.crossing(&b).filter(|x| *x > 0.0);
        ui.add_space(6.0);

        kit::inset(pal).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(tr.t("ln-offers-ask"))
                    .size(18.0)
                    .color(pal.strong),
            );
            ui.horizontal_wrapped(|ui| {
                ui.label(tr.t("ln-offers-guess"));
                let r = ui.add(
                    egui::DragValue::new(&mut self.s.cross_guess)
                        .range(0.0..=100.0)
                        .speed(0.1)
                        .max_decimals(1),
                );
                ui.label("km");
                if r.changed() {
                    self.cross_verdict = None;
                }
                if ui
                    .add_enabled(cross.is_some(), egui::Button::new(tr.t("ln-check")))
                    .clicked()
                    && let Some(x) = cross
                {
                    let v = judge(self.s.cross_guess, x);
                    self.cross_verdict = Some(v);
                    self.cross_revealed |= v == Verdict::Exact;
                }
                if ui.button(format!("👁 {}", tr.t("ln-show"))).clicked() {
                    self.cross_revealed = true;
                }
            });
            if let Some(v) = self.cross_verdict {
                verdict_line(ui, tr, pal, v);
            }
            if self.cross_revealed {
                ui.label(RichText::new(offers_answer(tr, a, b)).color(pal.series[1]));
            }
        });
        ui.add_space(8.0);

        let mut trip = self.s.trip;
        kit::try_slider(
            ui,
            tr,
            &tr.t("ln-trip"),
            &mut trip,
            0.0..=TRIP_MAX,
            0.5,
            "km",
        );
        self.s.trip = trip;
        let (pa, pb) = (a.at(trip), b.at(trip));
        let mut args = FluentArgs::new();
        args.set("a", euros(tr, pa));
        args.set("b", euros(tr, pb));
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(tr.ta("ln-offers-now", &args))
                    .size(18.0)
                    .color(pal.strong),
            );
            let d = (pa - pb).abs();
            let mut args = FluentArgs::new();
            args.set("d", euros(tr, d));
            let (key, c) = if d < 0.005 {
                ("ln-offers-same", pal.series[1])
            } else if pa < pb {
                ("ln-offers-cheaper-a", colors[0])
            } else {
                ("ln-offers-cheaper-b", colors[1])
            };
            ui.label(RichText::new(tr.ta(key, &args)).color(c));
        });

        let (resp, p) = ui.allocate_painter(vec2(ui.available_width(), 260.0), Sense::hover());
        offers_plot(
            &p,
            pal,
            tr,
            resp.rect,
            a,
            b,
            trip,
            self.cross_revealed.then_some(cross).flatten(),
        );
        ui.label(
            RichText::new(format!("🐄 {}", tr.t("ln-offers-cow")))
                .small()
                .color(pal.weak),
        );
    }
}

fn offers_answer(tr: &I18n, a: Line, b: Line) -> String {
    match a.crossing(&b) {
        None => tr.t("ln-offers-parallel"),
        Some(x) if x <= 0.0 => tr.t("ln-offers-never"),
        Some(x) => {
            let mut args = FluentArgs::new();
            args.set("x", format!("{} km", num(tr, x)));
            args.set("y", euros(tr, a.at(x)));
            // shorter trips: the cheaper start wins
            let (short, long) = if a.start < b.start {
                ("A", "B")
            } else {
                ("B", "A")
            };
            args.set("short", short);
            args.set("long", long);
            tr.ta("ln-offers-cross", &args)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn offers_plot(
    p: &egui::Painter,
    pal: &Palette,
    tr: &I18n,
    r: Rect,
    a: Line,
    b: Line,
    trip: f64,
    cross: Option<f64>,
) {
    draw::canvas(p, pal, r);
    let ymax = draw::nice_max(a.at(TRIP_MAX).max(b.at(TRIP_MAX)).max(1.0));
    let xl = |v: f64| format!("{} km", num(tr, v));
    let yl = |v: f64| format!("{} €", num(tr, v));
    let (to, plot) = draw::grid_axes(
        p,
        pal,
        r.shrink(4.0),
        &Axis {
            max: TRIP_MAX,
            label: &xl,
        },
        &Axis {
            max: ymax,
            label: &yl,
        },
    );
    let colors = [pal.series[3], pal.series[4]];
    for (l, c) in [(a, colors[0]), (b, colors[1])] {
        p.line_segment(
            [to(0.0, l.start), to(TRIP_MAX, l.at(TRIP_MAX))],
            Stroke::new(2.5, c),
        );
    }
    let top = to(trip, ymax);
    draw::dashed(p, to(trip, 0.0), top, 1.3, pal.strong);
    for (l, c) in [(a, colors[0]), (b, colors[1])] {
        p.circle_filled(to(trip, l.at(trip)), 5.0, c);
    }
    if let Some(x) = cross.filter(|x| *x <= TRIP_MAX) {
        let c = to(x, a.at(x));
        p.circle_filled(c, 6.0, pal.series[1]);
        draw::tag(
            p,
            pal,
            c + vec2(8.0, 8.0),
            Align2::LEFT_TOP,
            format!("{} km: {}", num(tr, x), tr.t("ln-offers-meet")),
            13.0,
            pal.series[1],
        );
    }
    draw::legend(
        p,
        plot.left_top() + vec2(10.0, 10.0),
        &[
            (colors[0], Some(false), tr.t("ln-offer-a")),
            (colors[1], Some(false), tr.t("ln-offer-b")),
        ],
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn deep_links_pick_example_add_dots_and_reveal() {
        let mut t = Linear::default();
        t.set_param("taxi/dots/answer");
        assert_eq!(t.s.example, Example::Taxi);
        assert!(t.revealed);
        assert_eq!(t.s.tries[Example::Taxi.index()].len(), 4);
        let l = line_of_tries(t.s.tries[1].points(), 3, 0.01).unwrap();
        assert!((l.start - 3.0).abs() < 1e-9 && (l.step - 1.2).abs() < 1e-9);
        // the candle's dots go past the end, so they no longer make a line
        t.set_param("candle/dots");
        assert_eq!(line_of_tries(t.s.tries[2].points(), 3, 0.01), None);
    }

    #[test]
    fn guesses_have_the_answers_on_the_plan() {
        let ans = |e: Example| e.value(e.guess_xs().1);
        assert!((ans(Example::Apples) - 9.0).abs() < 1e-9);
        assert!((ans(Example::Taxi) - 12.6).abs() < 1e-9);
        assert!((ans(Example::Candle) - 8.0).abs() < 1e-9);
        assert_eq!(Example::Candle.value(12.0), 0.0);
    }

    #[test]
    fn words_read_well_in_both_languages() {
        let en = I18n::new(Lang::En);
        assert_eq!(
            working(&en, Example::Taxi, 8.0),
            "3.00 € + 8 × 1.20 € = 12.60 €"
        );
        assert_eq!(
            working(&en, Example::Candle, 6.0),
            "20 cm − 6 × 2 cm = 8 cm"
        );
        assert_eq!(working(&en, Example::Apples, 5.0), "5 × 1.80 € = 9.00 €");
        assert_eq!(amount(&en, Example::Candle, "x", 1.0), "1 hour");
        let sk = I18n::new(Lang::Sk);
        assert_eq!(
            working(&sk, Example::Taxi, 8.0),
            "3,00 € + 8 × 1,20 € = 12,60 €"
        );
        assert_eq!(amount(&sk, Example::Candle, "xq", 1.0), "1 hodinu");
        assert_eq!(amount(&sk, Example::Candle, "xq", 3.0), "3 hodiny");
        assert_eq!(amount(&sk, Example::Candle, "xq", 6.0), "6 hodín");
        assert_eq!(amount(&sk, Example::Candle, "x", 1.5), "1,5 hodiny");
        assert!(offers_answer(&sk, OFFER_A, OFFER_B).starts_with("Pri 8 km stoja obidva 14,00 €"));
    }
}
