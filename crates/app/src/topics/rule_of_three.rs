//! Rule of three (trojčlenka): "If 180 is 30 %, what is 100 %?"
//!
//! One question, four pictures of the same route (ten pieces, two rulers, a
//! straight line through zero, a hundred squares), a guess before the answer,
//! the inverse case (painters), and two cases where the rule lies. The maths
//! is in `sims::ratio`; this file only draws. Plan: docs/WHAT-CHANGES-WHAT.md.

use egui::{
    Align2, Color32, FontId, Painter, Pos2, Rect, RichText, Sense, Shape, Stroke, StrokeKind, pos2,
    vec2,
};
use fluent_bundle::FluentArgs;
use serde::{Deserialize, Serialize};
use sims::ratio::{Question, Verdict, judge};

use super::{Cx, Topic, heading, storage_key};
use crate::i18n::I18n;
use crate::kit;
use crate::theme::{Palette, lerp};

pub const ID: &str = "rule-of-three";

/// A ready-made question. The units decide the words around the numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
enum Example {
    /// If 180 is 30 %, what is 100 %?
    #[default]
    Percent,
    /// The recipe for 4 people needs 250 ml of milk. How much for 6?
    Pancakes,
    /// 6.5 l per 100 km. How much for 340 km?
    Fuel,
    /// After 30 % off you pay 42 €. What was the price?
    Sale,
}

impl Example {
    const ALL: [Example; 4] = [
        Example::Percent,
        Example::Pancakes,
        Example::Fuel,
        Example::Sale,
    ];

    fn param(self) -> &'static str {
        match self {
            Example::Percent => "percent",
            Example::Pancakes => "pancakes",
            Example::Fuel => "fuel",
            Example::Sale => "sale",
        }
    }

    fn glyph(self) -> &'static str {
        match self {
            Example::Percent => "💯",
            Example::Pancakes => "🍳",
            Example::Fuel => "⛽",
            Example::Sale => "🏷",
        }
    }

    fn question(self) -> Question {
        match self {
            Example::Percent => Question::direct(30.0, 180.0, 100.0),
            Example::Pancakes => Question::direct(4.0, 250.0, 6.0),
            Example::Fuel => Question::direct(100.0, 6.5, 340.0),
            Example::Sale => Question::direct(70.0, 42.0, 100.0),
        }
    }

    /// The first number is a percentage, so the hundred squares make sense.
    fn in_percent(self) -> bool {
        matches!(self, Example::Percent | Example::Sale)
    }

    /// Catalog keys: `r3-ex-<param>` (button), `.story` (the situation).
    fn key(self) -> String {
        format!("r3-ex-{}", self.param())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
enum Picture {
    #[default]
    Pieces,
    Rulers,
    Line,
    Squares,
}

impl Picture {
    const ALL: [Picture; 4] = [
        Picture::Pieces,
        Picture::Rulers,
        Picture::Line,
        Picture::Squares,
    ];

    fn glyph(self) -> &'static str {
        match self {
            Picture::Pieces => "▦",
            Picture::Rulers => "📏",
            Picture::Line => "📈",
            Picture::Squares => "💯",
        }
    }

    fn key(self) -> &'static str {
        match self {
            Picture::Pieces => "r3-pic-pieces",
            Picture::Rulers => "r3-pic-rulers",
            Picture::Line => "r3-pic-line",
            Picture::Squares => "r3-pic-squares",
        }
    }
}

/// Everything the visitor can change; saved with consent.
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
struct State {
    example: Example,
    a: f64,
    b: f64,
    c: f64,
    picture: Picture,
    guess: f64,
    painters: f64,
    days: f64,
    ask_painters: f64,
}

impl Default for State {
    fn default() -> Self {
        let q = Example::Percent.question();
        Self {
            example: Example::Percent,
            a: q.a,
            b: q.b,
            c: q.c,
            picture: Picture::Pieces,
            guess: 0.0,
            painters: 4.0,
            days: 6.0,
            ask_painters: 3.0,
        }
    }
}

#[derive(Default)]
pub struct RuleOfThree {
    s: State,
    /// Last verdict on the guess; cleared when the question changes.
    verdict: Option<Verdict>,
    revealed: bool,
}

impl RuleOfThree {
    fn pick(&mut self, ex: Example) {
        let q = ex.question();
        self.s.example = ex;
        (self.s.a, self.s.b, self.s.c) = (q.a, q.b, q.c);
        if !ex.in_percent() && self.s.picture == Picture::Squares {
            self.s.picture = Picture::Pieces;
        }
        self.reset_guess();
    }

    fn reset_guess(&mut self) {
        self.verdict = None;
        self.revealed = false;
    }

    fn question(&self) -> Question {
        Question::direct(self.s.a, self.s.b, self.s.c)
    }
}

impl Topic for RuleOfThree {
    fn id(&self) -> &'static str {
        ID
    }

    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Cx) {
        let (tr, pal) = (cx.tr, cx.pal);
        let meta = content::page(ID).expect("rule-of-three is on the site map");
        heading(ui, tr, pal, meta);

        ui.add_space(4.0);
        kit::section(ui, pal, &format!("⚖ {}", tr.t("r3-direct-title")), |ui| {
            self.question_ui(ui, tr, pal);
            ui.add_space(10.0);
            self.picture_ui(ui, tr, pal);
            ui.add_space(4.0);
            self.school_ui(ui, tr, pal);
        });
        kit::section(
            ui,
            pal,
            &format!("👥 {}", tr.t("r3-inverse-title")),
            |ui| {
                self.inverse_ui(ui, tr, pal);
            },
        );
        kit::section(ui, pal, &format!("⚠ {}", tr.t("r3-lies-title")), |ui| {
            ui.label(tr.t("r3-lies-intro"));
            ui.add_space(6.0);
            lies_ui(ui, tr, pal);
        });
        kit::takeaway(ui, pal, tr.t("r3-rule"));
        ui.add_space(8.0);
        kit::grown_ups(ui, pal, tr, ID, |ui| {
            ui.label(tr.t("r3-grown-formula"));
            ui.label(tr.t("r3-grown-inverse"));
            ui.label(tr.t("r3-grown-numbers"));
            ui.label(RichText::new(tr.t("r3-grown-sources")).color(pal.weak));
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

    /// `#/rule-of-three/fuel` opens that example; `…/fuel/answer` shows the answer.
    fn set_param(&mut self, param: &str) {
        let mut parts = param.split('/');
        if let Some(ex) = parts
            .next()
            .and_then(|p| Example::ALL.into_iter().find(|e| e.param() == p))
        {
            self.pick(ex);
        }
        for p in parts {
            if p == "answer" {
                self.revealed = true;
            } else if let Some(pic) = Picture::ALL
                .into_iter()
                .find(|x| x.key().trim_start_matches("r3-pic-") == p)
            {
                self.s.picture = pic;
            }
        }
    }
}

// ---------------------------------------------------------------- numbers and words

/// Up to two decimals, no trailing zeros, decimal comma in Slovak.
fn num(tr: &I18n, x: f64) -> String {
    let s = format!("{x:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    tr.num(s)
}

/// A number with its unit, e.g. "30 %", "4 people", "6,5 l".
/// Units with plural forms come from the catalog: `r3-unit-<example>-a/b`.
fn amount(tr: &I18n, ex: Example, which: char, x: f64) -> String {
    let mut args = FluentArgs::new();
    args.set("n", x);
    args.set("num", num(tr, x));
    tr.ta(&format!("r3-unit-{}-{which}", ex.param()), &args)
}

// ---------------------------------------------------------------- the question

impl RuleOfThree {
    fn question_ui(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let choices: Vec<kit::Choice<Example>> = Example::ALL
            .iter()
            .map(|e| kit::Choice {
                value: *e,
                label: format!("{} {}", e.glyph(), tr.t(&e.key())),
                disabled: None,
            })
            .collect();
        let mut picked = self.s.example;
        if kit::choice_bar(ui, pal, &tr.t("r3-examples"), &choices, &mut picked) {
            self.pick(picked);
        }
        let ex = self.s.example;
        ui.add_space(6.0);
        ui.label(tr.attr(&ex.key(), "story"));
        ui.add_space(4.0);

        let q = self.question();
        let mut args = FluentArgs::new();
        args.set("a", amount(tr, ex, 'a', q.a));
        args.set("b", amount(tr, ex, 'b', q.b));
        args.set("c", amount(tr, ex, 'a', q.c));
        ui.label(
            RichText::new(tr.ta(&format!("r3-q-{}", ex.param()), &args))
                .size(24.0)
                .color(pal.strong),
        );
        ui.add_space(4.0);

        // Form style (DESIGN.md "Inputs"): the quantity's name before the field,
        // only a fixed symbol after it, so no plural forms next to numbers.
        let mut changed = false;
        let (name_a, name_b) = (qty(tr, ex, 'a'), qty(tr, ex, 'b'));
        let (sym_a, sym_b) = (sym(tr, ex, 'a'), sym(tr, ex, 'b'));
        let cap_w = 96.0;
        let caption = |ui: &mut egui::Ui, key: &str| {
            ui.add_sized(
                [cap_w, 20.0],
                egui::Label::new(RichText::new(tr.t(key)).color(pal.weak)),
            );
        };
        for row in 0..2 {
            ui.horizontal_wrapped(|ui| {
                caption(ui, if row == 0 { "r3-we-know" } else { "r3-we-ask" });
                ui.label(&name_a);
                let v = if row == 0 {
                    &mut self.s.a
                } else {
                    &mut self.s.c
                };
                let lo = if row == 0 { 0.01 } else { 0.0 };
                changed |= ui
                    .add(egui::DragValue::new(v).range(lo..=1.0e6).speed(1.0))
                    .changed();
                unit_label(ui, sym_a.clone());
                ui.label("  →  ");
                ui.label(&name_b);
                if row == 0 {
                    changed |= ui
                        .add(
                            egui::DragValue::new(&mut self.s.b)
                                .range(0.0..=1.0e9)
                                .speed(1.0),
                        )
                        .changed();
                    unit_label(ui, sym_b.clone());
                } else {
                    ui.label(RichText::new("?").strong().size(18.0).color(pal.series[1]));
                }
            });
        }
        ui.add_space(4.0);
        if changed {
            self.reset_guess();
        }

        let Some(answer) = q.answer() else {
            ui.label(RichText::new(tr.t("r3-no-answer")).color(pal.weak));
            return;
        };
        kit::inset(pal).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label(tr.t("r3-your-guess"));
                ui.label(RichText::new(&name_b).color(pal.weak));
                let r = ui.add(egui::DragValue::new(&mut self.s.guess).speed(1.0));
                unit_label(ui, sym_b.clone());
                if r.changed() {
                    self.verdict = None;
                }
                if ui.button(tr.t("r3-check")).clicked() {
                    self.verdict = Some(judge(self.s.guess, answer));
                    if self.verdict == Some(Verdict::Exact) {
                        self.revealed = true;
                    }
                }
                if ui.button(format!("👁 {}", tr.t("r3-show"))).clicked() {
                    self.revealed = true;
                }
            });
            if let Some(v) = self.verdict {
                let (key, color) = match v {
                    Verdict::Exact => ("r3-verdict-exact", pal.series[1]),
                    Verdict::Close => ("r3-verdict-close", pal.note),
                    Verdict::TooLow => ("r3-verdict-low", pal.series[2]),
                    Verdict::TooHigh => ("r3-verdict-high", pal.series[2]),
                };
                ui.label(RichText::new(tr.t(key)).color(color));
            }
            if self.revealed {
                let mut args = FluentArgs::new();
                args.set("c", amount(tr, ex, 'a', q.c));
                args.set("answer", amount(tr, ex, 'b', answer));
                ui.label(
                    RichText::new(tr.ta("r3-answer", &args))
                        .size(20.0)
                        .color(pal.series[1]),
                );
                if ex == Example::Sale && q == Example::Sale.question() {
                    ui.label(RichText::new(tr.t("r3-sale-trap")).color(pal.series[2]));
                }
            }
        });
    }

    fn picture_ui(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let ex = self.s.example;
        let choices: Vec<kit::Choice<Picture>> = Picture::ALL
            .iter()
            .map(|p| kit::Choice {
                value: *p,
                label: format!("{} {}", p.glyph(), tr.t(p.key())),
                disabled: (*p == Picture::Squares && !ex.in_percent())
                    .then(|| tr.t("r3-squares-only-percent")),
            })
            .collect();
        kit::choice_bar(ui, pal, &tr.t("r3-picture"), &choices, &mut self.s.picture);
        ui.add_space(4.0);
        ui.label(RichText::new(tr.attr(self.s.picture.key(), "hint")).color(pal.weak));

        let q = self.question();
        let width = ui.available_width();
        let (resp, painter) = ui.allocate_painter(vec2(width, 300.0), Sense::hover());
        let r = resp.rect;
        painter.rect_filled(r, 3.0, pal.deep);
        let d = Draw {
            p: &painter,
            pal,
            tr,
            ex,
            q,
            revealed: self.revealed,
        };
        match (q.answer(), self.s.picture) {
            (None, _) => d.message(r, "r3-no-answer"),
            (Some(ans), Picture::Pieces) => d.pieces(r.shrink(12.0), ans),
            (Some(ans), Picture::Rulers) => d.rulers(r.shrink(12.0), ans),
            (Some(ans), Picture::Line) => d.line(r.shrink(12.0), ans),
            (Some(ans), Picture::Squares) => d.squares(r.shrink(12.0), ans),
        }
    }

    fn school_ui(&self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let q = self.question();
        egui::CollapsingHeader::new(format!("🎓 {}", tr.t("r3-school-title")))
            .id_salt("r3-school")
            .default_open(false)
            .show(ui, |ui| {
                ui.label(tr.t("r3-school-direct"));
                if let Some(k) = q.per_one() {
                    let mut args = FluentArgs::new();
                    args.set("k", num(tr, k));
                    args.set("one", amount(tr, self.s.example, 'a', 1.0));
                    args.set("per", amount(tr, self.s.example, 'b', k));
                    ui.label(RichText::new(tr.ta("r3-school-k", &args)).color(pal.accent));
                }
                ui.label(tr.t("r3-school-line"));
                ui.label(RichText::new(tr.t("r3-school-words")).color(pal.weak));
            });
    }

    fn inverse_ui(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("r3-inverse-story"));
        ui.add_space(4.0);
        egui::Grid::new("r3-inverse-inputs")
            .num_columns(4)
            .spacing(vec2(10.0, 6.0))
            .show(ui, |ui| {
                ui.label(tr.t("r3-inv-painters"));
                ui.add(whole(&mut self.s.painters, 12.0));
                ui.label(tr.t("r3-inv-days"));
                ui.add(whole(&mut self.s.days, 20.0));
                ui.end_row();
            });
        ui.horizontal_wrapped(|ui| {
            // no gap before the comma that follows the field
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label(tr.t("r3-inv-ask"));
            ui.add_space(8.0);
            ui.add(whole(&mut self.s.ask_painters, 12.0));
            ui.label(tr.t("r3-inv-ask-end"));
        });
        for v in [
            &mut self.s.painters,
            &mut self.s.days,
            &mut self.s.ask_painters,
        ] {
            *v = v.round();
        }
        let q = Question::inverse(self.s.painters, self.s.days, self.s.ask_painters);
        let Some(days) = q.answer() else { return };
        let mut args = FluentArgs::new();
        args.set("work", num(tr, self.s.painters * self.s.days));
        args.set("p", num(tr, self.s.ask_painters));
        args.set("d", num(tr, days));
        args.set("pn", self.s.ask_painters);
        args.set("dn", days);
        ui.label(RichText::new(tr.ta("r3-inverse-answer", &args)).color(pal.series[1]));

        let rows = self.s.painters.max(self.s.ask_painters) as f32;
        let (resp, painter) = ui.allocate_painter(
            vec2(ui.available_width(), rows * 14.0 + 46.0),
            Sense::hover(),
        );
        let r = resp.rect;
        painter.rect_filled(r, 3.0, pal.deep);
        let cell = 14.0_f32;
        let grid = |origin: Pos2, rows: f64, cols: f64, c: Color32, label: String| {
            let (rows, full) = (rows as usize, cols.floor() as usize);
            let frac = (cols - full as f64) as f32;
            for i in 0..rows {
                for j in 0..=full {
                    let w = if j < full { cell } else { cell * frac };
                    if w < 1.0 {
                        continue;
                    }
                    let sq = Rect::from_min_size(
                        origin + vec2(j as f32 * cell, i as f32 * cell),
                        vec2(w, cell),
                    )
                    .shrink(1.2);
                    painter.rect_filled(sq, 1.5, c);
                }
            }
            painter.text(
                origin - vec2(0.0, 5.0),
                Align2::LEFT_BOTTOM,
                label,
                FontId::proportional(13.0),
                c,
            );
        };
        let mut a = FluentArgs::new();
        a.set("p", num(tr, self.s.painters));
        a.set("d", num(tr, self.s.days));
        grid(
            r.left_top() + vec2(14.0, 30.0),
            self.s.painters,
            self.s.days,
            pal.series[0],
            tr.ta("r3-grid-label", &a),
        );
        let mut a = FluentArgs::new();
        a.set("p", num(tr, self.s.ask_painters));
        a.set("d", num(tr, days));
        let x2 = r.left() + 14.0 + (self.s.days as f32 + 3.0) * cell;
        let x2 = x2.max(r.left() + 220.0);
        if x2 + days as f32 * cell < r.right() {
            grid(
                pos2(x2, r.top() + 30.0),
                self.s.ask_painters,
                days,
                pal.series[1],
                tr.ta("r3-grid-label", &a),
            );
        }
        ui.label(
            RichText::new(format!("🐄 {}", tr.t("r3-inverse-cow")))
                .small()
                .color(pal.weak),
        );
    }
}

/// A unit next to a number field. Skipped when empty: an empty label inside
/// `horizontal_wrapped` sends the next widget back to the start of the row.
fn unit_label(ui: &mut egui::Ui, unit: String) {
    if !unit.is_empty() {
        ui.label(unit);
    }
}

/// Name of a quantity for its input field, e.g. "People" / "Počet osôb".
fn qty(tr: &I18n, ex: Example, which: char) -> String {
    tr.t(&format!("r3-qty-{}-{which}", ex.param()))
}

/// Fixed symbol after an input field ("%", "km", "€"); empty when there is none.
fn sym(tr: &I18n, ex: Example, which: char) -> String {
    tr.t(&format!("r3-sym-{}-{which}", ex.param()))
        .trim()
        .to_string()
}

// ---------------------------------------------------------------- pictures

struct Draw<'a> {
    p: &'a Painter,
    pal: &'a Palette,
    tr: &'a I18n,
    ex: Example,
    q: Question,
    revealed: bool,
}

impl Draw<'_> {
    fn text(&self, pos: Pos2, anchor: Align2, t: impl ToString, size: f32, c: Color32) {
        self.p
            .text(pos, anchor, t.to_string(), FontId::proportional(size), c);
    }

    fn message(&self, r: Rect, key: &str) {
        self.text(
            r.center(),
            Align2::CENTER_CENTER,
            self.tr.t(key),
            15.0,
            self.pal.weak,
        );
    }

    /// The asked value, or "?" until the answer is shown.
    fn hidden(&self, x: f64) -> String {
        if self.revealed {
            amount(self.tr, self.ex, 'b', x)
        } else {
            "?".into()
        }
    }

    fn a(&self, x: f64) -> String {
        amount(self.tr, self.ex, 'a', x)
    }

    fn b(&self, x: f64) -> String {
        amount(self.tr, self.ex, 'b', x)
    }

    fn bracket(&self, x0: f32, x1: f32, y: f32, up: bool, c: Color32, label: String) {
        let d = if up { -6.0 } else { 6.0 };
        let s = Stroke::new(1.4, c);
        self.p.line_segment([pos2(x0, y), pos2(x0, y + d)], s);
        self.p.line_segment([pos2(x1, y), pos2(x1, y + d)], s);
        self.p.line_segment([pos2(x0, y + d), pos2(x1, y + d)], s);
        let (ty, anchor) = if up {
            (y + d - 3.0, Align2::CENTER_BOTTOM)
        } else {
            (y + d + 3.0, Align2::CENTER_TOP)
        };
        self.text(pos2((x0 + x1) / 2.0, ty), anchor, label, 14.0, c);
    }

    fn arrow(&self, from: Pos2, to: Pos2, bend: f32, c: Color32, label: &str) {
        let mid = pos2((from.x + to.x) / 2.0, (from.y + to.y) / 2.0 + bend);
        self.p
            .add(egui::epaint::QuadraticBezierShape::from_points_stroke(
                [from, mid, to],
                false,
                Color32::TRANSPARENT,
                Stroke::new(1.6, c),
            ));
        let dir = (to - mid).normalized();
        let n = vec2(-dir.y, dir.x);
        self.p.add(Shape::convex_polygon(
            vec![to, to - dir * 8.0 + n * 4.0, to - dir * 8.0 - n * 4.0],
            c,
            Stroke::NONE,
        ));
        let lp = pos2(mid.x, (from.y + to.y) / 2.0 + bend * 0.55);
        let g = self
            .p
            .layout_no_wrap(label.to_string(), FontId::proportional(13.0), c);
        let rr = Rect::from_center_size(lp, g.size() + vec2(8.0, 2.0));
        self.p.rect_filled(rr, 3.0, self.pal.deep);
        self.p.galley(rr.min + vec2(4.0, 1.0), g, c);
    }

    fn tag(&self, pos: Pos2, anchor: Align2, s: String, c: Color32) {
        let g = self
            .p
            .layout_no_wrap(s, FontId::proportional(14.0), self.pal.deep);
        let rr = anchor.anchor_size(pos, g.size() + vec2(8.0, 2.0));
        self.p.rect_filled(rr, 3.0, c);
        self.p.galley(rr.min + vec2(4.0, 1.0), g, self.pal.deep);
    }

    /// Ten pieces: the known part and the asked part as pieces of one shared step.
    fn pieces(&self, r: Rect, ans: f64) {
        let Some(st) = self.q.steps().filter(|s| s.down.max(s.up) <= 24) else {
            return self.message(r, "r3-too-many-pieces");
        };
        let pal = self.pal;
        let n = st.down.max(st.up) as usize;
        let piece_b = self.q.b / st.down as f64;
        let bar = Rect::from_min_max(
            pos2(r.left() + 6.0, r.center().y - 26.0),
            pos2(r.right() - 6.0, r.center().y + 18.0),
        );
        let w = bar.width() / n as f32;
        for i in 0..n {
            let seg = Rect::from_min_size(
                pos2(bar.left() + i as f32 * w, bar.top()),
                vec2(w, bar.height()),
            )
            .shrink(1.5);
            let known = i < st.down as usize;
            let asked = i < st.up as usize;
            if known {
                let c = if i == 0 {
                    pal.accent
                } else {
                    lerp(pal.accent, pal.card, 0.18)
                };
                self.p.rect_filled(seg, 2.0, c);
            } else {
                self.p
                    .rect_filled(seg, 2.0, lerp(pal.deep, pal.series[1], 0.12));
                self.p.rect_stroke(
                    seg,
                    2.0,
                    Stroke::new(
                        1.0,
                        lerp(pal.deep, pal.series[1], if asked { 0.7 } else { 0.25 }),
                    ),
                    StrokeKind::Inside,
                );
            }
            if n <= 12 {
                let c = if known { pal.deep } else { pal.series[1] };
                self.text(
                    seg.center(),
                    Align2::CENTER_CENTER,
                    num(self.tr, piece_b),
                    13.0,
                    c,
                );
            }
        }
        let one = Rect::from_min_size(bar.min, vec2(w, bar.height())).expand(2.5);
        self.p
            .rect_stroke(one, 3.0, Stroke::new(2.0, pal.strong), StrokeKind::Outside);

        let mut args = FluentArgs::new();
        args.set("a", self.a(self.q.a));
        args.set("b", self.b(self.q.b));
        self.bracket(
            bar.left(),
            bar.left() + st.down as f32 * w,
            bar.top() - 4.0,
            true,
            pal.accent,
            self.tr.ta("r3-is-pair", &args),
        );
        let mut args = FluentArgs::new();
        args.set("step", self.a(st.step));
        args.set("b", self.b(self.q.b));
        args.set("down", st.down as f64);
        args.set("piece", self.b(piece_b));
        self.text(
            pos2(bar.left(), bar.bottom() + 10.0),
            Align2::LEFT_TOP,
            self.tr.ta("r3-one-piece", &args),
            14.0,
            pal.strong,
        );
        let mut args = FluentArgs::new();
        args.set("c", self.a(self.q.c));
        args.set("up", st.up as f64);
        args.set("piece", self.b(piece_b));
        args.set("answer", self.hidden(ans));
        self.bracket(
            bar.left(),
            bar.left() + st.up as f32 * w,
            bar.bottom() + 36.0,
            false,
            pal.series[1],
            self.tr.ta("r3-all-pieces", &args),
        );
    }

    /// Two rulers: the first quantity on top, the second below, partners line up.
    fn rulers(&self, r: Rect, ans: f64) {
        let pal = self.pal;
        let q = self.q;
        let max_a = nice_max(q.a.max(q.c) * 1.08);
        let (x0, x1) = (r.left() + 24.0, r.right() - 30.0);
        let (yt, yb) = (r.top() + r.height() * 0.38, r.top() + r.height() * 0.62);
        let x = |t: f64| x0 + (t / max_a) as f32 * (x1 - x0);
        let line = Stroke::new(1.5, pal.weak);
        self.p.line_segment([pos2(x0, yt), pos2(x1, yt)], line);
        self.p.line_segment([pos2(x0, yb), pos2(x1, yb)], line);
        let tick = nice_step(max_a / 10.0);
        let mut t = 0.0;
        while t <= max_a + 1e-9 {
            let xx = x(t);
            self.p
                .line_segment([pos2(xx, yt - 4.0), pos2(xx, yt + 4.0)], line);
            self.p
                .line_segment([pos2(xx, yb - 4.0), pos2(xx, yb + 4.0)], line);
            let small = FontId::proportional(12.0);
            self.p.text(
                pos2(xx, yt - 7.0),
                Align2::CENTER_BOTTOM,
                num(self.tr, t),
                small.clone(),
                pal.weak,
            );
            let below = if self.revealed || (t - q.c).abs() > 1e-9 {
                num(self.tr, t * q.b / q.a)
            } else {
                "?".into()
            };
            self.p.text(
                pos2(xx, yb + 7.0),
                Align2::CENTER_TOP,
                below,
                small,
                pal.weak,
            );
            t += tick;
        }
        let pair = |t: f64, c: Color32, bottom: String| {
            let xx = x(t);
            self.p
                .line_segment([pos2(xx, yt), pos2(xx, yb)], Stroke::new(2.5, c));
            self.p.circle_filled(pos2(xx, yt), 5.0, c);
            self.p.circle_filled(pos2(xx, yb), 5.0, c);
            self.tag(pos2(xx, yt - 24.0), Align2::CENTER_BOTTOM, self.a(t), c);
            self.tag(pos2(xx, yb + 24.0), Align2::CENTER_TOP, bottom, c);
        };
        pair(q.a, pal.accent, self.b(q.b));
        let mut from = q.a;
        if let Some(st) = q.steps().filter(|s| s.down > 1) {
            pair(st.step, pal.strong, self.b(q.b / st.down as f64));
            let lab = format!("÷ {}", st.down);
            self.arrow(
                pos2(x(q.a), yt - 52.0),
                pos2(x(st.step), yt - 52.0),
                -14.0,
                pal.strong,
                &lab,
            );
            self.arrow(
                pos2(x(q.a), yb + 52.0),
                pos2(x(st.step), yb + 52.0),
                14.0,
                pal.strong,
                &lab,
            );
            from = st.step;
        }
        pair(q.c, pal.series[1], self.hidden(ans));
        if let Some(st) = q.steps().filter(|s| s.up > 1) {
            let lab = format!("× {}", st.up);
            self.arrow(
                pos2(x(from), yt - 52.0),
                pos2(x(q.c), yt - 52.0),
                -40.0,
                pal.series[1],
                &lab,
            );
            self.arrow(
                pos2(x(from), yb + 52.0),
                pos2(x(q.c), yb + 52.0),
                40.0,
                pal.series[1],
                &lab,
            );
        }
    }

    /// A straight line through zero: every equal step on one axis adds the same on the other.
    fn line(&self, r: Rect, ans: f64) {
        let pal = self.pal;
        let q = self.q;
        let xmax = nice_max(q.a.max(q.c) * 1.1);
        let ymax = nice_max(xmax * q.b / q.a * 1.05);
        let plot = Rect::from_min_max(r.min + vec2(52.0, 8.0), r.max - vec2(12.0, 22.0));
        let to = |x: f64, y: f64| {
            pos2(
                plot.left() + (x / xmax) as f32 * plot.width(),
                plot.bottom() - (y / ymax) as f32 * plot.height(),
            )
        };
        let grid = Stroke::new(1.0, lerp(pal.deep, pal.line, 0.7));
        let small = FontId::proportional(12.0);
        let ys = nice_step(ymax / 6.0);
        let mut y = 0.0;
        while y <= ymax + 1e-9 {
            self.p.line_segment([to(0.0, y), to(xmax, y)], grid);
            self.p.text(
                to(0.0, y) - vec2(6.0, 0.0),
                Align2::RIGHT_CENTER,
                num(self.tr, y),
                small.clone(),
                pal.weak,
            );
            y += ys;
        }
        let xs = nice_step(xmax / 10.0);
        let mut x = 0.0;
        while x <= xmax + 1e-9 {
            self.p.line_segment([to(x, 0.0), to(x, ymax)], grid);
            self.p.text(
                to(x, 0.0) + vec2(0.0, 5.0),
                Align2::CENTER_TOP,
                num(self.tr, x),
                small.clone(),
                pal.weak,
            );
            x += xs;
        }
        let k = q.b / q.a;
        let xend = xmax.min(ymax / k);
        self.p.line_segment(
            [to(0.0, 0.0), to(xend, xend * k)],
            Stroke::new(2.5, pal.accent),
        );
        let dash = |a: Pos2, b: Pos2, c: Color32| {
            self.p
                .extend(Shape::dashed_line(&[a, b], Stroke::new(1.3, c), 5.0, 4.0));
        };
        for (x, y, c, lab) in [
            (
                q.a,
                q.b,
                pal.accent,
                format!("{} → {}", self.a(q.a), self.b(q.b)),
            ),
            (
                q.c,
                ans,
                pal.series[1],
                format!("{} → {}", self.a(q.c), self.hidden(ans)),
            ),
        ] {
            dash(to(x, 0.0), to(x, y), c);
            dash(to(0.0, y), to(x, y), c);
            self.p.circle_filled(to(x, y), 6.0, c);
            self.text(
                to(x, y) + vec2(-10.0, -6.0),
                Align2::RIGHT_BOTTOM,
                lab,
                14.0,
                c,
            );
        }
        // the step, drawn once in the middle of the line
        let step = q.steps().map(|s| s.step).unwrap_or(xs);
        let sx = (xend * 0.5 / step).floor().max(1.0) * step;
        let (p0, p1, p2) = (
            to(sx, sx * k),
            to(sx + step, sx * k),
            to(sx + step, (sx + step) * k),
        );
        if (p1.x - p0.x) > 12.0 {
            self.p.line_segment([p0, p1], Stroke::new(1.5, pal.strong));
            self.p.line_segment([p1, p2], Stroke::new(1.5, pal.strong));
            self.text(
                pos2((p0.x + p1.x) / 2.0, p0.y + 4.0),
                Align2::CENTER_TOP,
                format!("+{}", self.a(step)),
                12.0,
                pal.strong,
            );
            self.text(
                pos2(p1.x + 5.0, (p1.y + p2.y) / 2.0),
                Align2::LEFT_CENTER,
                format!("+{}", self.b(step * k)),
                12.0,
                pal.strong,
            );
            let mut args = FluentArgs::new();
            args.set("step", self.a(step));
            args.set("adds", self.b(step * k));
            self.text(
                pos2(plot.left() + 10.0, plot.top() + 8.0),
                Align2::LEFT_TOP,
                self.tr.ta("r3-line-step", &args),
                13.0,
                pal.text,
            );
        }
    }

    /// A hundred squares: one square is 1 %.
    fn squares(&self, r: Rect, ans: f64) {
        let pal = self.pal;
        let q = self.q;
        let known = q.a.round().clamp(0.0, 100.0) as usize;
        let side = ((r.height() - 4.0) / 10.0).min(28.0);
        let g0 = pos2(r.left() + 4.0, r.top() + 2.0);
        for i in 0..100 {
            let (row, col) = (i / 10, i % 10);
            let sq = Rect::from_min_size(
                g0 + vec2(col as f32 * side, row as f32 * side),
                vec2(side, side),
            )
            .shrink(1.5);
            let c = if i < known {
                pal.accent
            } else {
                lerp(pal.deep, pal.series[1], 0.18)
            };
            self.p.rect_filled(sq, 1.5, c);
        }
        self.p.rect_stroke(
            Rect::from_min_size(g0, vec2(side, side)).expand(1.0),
            2.0,
            Stroke::new(2.2, pal.strong),
            StrokeKind::Outside,
        );
        let tx = g0.x + 10.0 * side + 22.0;
        let per = q.b / q.a;
        let mut l1 = FluentArgs::new();
        l1.set("n", num(self.tr, q.a));
        l1.set("b", self.b(q.b));
        l1.set("count", q.a);
        let mut l2 = FluentArgs::new();
        l2.set("b", self.b(q.b));
        l2.set("n", num(self.tr, q.a));
        l2.set("per", self.b(per));
        let mut l3 = FluentArgs::new();
        l3.set("n", num(self.tr, q.c));
        l3.set("per", self.b(per));
        l3.set("answer", self.hidden(ans));
        l3.set("count", q.c);
        let lines = [
            (self.tr.ta("r3-sq-known", &l1), pal.accent),
            (self.tr.ta("r3-sq-one", &l2), pal.strong),
            (self.tr.ta("r3-sq-asked", &l3), pal.series[1]),
        ];
        for (k, (t, c)) in lines.into_iter().enumerate() {
            self.text(
                pos2(tx, r.top() + 20.0 + k as f32 * 26.0),
                Align2::LEFT_TOP,
                t,
                16.0,
                c,
            );
        }
        let hint = self.tr.t("r3-sq-percent");
        let g = self.p.layout(
            hint,
            FontId::proportional(14.0),
            pal.weak,
            (r.right() - tx - 8.0).max(80.0),
        );
        self.p
            .galley(pos2(tx, r.top() + 20.0 + 3.0 * 26.0 + 12.0), g, pal.weak);
    }
}

/// 1, 2, 2.5 or 5 times a power of ten, at least `x`.
fn nice_step(x: f64) -> f64 {
    let p = 10f64.powf(x.log10().floor());
    for m in [1.0, 2.0, 2.5, 5.0, 10.0] {
        if m * p >= x - 1e-12 {
            return m * p;
        }
    }
    10.0 * p
}

/// A round axis end at least `x`.
fn nice_max(x: f64) -> f64 {
    let s = nice_step(x / 5.0);
    (x / s).ceil() * s
}

// ---------------------------------------------------------------- when it lies

fn lies_ui(ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
    // side by side when there is room, stacked on a phone
    let avail = ui.available_width();
    let w = if avail < 560.0 {
        avail
    } else {
        (avail - 12.0) / 2.0
    };
    let dir = if avail < 560.0 {
        egui::Layout::top_down(egui::Align::Min)
    } else {
        egui::Layout::left_to_right(egui::Align::Min)
    };
    ui.with_layout(dir, |ui| {
        for (title, draw) in [
            (
                "r3-lie-charging",
                charging as fn(&Painter, Rect, &I18n, &Palette),
            ),
            ("r3-lie-taxi", taxi),
        ] {
            kit::inset(pal).show(ui, |ui| {
                ui.set_width(w - 22.0);
                ui.vertical(|ui| {
                    ui.strong(tr.t(title));
                    ui.label(RichText::new(tr.attr(title, "story")).color(pal.weak));
                    let (resp, p) = ui.allocate_painter(vec2(w - 22.0, 220.0), Sense::hover());
                    p.rect_filled(resp.rect, 3.0, pal.deep);
                    draw(&p, resp.rect, tr, pal);
                    ui.label(
                        RichText::new(format!("🐄 {}", tr.attr(title, "cow")))
                            .small()
                            .color(pal.weak),
                    );
                });
            });
        }
    });
}

fn axes(
    p: &Painter,
    r: Rect,
    pal: &Palette,
    x_ticks: &[(f32, String)],
    y_ticks: &[(f32, String)],
) -> Rect {
    let plot = Rect::from_min_max(r.min + vec2(46.0, 10.0), r.max - vec2(12.0, 22.0));
    let grid = Stroke::new(1.0, lerp(pal.deep, pal.line, 0.7));
    let small = FontId::proportional(12.0);
    for (t, s) in y_ticks {
        let y = plot.bottom() - t * plot.height();
        p.line_segment([pos2(plot.left(), y), pos2(plot.right(), y)], grid);
        p.text(
            pos2(plot.left() - 5.0, y),
            Align2::RIGHT_CENTER,
            s,
            small.clone(),
            pal.weak,
        );
    }
    for (t, s) in x_ticks {
        let x = plot.left() + t * plot.width();
        p.text(
            pos2(x, plot.bottom() + 4.0),
            Align2::CENTER_TOP,
            s,
            small.clone(),
            pal.weak,
        );
    }
    plot
}

fn legend(p: &Painter, at: Pos2, rows: &[(Color32, bool, String)]) {
    for (i, (c, dashed, label)) in rows.iter().enumerate() {
        let y = at.y + i as f32 * 17.0;
        let (a, b) = (pos2(at.x, y), pos2(at.x + 18.0, y));
        if *dashed {
            p.extend(Shape::dashed_line(&[a, b], Stroke::new(2.0, *c), 4.0, 3.0));
        } else {
            p.line_segment([a, b], Stroke::new(2.5, *c));
        }
        p.text(
            pos2(at.x + 24.0, y),
            Align2::LEFT_CENTER,
            label,
            FontId::proportional(12.0),
            *c,
        );
    }
}

/// 20 % in 15 minutes; by the rule of three full in 75. Real phones slow down.
fn charging(p: &Painter, r: Rect, tr: &I18n, pal: &Palette) {
    let (tmax, cmax) = (110.0_f32, 100.0_f32);
    let pct = |v: f32| format!("{} %", v as i32);
    let min = |v: f32| format!("{} min", v as i32);
    let plot = axes(
        p,
        r,
        pal,
        &[0.0, 30.0, 60.0, 90.0].map(|t| (t / tmax, min(t))),
        &[0.0, 25.0, 50.0, 75.0, 100.0].map(|c| (c / cmax, pct(c))),
    );
    let to = |t: f32, c: f32| {
        pos2(
            plot.left() + t / tmax * plot.width(),
            plot.bottom() - c / cmax * plot.height(),
        )
    };
    p.extend(Shape::dashed_line(
        &[to(0.0, 0.0), to(75.0, 100.0)],
        Stroke::new(2.0, pal.accent),
        6.0,
        4.0,
    ));
    let pts: Vec<Pos2> = (0..=110)
        .map(|i| {
            let t = i as f32;
            // typical shape: steady to about 80 %, then slower and slower
            let c = if t <= 60.0 {
                80.0 * t / 60.0
            } else {
                100.0 - 20.0 * (-(t - 60.0) / 18.0).exp()
            };
            to(t, c)
        })
        .collect();
    p.add(Shape::line(pts, Stroke::new(2.5, pal.series[1])));
    p.circle_filled(to(15.0, 20.0), 5.0, pal.strong);
    legend(
        p,
        plot.left_top() + vec2(10.0, 10.0),
        &[
            (pal.accent, true, tr.t("r3-lie-charging-rule")),
            (pal.series[1], false, tr.t("r3-lie-charging-real")),
        ],
    );
}

/// 5 km for 9 €: by the rule of three 10 km cost 18 €. With a 3 € start fee they cost 15 €.
fn taxi(p: &Painter, r: Rect, tr: &I18n, pal: &Palette) {
    let (kmax, emax) = (11.0_f32, 20.0_f32);
    let plot = axes(
        p,
        r,
        pal,
        &[0.0, 5.0, 10.0].map(|k| (k / kmax, format!("{} km", k as i32))),
        &[0.0, 5.0, 10.0, 15.0, 20.0].map(|e| (e / emax, format!("{} €", e as i32))),
    );
    let to = |k: f32, e: f32| {
        pos2(
            plot.left() + k / kmax * plot.width(),
            plot.bottom() - e / emax * plot.height(),
        )
    };
    p.extend(Shape::dashed_line(
        &[to(0.0, 0.0), to(10.5, 18.9)],
        Stroke::new(2.0, pal.accent),
        6.0,
        4.0,
    ));
    p.line_segment(
        [to(0.0, 3.0), to(10.5, 15.6)],
        Stroke::new(2.5, pal.series[1]),
    );
    p.circle_filled(to(5.0, 9.0), 5.0, pal.strong);
    p.circle_filled(to(10.0, 18.0), 5.0, pal.accent);
    p.circle_filled(to(10.0, 15.0), 5.0, pal.series[1]);
    legend(
        p,
        plot.left_top() + vec2(10.0, 10.0),
        &[
            (pal.accent, true, tr.t("r3-lie-taxi-rule")),
            (pal.series[1], false, tr.t("r3-lie-taxi-real")),
        ],
    );
}

/// A whole-number input from 1 to `max`.
fn whole(v: &mut f64, max: f64) -> egui::DragValue<'_> {
    egui::DragValue::new(v)
        .max_decimals(0)
        .range(1.0..=max)
        .speed(0.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_axes() {
        assert_eq!(nice_step(9.0), 10.0);
        assert_eq!(nice_step(2.1), 2.5);
        assert_eq!(nice_max(110.0), 125.0);
        assert_eq!(nice_max(374.0), 400.0);
    }

    #[test]
    fn deep_links_pick_example_and_picture() {
        let mut t = RuleOfThree::default();
        t.set_param("fuel/rulers/answer");
        assert_eq!(t.s.example, Example::Fuel);
        assert_eq!(t.s.picture, Picture::Rulers);
        assert!(t.revealed);
        assert_eq!(
            t.question().answer().map(|x| (x * 10.0).round()),
            Some(221.0)
        );
    }

    #[test]
    fn units_read_well_in_both_languages() {
        use crate::i18n::Lang;
        let en = I18n::new(Lang::En);
        assert_eq!(amount(&en, Example::Percent, 'a', 30.0), "30 %");
        assert_eq!(amount(&en, Example::Pancakes, 'a', 1.0), "1 person");
        assert_eq!(amount(&en, Example::Pancakes, 'a', 6.0), "6 people");
        assert_eq!(amount(&en, Example::Fuel, 'b', 6.5), "6.5 l");
        let sk = I18n::new(Lang::Sk);
        assert_eq!(amount(&sk, Example::Fuel, 'b', 6.5), "6,5 l");
        assert_eq!(amount(&sk, Example::Pancakes, 'a', 1.0), "1 osoba");
        assert_eq!(amount(&sk, Example::Pancakes, 'a', 4.0), "4 osoby");
        assert_eq!(amount(&sk, Example::Pancakes, 'a', 6.0), "6 osôb");
    }
}
