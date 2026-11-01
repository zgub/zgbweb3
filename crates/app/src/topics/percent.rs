//! Percent tricks (`percent`): the shortcuts and traps many grown-ups never learnt.
//!
//! Six sections, built to docs/DESIGN.md: percent in your head, swapping the
//! numbers, up and down is not back, discounts don't add, taking VAT out, and
//! headline tricks (percentage points, "risk doubles"). Maths in `sims::percent`.

use egui::{Align2, Painter, Rect, RichText, Sense, Stroke, pos2, vec2};
use fluent_bundle::FluentArgs;
use serde::{Deserialize, Serialize};
use sims::percent::{self as pc, Quick};
use sims::ratio::{Verdict, judge};

use super::{Cx, Topic, heading, storage_key};
use crate::draw;
use crate::i18n::I18n;
use crate::kit;
use crate::theme::{Palette, lerp};

pub const ID: &str = "percent";

/// Up-and-down scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
enum UpDown {
    #[default]
    UpThenDown,
    DownThenUp,
    Half,
    Double,
}

impl UpDown {
    const ALL: [UpDown; 4] = [
        UpDown::UpThenDown,
        UpDown::DownThenUp,
        UpDown::Half,
        UpDown::Double,
    ];

    fn steps(self) -> (f64, f64) {
        match self {
            UpDown::UpThenDown => (20.0, -20.0),
            UpDown::DownThenUp => (-20.0, 20.0),
            UpDown::Half => (-50.0, 50.0),
            UpDown::Double => (100.0, -50.0),
        }
    }

    fn label(self, tr: &I18n) -> String {
        let (a, b) = self.steps();
        format!("{} {}", signed(tr, a), signed(tr, b))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
enum Headline {
    #[default]
    Points,
    Risk,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
struct State {
    number: f64,
    quick: usize,
    swap_p: f64,
    swap_x: f64,
    start: f64,
    first: f64,
    second: f64,
    updown: UpDown,
    disc1: f64,
    disc2: f64,
    disc_guess: f64,
    gross: f64,
    vat: f64,
    headline: Headline,
    from: f64,
    to: f64,
    risk_before: f64,
    risk_after: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            number: 240.0,
            quick: 6, // 15 %
            swap_p: 8.0,
            swap_x: 50.0,
            start: 100.0,
            first: 20.0,
            second: -20.0,
            updown: UpDown::UpThenDown,
            disc1: 30.0,
            disc2: 20.0,
            disc_guess: 0.0,
            gross: 123.0,
            vat: 23.0,
            headline: Headline::Points,
            from: 2.0,
            to: 3.0,
            risk_before: 1.0,
            risk_after: 2.0,
        }
    }
}

#[derive(Default)]
pub struct Percent {
    s: State,
    disc_verdict: Option<Verdict>,
    disc_revealed: bool,
}

impl Topic for Percent {
    fn id(&self) -> &'static str {
        ID
    }

    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Cx) {
        let (tr, pal) = (cx.tr, cx.pal);
        let meta = content::page(ID).expect("percent is on the site map");
        heading(ui, tr, pal, meta);
        ui.add_space(4.0);

        kit::section(ui, pal, &format!("💡 {}", tr.t("pc-head-title")), |ui| {
            self.in_your_head(ui, tr, pal)
        });
        kit::section(ui, pal, &format!("🔄 {}", tr.t("pc-swap-title")), |ui| {
            self.swap(ui, tr, pal)
        });
        kit::section(ui, pal, &format!("📉 {}", tr.t("pc-updown-title")), |ui| {
            self.up_down(ui, tr, pal)
        });
        kit::section(ui, pal, &format!("🏷 {}", tr.t("pc-disc-title")), |ui| {
            self.discounts(ui, tr, pal)
        });
        kit::section(ui, pal, &format!("💶 {}", tr.t("pc-vat-title")), |ui| {
            self.vat(ui, tr, pal)
        });
        kit::section(ui, pal, &format!("📰 {}", tr.t("pc-news-title")), |ui| {
            self.headlines(ui, tr, pal)
        });
        kit::takeaway(ui, pal, tr.t("pc-rule"));
        ui.add_space(8.0);
        kit::grown_ups(ui, pal, tr, ID, |ui| {
            for k in [
                "pc-grown-of",
                "pc-grown-chain",
                "pc-grown-vat",
                "pc-grown-points",
            ] {
                ui.label(tr.t(k));
            }
            ui.label(RichText::new(tr.t("pc-grown-sources")).color(pal.weak));
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

    /// `#/percent/risk` opens the headline section on "risk doubles";
    /// `…/answer` reveals the stacked-discount answer.
    fn set_param(&mut self, param: &str) {
        for p in param.split('/') {
            match p {
                "risk" => self.s.headline = Headline::Risk,
                "points" => self.s.headline = Headline::Points,
                "answer" => self.disc_revealed = true,
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------- words and numbers

fn num(tr: &I18n, x: f64) -> String {
    let s = format!("{x:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    tr.num(s)
}

fn pct(tr: &I18n, x: f64) -> String {
    format!("{} %", num(tr, x))
}

/// "+20 %", "−20 %" (with a real minus sign).
fn signed(tr: &I18n, x: f64) -> String {
    if x < 0.0 {
        format!("−{} %", num(tr, -x))
    } else {
        format!("+{} %", num(tr, x))
    }
}

fn eur(tr: &I18n, x: f64) -> String {
    format!("{} €", num(tr, x))
}

fn args(pairs: &[(&'static str, String)]) -> FluentArgs<'static> {
    let mut a = FluentArgs::new();
    for (k, v) in pairs {
        a.set(*k, v.clone());
    }
    a
}

/// Like [`args`], plus a number `n` that plural selectors can use.
fn argn(pairs: &[(&'static str, String)], n: f64) -> FluentArgs<'static> {
    let mut a = args(pairs);
    a.set("n", n);
    a
}

/// A form-style number field: "Name: [value] symbol" (DESIGN.md "Inputs").
fn field(
    ui: &mut egui::Ui,
    tr: &I18n,
    key: &str,
    v: &mut f64,
    range: std::ops::RangeInclusive<f64>,
    sym: &str,
) -> bool {
    // the label stays on one line; the whole field wraps to the next row instead
    ui.add(egui::Label::new(tr.t(key)).extend());
    let r = ui.add(
        egui::DragValue::new(v)
            .range(range)
            .speed(1.0)
            .max_decimals(2),
    );
    if !sym.is_empty() {
        ui.label(sym);
    }
    ui.add_space(10.0);
    r.changed()
}

fn canvas(ui: &mut egui::Ui, pal: &Palette, h: f32) -> (Rect, Painter) {
    let (resp, p) = ui.allocate_painter(vec2(ui.available_width(), h), Sense::hover());
    draw::canvas(&p, pal, resp.rect);
    (resp.rect, p)
}

// ---------------------------------------------------------------- sections

impl Percent {
    /// 50, 25, 10, 5, 1, 20, 15 % in your head, with the hundred squares.
    fn in_your_head(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("pc-head-intro"));
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            field(ui, tr, "pc-number", &mut self.s.number, 0.0..=1.0e9, "");
        });
        ui.add_space(4.0);
        let choices: Vec<kit::Choice<usize>> = Quick::ALL
            .iter()
            .enumerate()
            .map(|(i, q)| kit::Choice {
                value: i,
                label: pct(tr, q.percent()),
                disabled: None,
            })
            .collect();
        let mut i = self.s.quick.min(Quick::ALL.len() - 1);
        kit::choice_bar(ui, pal, &tr.t("pc-head-pick"), &choices, &mut i);
        self.s.quick = i;
        let q = Quick::ALL[i];
        let x = self.s.number;
        let tenth = x / 10.0;
        let how = match q {
            Quick::Half => tr.ta(
                "pc-how-half",
                &args(&[("x", num(tr, x)), ("r", num(tr, x / 2.0))]),
            ),
            Quick::Quarter => tr.ta(
                "pc-how-quarter",
                &args(&[
                    ("x", num(tr, x)),
                    ("h", num(tr, x / 2.0)),
                    ("r", num(tr, x / 4.0)),
                ]),
            ),
            Quick::Tenth => tr.ta(
                "pc-how-tenth",
                &args(&[("x", num(tr, x)), ("r", num(tr, tenth))]),
            ),
            Quick::HalfTenth => tr.ta(
                "pc-how-five",
                &args(&[("t", num(tr, tenth)), ("r", num(tr, tenth / 2.0))]),
            ),
            Quick::Hundredth => tr.ta(
                "pc-how-one",
                &args(&[("x", num(tr, x)), ("r", num(tr, x / 100.0))]),
            ),
            Quick::TwoTenths => tr.ta(
                "pc-how-twenty",
                &args(&[("t", num(tr, tenth)), ("r", num(tr, tenth * 2.0))]),
            ),
            Quick::TenPlusFive => tr.ta(
                "pc-how-fifteen",
                &args(&[
                    ("t", num(tr, tenth)),
                    ("f", num(tr, tenth / 2.0)),
                    ("r", num(tr, tenth * 1.5)),
                ]),
            ),
        };
        ui.add_space(4.0);
        ui.label(RichText::new(how).size(18.0).color(pal.series[1]));
        ui.add_space(4.0);

        // the hundred squares: p of them, each worth 1 % of the number
        // grid on the left, words on the right; on a phone, words below the grid
        let narrow = ui.available_width() < 520.0;
        let (r, p) = canvas(ui, pal, if narrow { 470.0 } else { 364.0 });
        let inner = r.shrink(12.0);
        let grid_area = Rect::from_min_size(inner.min, vec2(inner.width(), 340.0));
        let n = q.percent() as usize;
        let empty = draw::empty_cell(pal);
        let one = num(tr, x / 100.0);
        let grid = draw::hundred_squares(
            &p,
            pal,
            grid_area,
            |i| if i < n { pal.accent } else { empty },
            Some(&one),
        );
        draw::outline_cell(&p, pal, grid, 10, 0);
        let (tx, top) = if narrow {
            (inner.left() + 2.0, grid.bottom() + 14.0)
        } else {
            (grid.right() + 22.0, inner.top() + 6.0)
        };
        let w = inner.right() - tx;
        let mut y = top;
        for (s, c) in [
            (
                tr.ta("pc-sq-one", &args(&[("v", num(tr, x / 100.0))])),
                pal.strong,
            ),
            (
                tr.ta(
                    "pc-sq-many",
                    &argn(&[("v", num(tr, pc::of(q.percent(), x)))], n as f64),
                ),
                pal.accent,
            ),
        ] {
            y += draw::wrapped(&p, pos2(tx, y), s, 16.0, c, w) + 10.0;
        }
        draw::wrapped(&p, pos2(tx, y + 4.0), tr.t("pc-sq-word"), 14.0, pal.weak, w);

        ui.add_space(4.0);
        egui::CollapsingHeader::new(format!("🎓 {}", tr.t("pc-school-title")))
            .id_salt("pc-school")
            .default_open(false)
            .show(ui, |ui| {
                let part = pc::of(q.percent(), x);
                ui.label(tr.ta(
                    "pc-school-words",
                    &args(&[
                        ("x", num(tr, x)),
                        ("p", num(tr, q.percent())),
                        ("part", num(tr, part)),
                    ]),
                ));
                ui.label(RichText::new(tr.t("pc-school-formula")).color(pal.accent));
            });
    }

    /// 8 % of 50 € is the same as 50 % of 8 €: swap when one way is hard.
    fn swap(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("pc-swap-intro"));
        ui.add_space(4.0);
        // real situations; the keys name the situation ("a tip of 8 % on 50 €")
        let presets = [
            ("pc-swap-tip", 8.0, 50.0),
            ("pc-swap-interest", 4.0, 75.0),
            ("pc-swap-sale", 18.0, 50.0),
            ("pc-swap-fee", 16.0, 25.0),
        ];
        let choices: Vec<kit::Choice<usize>> = presets
            .iter()
            .enumerate()
            .map(|(i, (k, a, b))| kit::Choice {
                value: i,
                label: tr.ta(k, &args(&[("p", num(tr, *a)), ("x", eur(tr, *b))])),
                disabled: None,
            })
            .collect();
        let mut sel = presets
            .iter()
            .position(|(_, a, b)| *a == self.s.swap_p && *b == self.s.swap_x)
            .unwrap_or(usize::MAX);
        if kit::choice_bar(ui, pal, &tr.t("pc-try"), &choices, &mut sel) {
            (_, self.s.swap_p, self.s.swap_x) = presets[sel];
        }
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            field(ui, tr, "pc-percent", &mut self.s.swap_p, 0.0..=100.0, "%");
            field(ui, tr, "pc-amount", &mut self.s.swap_x, 0.0..=1000.0, "€");
        });
        let (a, b) = (self.s.swap_p, self.s.swap_x);
        let v = pc::of(a, b);
        let nums = args(&[
            ("p", num(tr, a)),
            ("x", eur(tr, b)),
            ("xn", num(tr, b)),
            ("pn", num(tr, a)),
            ("v", eur(tr, v)),
        ]);
        kit::inset(pal).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(tr.ta("pc-swap-hard", &nums))
                    .size(18.0)
                    .color(pal.accent),
            );
            ui.label(
                RichText::new(tr.ta("pc-swap-easy", &nums))
                    .size(18.0)
                    .color(pal.series[1]),
            );
            ui.label(RichText::new(tr.ta("pc-swap-why", &nums)).color(pal.weak));
        });
    }

    /// +p then −p is not back.
    fn up_down(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("pc-updown-intro"));
        ui.add_space(4.0);
        let choices: Vec<kit::Choice<UpDown>> = UpDown::ALL
            .iter()
            .map(|u| kit::Choice {
                value: *u,
                label: u.label(tr),
                disabled: None,
            })
            .collect();
        if kit::choice_bar(ui, pal, &tr.t("pc-try"), &choices, &mut self.s.updown) {
            (self.s.first, self.s.second) = self.s.updown.steps();
        }
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            field(ui, tr, "pc-start", &mut self.s.start, 1.0..=1.0e6, "€");
            field(ui, tr, "pc-first", &mut self.s.first, -99.0..=300.0, "%");
            field(ui, tr, "pc-second", &mut self.s.second, -99.0..=300.0, "%");
        });
        let steps = [self.s.first, self.s.second];
        let c = pc::chain(self.s.start, &steps);
        let total = pc::combined(&steps);

        let (r, p) = canvas(ui, pal, 240.0);
        let plot = Rect::from_min_max(r.min + vec2(60.0, 28.0), r.max - vec2(20.0, 30.0));
        let top = c.iter().cloned().fold(0.0_f64, f64::max) * 1.12;
        let y = |v: f64| plot.bottom() - (v / top) as f32 * plot.height();
        let bw = plot.width() / 6.0;
        let colors = [
            pal.accent,
            lerp(pal.accent, pal.series[1], 0.5),
            pal.series[1],
        ];
        let names = [
            tr.t("pc-bar-start"),
            tr.t("pc-bar-after1"),
            tr.t("pc-bar-after2"),
        ];
        let xs: Vec<f32> = (0..3)
            .map(|i| plot.left() + bw * (0.5 + 2.0 * i as f32))
            .collect();
        for i in 0..3 {
            let bar = Rect::from_min_max(pos2(xs[i], y(c[i])), pos2(xs[i] + bw, plot.bottom()));
            p.rect_filled(bar, 2.0, colors[i]);
            draw::text(
                &p,
                pos2(bar.center().x, bar.top() - 4.0),
                Align2::CENTER_BOTTOM,
                eur(tr, c[i]),
                15.0,
                colors[i],
            );
            draw::text(
                &p,
                pos2(bar.center().x, plot.bottom() + 6.0),
                Align2::CENTER_TOP,
                &names[i],
                13.0,
                pal.weak,
            );
        }
        // the starting level, dashed across: where "back" would be
        draw::dashed(
            &p,
            pos2(plot.left(), y(c[0])),
            pos2(plot.right(), y(c[0])),
            1.5,
            pal.strong,
        );
        draw::text(
            &p,
            pos2(plot.left() - 6.0, y(c[0])),
            Align2::RIGHT_CENTER,
            eur(tr, c[0]),
            12.0,
            pal.strong,
        );
        for (i, s) in steps.iter().enumerate() {
            // between the bars, half way up: the change, then an arrow
            let mid = pos2((xs[i] + bw + xs[i + 1]) / 2.0, plot.center().y);
            draw::text(
                &p,
                mid,
                Align2::CENTER_BOTTOM,
                signed(tr, *s),
                13.0,
                pal.text,
            );
            draw::text(&p, mid, Align2::CENTER_TOP, "→", 15.0, pal.text);
        }

        let mut a = vec![("total", signed(tr, total))];
        if let Some(u) = pc::undo(self.s.first) {
            a.push(("first", signed(tr, self.s.first)));
            a.push(("undo", signed(tr, u)));
        }
        ui.add_space(4.0);
        ui.label(
            RichText::new(tr.ta("pc-updown-result", &args(&a)))
                .size(17.0)
                .color(pal.series[1]),
        );
        ui.label(RichText::new(tr.t("pc-updown-why")).color(pal.weak));
    }

    /// −30 % then −20 % is −44 %, not −50 %.
    fn discounts(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("pc-disc-intro"));
        ui.add_space(4.0);
        let mut changed = false;
        ui.horizontal_wrapped(|ui| {
            changed |= field(ui, tr, "pc-disc-first", &mut self.s.disc1, 0.0..=100.0, "%");
            changed |= field(
                ui,
                tr,
                "pc-disc-second",
                &mut self.s.disc2,
                0.0..=100.0,
                "%",
            );
        });
        if changed {
            self.disc_verdict = None;
            self.disc_revealed = false;
        }
        let total = -pc::combined(&[-self.s.disc1, -self.s.disc2]);
        kit::inset(pal).show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label(tr.t("pc-disc-guess"));
                let r = ui.add(
                    egui::DragValue::new(&mut self.s.disc_guess)
                        .range(0.0..=100.0)
                        .speed(1.0),
                );
                ui.label("%");
                if r.changed() {
                    self.disc_verdict = None;
                }
                if ui.button(tr.t("r3-check")).clicked() {
                    let v = judge(self.s.disc_guess, total);
                    self.disc_verdict = Some(v);
                    self.disc_revealed |= v == Verdict::Exact;
                }
                if ui.button(format!("👁 {}", tr.t("r3-show"))).clicked() {
                    self.disc_revealed = true;
                }
            });
            if let Some(v) = self.disc_verdict {
                let (k, c) = match v {
                    Verdict::Exact => ("r3-verdict-exact", pal.series[1]),
                    Verdict::Close => ("pc-close", pal.note),
                    Verdict::TooLow => ("pc-too-low", pal.series[2]),
                    Verdict::TooHigh => ("pc-too-high", pal.series[2]),
                };
                ui.label(RichText::new(tr.t(k)).color(c));
            }
            if self.disc_revealed {
                let a = args(&[
                    ("a", pct(tr, self.s.disc1)),
                    ("b", pct(tr, self.s.disc2)),
                    ("sum", pct(tr, self.s.disc1 + self.s.disc2)),
                    ("total", pct(tr, total)),
                ]);
                ui.label(
                    RichText::new(tr.ta("pc-disc-answer", &a))
                        .size(18.0)
                        .color(pal.series[1]),
                );
            }
        });

        // hundred squares: first discount, second discount (of what is left), what you pay
        let first = self.s.disc1.round().clamp(0.0, 100.0) as usize;
        let second = (pc::of(self.s.disc2, 100.0 - self.s.disc1))
            .round()
            .clamp(0.0, 100.0) as usize;
        // grid on the left, words on the right; on a phone, words below the grid
        let narrow = ui.available_width() < 520.0;
        let (r, p) = canvas(ui, pal, if narrow { 350.0 } else { 230.0 });
        let inner = r.shrink(12.0);
        let grid_area = Rect::from_min_size(inner.min, vec2(inner.width(), 206.0));
        let off1 = lerp(pal.deep, pal.series[2], 0.55);
        let off2 = lerp(pal.deep, pal.series[2], 0.30);
        let grid = draw::unit_grid(&p, grid_area, 10, 10, 1.5, &|i| {
            if i < first {
                off1
            } else if i < first + second {
                off2
            } else {
                pal.series[1]
            }
        });
        let (tx, top) = if narrow {
            (inner.left() + 2.0, grid.bottom() + 14.0)
        } else {
            (grid.right() + 22.0, inner.top() + 6.0)
        };
        let w = inner.right() - tx;
        let pay = 100 - (first + second).min(100);
        let lines = [
            (
                tr.ta(
                    "pc-disc-sq1",
                    &argn(&[("a", pct(tr, self.s.disc1))], first as f64),
                ),
                off1,
            ),
            (
                tr.ta(
                    "pc-disc-sq2",
                    &argn(
                        &[
                            ("b", pct(tr, self.s.disc2)),
                            ("left", (100 - first).to_string()),
                        ],
                        second as f64,
                    ),
                ),
                lerp(off2, pal.text, 0.4),
            ),
            (tr.ta("pc-disc-sq3", &argn(&[], pay as f64)), pal.series[1]),
        ];
        let mut y = top;
        for (s, c) in lines {
            y += draw::wrapped(&p, pos2(tx, y), s, 15.0, c, w) + 10.0;
        }
    }

    /// 123 € with 23 % VAT is 100 € without, not 94.71 €.
    fn vat(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("pc-vat-intro"));
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            field(ui, tr, "pc-vat-gross", &mut self.s.gross, 0.01..=1.0e7, "€");
            field(ui, tr, "pc-vat-rate", &mut self.s.vat, 0.0..=50.0, "%");
        });
        let (g, rate) = (self.s.gross, self.s.vat);
        let net = pc::without_vat(g, rate);
        let wrong = pc::change(g, -rate);

        let (r, p) = canvas(ui, pal, 170.0);
        let bar = Rect::from_min_max(
            pos2(r.left() + 16.0, r.top() + 56.0),
            pos2(r.right() - 16.0, r.top() + 96.0),
        );
        let x = |v: f64| bar.left() + (v / g) as f32 * bar.width();
        let net_r = Rect::from_min_max(bar.min, pos2(x(net), bar.bottom()));
        let vat_r = Rect::from_min_max(pos2(x(net), bar.top()), bar.max);
        p.rect_filled(net_r, 2.0, pal.series[1]);
        p.rect_filled(vat_r, 2.0, lerp(pal.deep, pal.accent, 0.55));
        draw::text(
            &p,
            net_r.center(),
            Align2::CENTER_CENTER,
            tr.ta("pc-vat-net", &args(&[("v", eur(tr, net))])),
            15.0,
            pal.deep,
        );
        draw::text(
            &p,
            vat_r.center(),
            Align2::CENTER_CENTER,
            tr.t("pc-vat-tax"),
            13.0,
            pal.deep,
        );
        draw::bracket(
            &p,
            bar.left(),
            bar.right(),
            bar.top() - 6.0,
            true,
            pal.accent,
            tr.ta("pc-vat-gross-label", &args(&[("v", eur(tr, g))])),
        );
        // the wrong cut
        let wx = x(wrong);
        draw::dashed(
            &p,
            pos2(wx, bar.top() - 4.0),
            pos2(wx, bar.bottom() + 22.0),
            2.0,
            pal.series[2],
        );
        draw::text(
            &p,
            pos2(wx, bar.bottom() + 26.0),
            Align2::CENTER_TOP,
            tr.ta("pc-vat-wrong-label", &args(&[("v", eur(tr, wrong))])),
            13.0,
            pal.series[2],
        );
        let a = args(&[
            ("g", eur(tr, g)),
            ("rate", pct(tr, rate)),
            ("f", num(tr, 1.0 + rate / 100.0)),
            ("net", eur(tr, net)),
            ("wrong", eur(tr, wrong)),
            ("share", pct(tr, rate / (100.0 + rate) * 100.0)),
        ]);
        ui.add_space(4.0);
        ui.label(
            RichText::new(tr.ta("pc-vat-right", &a))
                .size(17.0)
                .color(pal.series[1]),
        );
        ui.label(RichText::new(tr.ta("pc-vat-wrong", &a)).color(pal.series[2]));
        ui.label(RichText::new(tr.ta("pc-vat-why", &a)).color(pal.weak));
    }

    /// Percentage points and "risk doubles".
    fn headlines(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("pc-news-intro"));
        ui.add_space(4.0);
        let choices = [
            kit::Choice {
                value: Headline::Points,
                label: tr.t("pc-news-points"),
                disabled: None,
            },
            kit::Choice {
                value: Headline::Risk,
                label: tr.t("pc-news-risk"),
                disabled: None,
            },
        ];
        kit::choice_bar(ui, pal, &tr.t("pc-try"), &choices, &mut self.s.headline);
        ui.add_space(4.0);
        match self.s.headline {
            Headline::Points => self.points(ui, tr, pal),
            Headline::Risk => self.risk(ui, tr, pal),
        }
    }

    fn points(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("pc-points-story"));
        ui.horizontal_wrapped(|ui| {
            field(ui, tr, "pc-points-from", &mut self.s.from, 0.0..=100.0, "%");
            field(ui, tr, "pc-points-to", &mut self.s.to, 0.0..=100.0, "%");
        });
        let (pts, rel) = pc::points_and_percent(self.s.from, self.s.to);
        let (r, p) = canvas(ui, pal, 150.0);
        let max = (self.s.from.max(self.s.to) * 1.3).max(1.0);
        let x0 = r.left() + 110.0;
        let x1 = r.right() - 30.0;
        let x = |v: f64| x0 + (v / max) as f32 * (x1 - x0);
        for (i, (v, c, key)) in [
            (self.s.from, pal.accent, "pc-points-before"),
            (self.s.to, pal.series[1], "pc-points-after"),
        ]
        .into_iter()
        .enumerate()
        {
            let yy = r.top() + 30.0 + i as f32 * 46.0;
            let bar = Rect::from_min_max(pos2(x0, yy), pos2(x(v), yy + 28.0));
            p.rect_filled(bar, 2.0, c);
            draw::text(
                &p,
                pos2(x0 - 10.0, bar.center().y),
                Align2::RIGHT_CENTER,
                tr.t(key),
                14.0,
                pal.text,
            );
            draw::text(
                &p,
                pos2(bar.right() + 6.0, bar.center().y),
                Align2::LEFT_CENTER,
                pct(tr, v),
                15.0,
                c,
            );
        }
        let mut a = vec![("pts", num(tr, pts.abs()))];
        a.push(("sign", if pts < 0.0 { "−".into() } else { "+".into() }));
        a.push((
            "rel",
            rel.map(|r| signed(tr, r)).unwrap_or_else(|| "—".into()),
        ));
        ui.add_space(4.0);
        ui.label(
            RichText::new(tr.ta("pc-points-result", &argn(&a, pts.abs())))
                .size(17.0)
                .color(pal.series[1]),
        );
        ui.label(RichText::new(tr.t("pc-points-why")).color(pal.weak));
    }

    fn risk(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        ui.label(tr.t("pc-risk-story"));
        ui.horizontal_wrapped(|ui| {
            field(
                ui,
                tr,
                "pc-risk-before",
                &mut self.s.risk_before,
                0.0..=100.0,
                "",
            );
            field(
                ui,
                tr,
                "pc-risk-after",
                &mut self.s.risk_after,
                0.0..=100.0,
                "",
            );
            ui.label(RichText::new(tr.t("pc-risk-of")).color(pal.weak));
        });
        self.s.risk_before = self.s.risk_before.round();
        self.s.risk_after = self.s.risk_after.round();
        let (b, a) = (self.s.risk_before as usize, self.s.risk_after as usize);

        // 10 000 people, one square each; the cases are drawn as rings so they can be found
        let side = ((ui.available_width() - 24.0) / 100.0).clamp(2.0, 5.0);
        let (r, p) = canvas(ui, pal, side * 100.0 + 24.0);
        let grid_r =
            Rect::from_min_size(r.min + vec2(12.0, 12.0), vec2(side * 100.0, side * 100.0));
        let dot = lerp(pal.deep, pal.text, 0.22);
        let grid = draw::unit_grid(&p, grid_r, 100, 100, (side * 0.18).min(0.6), &|_| dot);
        let cell = |i: usize| {
            // spread the cases over the grid so they don't sit in one corner
            let k = (i * 3779 + 1234) % 10_000;
            let (row, col) = (k / 100, k % 100);
            pos2(
                grid.left() + (col as f32 + 0.5) * side,
                grid.top() + (row as f32 + 0.5) * side,
            )
        };
        for i in 0..a.max(b) {
            let c = if i < b { pal.accent } else { pal.series[1] };
            p.circle_stroke(cell(i), 6.0, Stroke::new(2.0, c));
            p.circle_filled(cell(i), side * 0.7, c);
        }
        let tx = grid.right() + 20.0;
        let w = r.right() - tx - 10.0;
        if w > 120.0 {
            let mut y = r.top() + 16.0;
            for (s, c) in [
                (tr.ta("pc-risk-dots", &args(&[])), pal.text),
                (tr.ta("pc-risk-before-n", &argn(&[], b as f64)), pal.accent),
                (
                    tr.ta("pc-risk-after-n", &argn(&[], a as f64)),
                    pal.series[1],
                ),
            ] {
                y += draw::wrapped(&p, pos2(tx, y), s, 15.0, c, w) + 10.0;
            }
        }
        let (pts, rel) = pc::points_and_percent(b as f64 / 100.0, a as f64 / 100.0);
        let aa = argn(
            &[
                (
                    "rel",
                    rel.map(|r| signed(tr, r)).unwrap_or_else(|| "—".into()),
                ),
                ("more", num(tr, (a as f64 - b as f64).abs())),
                ("pts", num(tr, pts.abs())),
            ],
            pts.abs(),
        );
        ui.add_space(4.0);
        ui.label(
            RichText::new(tr.ta("pc-risk-result", &aa))
                .size(17.0)
                .color(pal.series[1]),
        );
        ui.label(RichText::new(tr.t("pc-risk-why")).color(pal.weak));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn signs_and_numbers_read_well() {
        let sk = I18n::new(Lang::Sk);
        assert_eq!(signed(&sk, -20.0), "−20 %");
        assert_eq!(signed(&sk, 12.5), "+12,5 %");
        assert_eq!(eur(&sk, 94.71), "94,71 €");
        let en = I18n::new(Lang::En);
        assert_eq!(UpDown::Half.label(&en), "−50 % +50 %");
    }

    #[test]
    fn deep_links() {
        let mut t = Percent::default();
        t.set_param("risk/answer");
        assert_eq!(t.s.headline, Headline::Risk);
        assert!(t.disc_revealed);
    }
}
