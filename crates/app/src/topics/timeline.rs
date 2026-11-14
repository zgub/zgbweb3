//! The timeline of everything (`timeline`): Big Bang to heat death on one
//! zoomable axis. Axis maths, ticks, events and the density measure live in
//! `sims::timeline`; this module draws them and handles wheel, drag and clicks.
//!
//! Two drawings:
//! - the **overview stripe**: the whole axis, always, coloured by how much
//!   happens where, with the current view marked. It tells a visitor where to
//!   zoom and which (very long) stretches hold nothing;
//! - the **track**: the zoomed view, one lane per kind of event, round-value
//!   ticks, click an event to open its card.

use egui::{Align2, Color32, FontId, Pos2, Rect, RichText, Sense, Stroke, StrokeKind, pos2, vec2};
use serde::{Deserialize, Serialize};
use sims::timeline::{self as tl, Amount, Event, Lane, Unit, When};

use super::{Cx, Topic, heading, storage_key};
use crate::args;
use crate::draw;
use crate::i18n::I18n;
use crate::kit;
use crate::theme::{Palette, lerp};

pub const ID: &str = "timeline";

/// Narrowest view, in units of `u` (deep enough for single years in history).
const MIN_WIDTH: f64 = 1e-7;
/// How strongly one wheel step zooms.
const WHEEL_ZOOM: f64 = 0.0025;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
struct State {
    u0: f64,
    u1: f64,
    selected: Option<String>,
    /// 0 = the stretched axis, 1 = honest even spacing (every year as wide).
    fair: f32,
}

impl Default for State {
    fn default() -> Self {
        let (u0, u1) = whole();
        Self {
            u0,
            u1,
            selected: None,
            fair: 0.0,
        }
    }
}

/// The whole axis with a small margin on both sides.
fn whole() -> (f64, f64) {
    let (a, b) = (tl::u_start(), tl::u_end());
    let m = (b - a) * 0.01;
    (a - m, b + m)
}

/// Places to jump to: the view's ends as moments.
#[derive(Clone, Copy, PartialEq)]
enum Jump {
    Whole,
    Beginning,
    Stars,
    SunEarth,
    YoungEarth,
    Dinosaurs,
    People,
    Future,
}

impl Jump {
    const ALL: [Jump; 8] = [
        Jump::Whole,
        Jump::Beginning,
        Jump::Stars,
        Jump::SunEarth,
        Jump::YoungEarth,
        Jump::Dinosaurs,
        Jump::People,
        Jump::Future,
    ];

    fn key(self) -> &'static str {
        match self {
            Jump::Whole => "tl-jump-whole",
            Jump::Beginning => "tl-jump-beginning",
            Jump::Stars => "tl-jump-stars",
            Jump::SunEarth => "tl-jump-sun-earth",
            Jump::YoungEarth => "tl-jump-young-earth",
            Jump::Dinosaurs => "tl-jump-dinosaurs",
            Jump::People => "tl-jump-people",
            Jump::Future => "tl-jump-future",
        }
    }

    fn glyph(self) -> &'static str {
        match self {
            Jump::Whole => "↔",
            Jump::Beginning => "💥",
            Jump::Stars => "⭐",
            Jump::SunEarth => "🌍",
            Jump::YoungEarth => "☄",
            Jump::Dinosaurs => "🐊",
            Jump::People => "👣",
            Jump::Future => "🌑",
        }
    }

    fn view(self) -> (f64, f64) {
        match self {
            Jump::Whole => whole(),
            Jump::Beginning => (tl::u_start() - 1.0, When::After(1e6).u()),
            Jump::Stars => (When::After(1e5).u(), When::After(2e9).u()),
            Jump::SunEarth => (When::Ago(8e9).u(), When::Ahead(2e10).u()),
            Jump::YoungEarth => (When::Ago(4.65e9).u(), When::Ago(3.3e9).u()),
            Jump::Dinosaurs => (When::Ago(360e6).u(), When::Ago(45e6).u()),
            Jump::People => (When::Ago(3.0e6).u(), When::Ago(2.5e3).u()),
            Jump::Future => (When::Ahead(1e9).u(), tl::u_end() + 2.0),
        }
    }
}

#[derive(Default)]
pub struct Timeline {
    s: State,
    /// A view the page is gliding towards (jumps, "zoom here").
    target: Option<(f64, f64)>,
    /// Where the view sits on the stripe this frame (left x, right x, bottom
    /// y), for the lines that link it to the track below.
    window: Option<(f32, f32, f32)>,
}

impl Timeline {
    fn width(&self) -> f64 {
        self.s.u1 - self.s.u0
    }

    /// Keep the view sane: not narrower than [`MIN_WIDTH`], not wider than the
    /// whole axis, and not off either end.
    fn clamp(&mut self) {
        let (a, b) = whole();
        let full = b - a;
        let mut w = self.width().clamp(MIN_WIDTH, full);
        if !w.is_finite() {
            w = full;
        }
        let mut c = (self.s.u0 + self.s.u1) * 0.5;
        if !c.is_finite() {
            c = (a + b) * 0.5;
        }
        let (lo, hi) = (a + w * 0.5, b - w * 0.5);
        c = if lo < hi {
            c.clamp(lo, hi)
        } else {
            (a + b) * 0.5
        };
        self.s.u0 = c - w * 0.5;
        self.s.u1 = c + w * 0.5;
    }

    /// Zoom by `factor` (< 1 = in) keeping `at` in place.
    fn zoom(&mut self, at: f64, factor: f64) {
        self.s.u0 = at - (at - self.s.u0) * factor;
        self.s.u1 = at + (self.s.u1 - at) * factor;
        self.clamp();
    }

    fn pan(&mut self, du: f64) {
        self.s.u0 += du;
        self.s.u1 += du;
        self.clamp();
    }

    fn go_to(&mut self, view: (f64, f64), calm: bool) {
        if calm {
            (self.s.u0, self.s.u1) = view;
            self.clamp();
            self.target = None;
        } else {
            self.target = Some(view);
        }
    }

    /// One frame of gliding: the centre moves linearly, the width moves in
    /// log scale, so a jump from the whole axis to one year looks steady.
    fn glide(&mut self, ui: &egui::Ui) {
        let Some((t0, t1)) = self.target else {
            return;
        };
        let dt = ui.input(|i| i.stable_dt).min(0.05) as f64;
        let k = 1.0 - (-dt * 7.0).exp();
        let (c, w) = ((self.s.u0 + self.s.u1) * 0.5, self.width().max(MIN_WIDTH));
        let (tc, tw) = ((t0 + t1) * 0.5, (t1 - t0).max(MIN_WIDTH));
        let nc = c + (tc - c) * k;
        let nw = (w.ln() + (tw.ln() - w.ln()) * k).exp();
        self.s.u0 = nc - nw * 0.5;
        self.s.u1 = nc + nw * 0.5;
        if (nc - tc).abs() < tw * 1e-3 && (nw / tw - 1.0).abs() < 1e-3 {
            (self.s.u0, self.s.u1) = (t0, t1);
            self.target = None;
        } else {
            ui.ctx().request_repaint();
        }
        self.clamp();
    }

    /// A view around one event: wide enough that its nearest neighbour sits
    /// well apart from it (about a tenth of the view), at most 3 units of `u`.
    fn around(e: &Event) -> (f64, f64) {
        let u = e.when.u();
        let gap = tl::EVENTS
            .iter()
            .filter(|o| o.id != e.id)
            .map(|o| (o.when.u() - u).abs())
            .filter(|d| *d > 1e-9)
            .fold(f64::INFINITY, f64::min);
        let half = (gap * 6.0).clamp(MIN_WIDTH * 50.0, 1.5);
        (u - half, u + half)
    }

    fn select(&mut self, id: &str) {
        self.s.selected = tl::event(id).map(|e| e.id.to_string());
    }
}

impl Topic for Timeline {
    fn id(&self) -> &'static str {
        ID
    }

    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Cx) {
        let (tr, pal) = (cx.tr, cx.pal);
        let meta = content::page(ID).expect("timeline is on the site map");
        heading(ui, tr, pal, meta);
        ui.add_space(4.0);
        self.glide(ui);

        kit::section(ui, pal, &format!("🗺 {}", tr.t("tl-map-title")), |ui| {
            kit::banner(
                ui,
                pal,
                &format!("⚠ {}", tr.t("tl-not-to-scale")),
                &tr.attr("tl-not-to-scale", "text"),
            );
            ui.add_space(8.0);
            ui.label(tr.t("tl-map-intro"));
            ui.add_space(6.0);
            self.stripe(ui, tr, pal);
            ui.add_space(CONNECT_H);
            self.track(ui, tr, pal);
            ui.add_space(6.0);
            ui.label(
                RichText::new(tr.t("tl-jump-caption"))
                    .size(kit::CAPTION)
                    .color(pal.weak),
            );
            ui.horizontal_wrapped(|ui| {
                for j in Jump::ALL {
                    if ui
                        .button(format!("{} {}", j.glyph(), tr.t(j.key())))
                        .clicked()
                    {
                        self.go_to(j.view(), cx.calm);
                    }
                }
            });
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(tr.t("tl-fair")).color(pal.text));
                let r = ui.add(
                    egui::Slider::new(&mut self.s.fair, 0.0..=1.0)
                        .custom_formatter(|v, _| format!("{:.0} %", v * 100.0)),
                );
                if r.changed() {
                    self.target = None;
                }
            });
            if self.s.fair > 0.0 {
                ui.label(RichText::new(tr.t("tl-fair-note")).small().color(pal.note));
            }
            ui.add_space(4.0);
            for key in ["tl-how", "tl-hops"] {
                ui.label(
                    RichText::new(format!("🖱 {}", tr.t(key)))
                        .small()
                        .color(pal.weak),
                );
            }
            let span = tl::years_between(
                When::at(self.s.u0.max(tl::u_start())),
                When::at(self.s.u1.min(tl::u_end())),
            );
            ui.label(
                RichText::new(tr.ta(
                    "tl-view-span",
                    &args!["a" => amount_text(tr, &tl::amount(span, 2))],
                ))
                .small()
                .color(pal.weak),
            );
            if let Some(e) = self.s.selected.as_deref().and_then(tl::event) {
                ui.add_space(8.0);
                self.card(ui, tr, pal, e, cx.calm);
            }
        });

        kit::section(ui, pal, &format!("🚶 {}", tr.t("tl-walk-title")), |ui| {
            walk(ui, tr, pal)
        });

        kit::takeaway(ui, pal, tr.t("tl-rule"));
        ui.add_space(8.0);
        kit::grown_ups(ui, pal, tr, ID, |ui| {
            for k in [
                "tl-grown-axis",
                "tl-grown-now",
                "tl-grown-stripe",
                "tl-grown-fair",
            ] {
                ui.label(tr.t(k));
            }
            ui.label(RichText::new(tr.t("tl-grown-sources")).color(pal.weak));
        });
    }

    fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &storage_key(ID), &self.s);
    }

    fn load(&mut self, storage: &dyn eframe::Storage) {
        if let Some(s) = eframe::get_value::<State>(storage, &storage_key(ID)) {
            self.s = s;
            self.clamp();
        }
    }

    /// `#/timeline/<event>` opens that event's card and zooms around it;
    /// `#/timeline/whole` shows everything; a trailing `/fair` turns honest
    /// spacing fully on (`#/timeline/crust/fair`).
    fn set_param(&mut self, param: &str) {
        for p in param.split('/') {
            if p == "whole" {
                (self.s.u0, self.s.u1) = whole();
            } else if p == "fair" {
                self.s.fair = 1.0;
            } else if let Some(e) = tl::event(p) {
                self.select(e.id);
                (self.s.u0, self.s.u1) = Self::around(e);
                self.clamp();
            }
        }
        self.target = None;
    }
}

// --- Drawing -------------------------------------------------------------------

/// Lane colours: cosmos sky, stars amber, the Sun and Earth teal, life olive.
fn lane_colour(pal: &Palette, lane: Lane) -> Color32 {
    match lane {
        Lane::Cosmos => pal.series[3],
        Lane::Stars => pal.series[0],
        Lane::Earth => pal.series[1],
        Lane::Life => pal.series[5],
        Lane::People => pal.series[4],
    }
}

fn lane_key(lane: Lane) -> &'static str {
    match lane {
        Lane::Cosmos => "tl-lane-cosmos",
        Lane::Stars => "tl-lane-stars",
        Lane::Earth => "tl-lane-earth",
        Lane::Life => "tl-lane-life",
        Lane::People => "tl-lane-people",
    }
}

const STRIPE_H: f32 = 28.0;
/// Height of the names above the stripe.
const STRIPE_LABELS: f32 = 18.0;
/// Gap between the stripe and the track, where the zoom lines run.
const CONNECT_H: f32 = 30.0;
const LANE_H: f32 = 90.0;
/// Space under each lane's line, for up to four rows of range bars.
const LANE_FOOT: f32 = 22.0;
/// Rows of labels above a lane's line.
const LABEL_ROWS: usize = 3;
const AXIS_H: f32 = 64.0;
const LABEL: f32 = 13.0;

impl Timeline {
    /// The overview: the whole axis, coloured by how much happens, the view marked.
    fn stripe(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let w = ui.available_width();
        let (rect, resp) =
            ui.allocate_exact_size(vec2(w, STRIPE_LABELS + STRIPE_H), Sense::click_and_drag());
        let p = ui.painter_at(rect);
        let bar = Rect::from_min_size(rect.min + vec2(0.0, STRIPE_LABELS), vec2(w, STRIPE_H));
        let (a, b) = whole();
        let x_of = |u: f64| bar.left() + ((u - a) / (b - a)) as f32 * bar.width();
        let u_of = |x: f32| a + ((x - bar.left()) / bar.width()) as f64 * (b - a);

        // colour columns by event density
        let cols = (bar.width() / 2.0).ceil() as usize;
        let dens = tl::density(tl::EVENTS, a, b, cols, 0.9);
        for (i, d) in dens.iter().enumerate() {
            let x0 = bar.left() + i as f32 * 2.0;
            let c = if *d < 0.01 {
                lerp(pal.deep, pal.line, 0.35)
            } else {
                lerp(
                    pal.deep,
                    pal.ramp_at(*d as f32),
                    (0.35 + 0.65 * d.sqrt()) as f32,
                )
            };
            p.rect_filled(
                Rect::from_min_max(
                    pos2(x0, bar.top()),
                    pos2((x0 + 2.0).min(bar.right()), bar.bottom()),
                ),
                0.0,
                c,
            );
        }
        // events as small marks along the bottom
        for e in tl::EVENTS {
            let x = x_of(e.when.u());
            p.line_segment(
                [pos2(x, bar.bottom() - 7.0), pos2(x, bar.bottom())],
                Stroke::new(1.5, pal.strong),
            );
        }
        // now
        let xn = x_of(tl::u_now());
        p.line_segment(
            [pos2(xn, bar.top()), pos2(xn, bar.bottom())],
            Stroke::new(1.5, pal.strong),
        );
        // the current view
        let (v0, v1) = (
            x_of(self.s.u0).max(bar.left()),
            x_of(self.s.u1).min(bar.right()),
        );
        let (v0, v1) = if v1 - v0 < 4.0 {
            let c = (v0 + v1) * 0.5;
            (c - 2.0, c + 2.0)
        } else {
            (v0, v1)
        };
        p.rect_stroke(
            Rect::from_min_max(pos2(v0, bar.top() - 1.0), pos2(v1, bar.bottom() + 1.0)),
            2.0,
            Stroke::new(2.0, pal.accent),
            StrokeKind::Middle,
        );
        p.rect_stroke(bar, 3.0, Stroke::new(1.0, pal.line), StrokeKind::Inside);
        self.window = Some((v0, v1, bar.bottom() + 1.0));

        // ends and now, above the stripe
        let y = rect.top() + 1.0;
        draw::text(
            &p,
            pos2(bar.left(), y),
            Align2::LEFT_TOP,
            tr.t("tl-stripe-start"),
            12.0,
            pal.weak,
        );
        draw::text(
            &p,
            pos2(bar.right(), y),
            Align2::RIGHT_TOP,
            tr.t("tl-stripe-end"),
            12.0,
            pal.weak,
        );
        draw::text(
            &p,
            pos2(xn, y),
            Align2::CENTER_TOP,
            tr.t("tl-stripe-now"),
            12.0,
            pal.strong,
        );

        // click or drag: centre the view there
        if (resp.clicked() || resp.dragged())
            && let Some(pos) = resp.interact_pointer_pos()
        {
            let u = u_of(pos.x.clamp(bar.left(), bar.right()));
            let half = self.width() * 0.5;
            self.target = None;
            self.s.u0 = u - half;
            self.s.u1 = u + half;
            self.clamp();
        }
        resp.on_hover_cursor(egui::CursorIcon::PointingHand)
            .on_hover_text(tr.t("tl-stripe-hover"));
    }

    /// The zoomed view: lanes, events, ticks.
    fn track(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
        let w = ui.available_width();
        let h = LANE_H * Lane::ALL.len() as f32 + AXIS_H + 8.0;
        let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click_and_drag());
        // the zoom lines: from the view's window on the stripe to the corners
        // of the track, with a faint fill, like a magnifying glass
        if let Some((v0, v1, y)) = self.window {
            let lens = ui.painter();
            let (tl_, tr_) = (rect.left_top(), rect.right_top());
            lens.add(egui::Shape::convex_polygon(
                vec![pos2(v0, y), pos2(v1, y), tr_, tl_],
                pal.accent.gamma_multiply(0.07),
                Stroke::NONE,
            ));
            let line = Stroke::new(1.5, pal.accent.gamma_multiply(0.75));
            lens.line_segment([pos2(v0, y), tl_], line);
            lens.line_segment([pos2(v1, y), tr_], line);
        }
        let p = ui.painter_at(rect);
        draw::canvas(&p, pal, rect);
        let inner = rect.shrink2(vec2(12.0, 4.0));

        // wheel and pinch zoom, sideways scroll and drag pan
        if resp.hovered() {
            let (scroll, pinch, pointer) =
                ui.input(|i| (i.smooth_scroll_delta, i.zoom_delta(), i.pointer.hover_pos()));
            let at = pointer.map_or((self.s.u0 + self.s.u1) * 0.5, |pos| {
                self.s.u0 + ((pos.x - inner.left()) / inner.width()) as f64 * self.width()
            });
            let factor = (-(scroll.y as f64) * WHEEL_ZOOM).exp() / pinch as f64;
            if factor != 1.0 {
                self.target = None;
                self.zoom(at, factor);
            }
            if scroll.x != 0.0 {
                self.target = None;
                self.pan(-(scroll.x / inner.width()) as f64 * self.width());
            }
            // the page must not scroll while the wheel zooms the timeline
            ui.input_mut(|i| i.smooth_scroll_delta = egui::Vec2::ZERO);
        }
        if resp.dragged() {
            self.target = None;
            let dx = resp.drag_delta().x;
            self.pan(-(dx / inner.width()) as f64 * self.width());
        }

        let (u0, u1) = (self.s.u0, self.s.u1);
        // honest spacing blends each position towards a plain, linear time
        // scale over the same view (every year as wide)
        let fair = self.s.fair.clamp(0.0, 1.0);
        let (n0, n1) = (When::at(u0).from_now(), When::at(u1).from_now());
        let x_of = |u: f64| {
            let xw = inner.left() + ((u - u0) / (u1 - u0)) as f32 * inner.width();
            if fair <= 0.0 || n1 <= n0 {
                return xw;
            }
            let n = When::at(u).from_now();
            let xl = inner.left() + ((n - n0) / (n1 - n0)) as f32 * inner.width();
            xw + (xl - xw) * fair
        };
        let lanes_top = inner.top() + 4.0;
        let axis_y = lanes_top + LANE_H * Lane::ALL.len() as f32;

        // the future is a forecast: shade it
        let xn = x_of(tl::u_now());
        if xn < inner.right() {
            let fut = Rect::from_min_max(
                pos2(xn.max(rect.left()), rect.top()),
                pos2(rect.right(), axis_y),
            );
            p.rect_filled(fut, 0.0, lerp(pal.deep, pal.line, 0.22));
        }

        // lanes; their names are kept free of event labels
        let mut lane_names: Vec<Rect> = Vec::new();
        for (i, lane) in Lane::ALL.iter().enumerate() {
            let y = lanes_top + LANE_H * (i as f32 + 1.0) - LANE_FOOT;
            p.line_segment(
                [pos2(inner.left(), y), pos2(inner.right(), y)],
                Stroke::new(1.0, lerp(pal.deep, pal.line, 0.8)),
            );
            let c = lerp(pal.weak, lane_colour(pal, *lane), 0.4);
            let g = p.layout_no_wrap(tr.t(lane_key(*lane)), FontId::proportional(12.0), c);
            let at = pos2(inner.left(), lanes_top + LANE_H * i as f32 + 2.0);
            lane_names.push(Rect::from_min_size(at, g.size()).expand(2.0));
            p.galley(at, g, c);
        }

        // ticks
        p.line_segment(
            [pos2(inner.left(), axis_y), pos2(inner.right(), axis_y)],
            Stroke::new(1.0, pal.line),
        );
        // parts of the view counted "since the Big Bang", "ago" and "from now";
        // a part too narrow for its caption gets no ticks either
        let parts = [
            (u0, u1.min(tl::u_switch()), "tl-axis-after"),
            (u0.max(tl::u_switch()), u1.min(tl::u_now()), "tl-axis-ago"),
            (u0.max(tl::u_now()), u1, "tl-axis-ahead"),
        ];
        let wide = |from: f64, to: f64| to > from && x_of(to) - x_of(from) > 100.0;
        let mut drawn: Vec<(f32, When)> = Vec::new();
        for t in tl::ticks(u0, u1, inner.width() as f64, 118.0) {
            let part = match t.when {
                When::After(_) => parts[0],
                When::Ago(_) => parts[1],
                When::Ahead(_) => parts[2],
            };
            let is_now = matches!(t.when, When::Ago(0.0));
            if !is_now && !wide(part.0, part.1) {
                continue;
            }
            let x = x_of(t.u);
            // squashed views pile ticks up: keep only those with room
            if drawn.last().is_some_and(|(lx, _)| x - lx < 90.0) {
                continue;
            }
            drawn.push((x, t.when));
            p.line_segment(
                [pos2(x, lanes_top), pos2(x, axis_y)],
                Stroke::new(1.0, lerp(pal.deep, pal.line, 0.55)),
            );
            p.line_segment(
                [pos2(x, axis_y), pos2(x, axis_y + 5.0)],
                Stroke::new(1.0, pal.weak),
            );
            let label = match t.when {
                When::Ago(0.0) => tr.t("tl-stripe-now"),
                w => amount_text(tr, &tl::amount(value(w), 3)),
            };
            let anchor = if x - inner.left() < 40.0 {
                Align2::LEFT_TOP
            } else if inner.right() - x < 40.0 {
                Align2::RIGHT_TOP
            } else {
                Align2::CENTER_TOP
            };
            draw::text(&p, pos2(x, axis_y + 6.0), anchor, label, 12.0, pal.weak);
        }
        // ×10 hops between neighbouring power-of-ten ticks (only on the
        // stretched axis, where equal steps mean multiplying)
        if fair < 0.01 {
            let y0 = axis_y + 23.0;
            let c = lerp(pal.weak, pal.note, 0.6);
            for pair in drawn.windows(2) {
                let ((x1, a), (x2, b)) = (pair[0], pair[1]);
                let (Some(ka), Some(kb)) = (pow10(a), pow10(b)) else {
                    continue;
                };
                // same form, and for "since the Big Bang" the same unit
                // (ticks under a year count seconds, later ones years)
                let in_seconds = |w: When| matches!(w, When::After(y) if y < 1.0);
                if std::mem::discriminant(&a) != std::mem::discriminant(&b)
                    || in_seconds(a) != in_seconds(b)
                    || x2 - x1 < 60.0
                {
                    continue;
                }
                let d = kb - ka;
                if d == 0 {
                    continue;
                }
                let sign = if d > 0 { "×" } else { "÷" };
                let label = if d.abs() == 1 {
                    format!("{sign}10")
                } else {
                    format!("{sign}10{}", sims::fmt::superscript(d.abs()))
                };
                let mid = (x1 + x2) * 0.5;
                p.add(egui::epaint::QuadraticBezierShape::from_points_stroke(
                    [pos2(x1 + 6.0, y0), pos2(mid, y0 + 16.0), pos2(x2 - 6.0, y0)],
                    false,
                    Color32::TRANSPARENT,
                    Stroke::new(1.2, c),
                ));
                let g = p.layout_no_wrap(label, FontId::proportional(12.0), c);
                let r = Rect::from_center_size(pos2(mid, y0 + 8.0), g.size()).expand(2.0);
                p.rect_filled(r, 2.0, pal.deep);
                p.galley(r.min + vec2(2.0, 2.0), g, c);
            }
        }

        // which way the ticks count, per part of the view
        let cap_y = axis_y + 44.0;
        for (from, to, key) in parts {
            if to > from {
                let (xa, xb) = (x_of(from), x_of(to));
                if xb - xa > 100.0 {
                    draw::text(
                        &p,
                        pos2((xa + xb) * 0.5, cap_y),
                        Align2::CENTER_TOP,
                        tr.t(key),
                        12.0,
                        lerp(pal.weak, pal.deep, 0.25),
                    );
                }
            }
        }

        // now line
        if xn >= inner.left() && xn <= inner.right() {
            draw::dashed(&p, pos2(xn, lanes_top), pos2(xn, axis_y), 1.3, pal.strong);
        }

        // events: dots, then labels placed so they don't overlap
        let font = FontId::proportional(LABEL);
        let mut placed: Vec<Rect> = lane_names;
        let mut hits: Vec<(Rect, Pos2, &'static str)> = Vec::new();
        // ranges of estimates: dashed bars just under the lane line, one row
        // per ranged event of the lane so they never run into each other
        let mut rows_used = [0usize; Lane::ALL.len()];
        for e in tl::EVENTS {
            let Some((a, b)) = e.range else { continue };
            let (ua, ub) = (a.u().max(u0), b.u().min(u1));
            if ub <= ua {
                continue;
            }
            // rows are handed out only among the bars in view
            let li = Lane::ALL.iter().position(|l| *l == e.lane).unwrap_or(0);
            let row = rows_used[li];
            rows_used[li] += 1;
            let y = lanes_top + LANE_H * (li as f32 + 1.0) - LANE_FOOT
                + 5.0
                + (row.min(3)) as f32 * 4.0;
            let c = lerp(pal.deep, lane_colour(pal, e.lane), 0.75);
            let (xa, xb) = (x_of(ua), x_of(ub));
            draw::dashed(&p, pos2(xa, y), pos2(xb, y), 2.0, c);
            for (u, x) in [(a.u(), xa), (b.u(), xb)] {
                if u >= u0 && u <= u1 {
                    p.line_segment([pos2(x, y - 3.0), pos2(x, y + 3.0)], Stroke::new(1.5, c));
                }
            }
        }

        let mut order: Vec<&Event> = tl::EVENTS.iter().collect();
        order.sort_by_key(|e| e.importance);
        for e in order {
            let u = e.when.u();
            if u < u0 || u > u1 {
                continue;
            }
            let li = Lane::ALL.iter().position(|l| *l == e.lane).unwrap_or(0);
            let base = lanes_top + LANE_H * (li as f32 + 1.0) - LANE_FOOT;
            let x = x_of(u);
            let c = lane_colour(pal, e.lane);
            let r = if e.importance == 1 { 5.5 } else { 4.5 };
            let dot = pos2(x, base);
            if e.uncertain {
                p.circle_filled(dot, r, pal.deep);
                p.circle_stroke(dot, r - 0.5, Stroke::new(2.0, c));
            } else {
                p.circle_filled(dot, r, c);
            }
            if self.s.selected.as_deref() == Some(e.id) {
                p.circle_stroke(dot, r + 4.0, Stroke::new(1.5, pal.strong));
            }
            let g = p.layout_no_wrap(tr.t(&format!("tl-ev-{}", e.id)), font.clone(), c);
            let size = g.size();
            let mut label_rect = None;
            for row in 0..LABEL_ROWS {
                let top = base - 10.0 - (row as f32 + 1.0) * (size.y + 2.0);
                let left = (x - size.x * 0.5).clamp(inner.left(), inner.right() - size.x);
                let cand = Rect::from_min_size(pos2(left, top), size).expand2(vec2(3.0, 0.0));
                if placed.iter().all(|q| !q.intersects(cand))
                    && cand.top() >= lanes_top + LANE_H * li as f32 - 2.0
                {
                    label_rect = Some(cand);
                    break;
                }
            }
            if let Some(lr) = label_rect {
                placed.push(lr);
                p.galley(lr.min + vec2(3.0, 0.0), g, c);
                if (lr.center().x - x).abs() > 2.0 || lr.bottom() < base - 12.0 {
                    p.line_segment(
                        [pos2(x, lr.bottom()), pos2(x, base - r - 1.0)],
                        Stroke::new(1.0, lerp(pal.deep, c, 0.5)),
                    );
                }
                hits.push((lr, dot, e.id));
            } else {
                hits.push((Rect::NOTHING, dot, e.id));
            }
        }

        // how much time fits in one finger's width under the mouse
        if fair < 0.01
            && let Some(pos) = resp.hover_pos()
            && pos.x > inner.left() + 30.0
            && pos.x < inner.right() - 30.0
            && pos.y < axis_y
        {
            let u_at = |x: f32| u0 + ((x - inner.left()) / inner.width()) as f64 * (u1 - u0);
            let span =
                tl::years_between(When::at(u_at(pos.x - 30.0)), When::at(u_at(pos.x + 30.0)));
            let y = (pos.y - 22.0).max(lanes_top + 14.0);
            let c = pal.note;
            let s = Stroke::new(1.5, c);
            p.line_segment([pos2(pos.x - 30.0, y), pos2(pos.x + 30.0, y)], s);
            for x in [pos.x - 30.0, pos.x + 30.0] {
                p.line_segment([pos2(x, y - 4.0), pos2(x, y + 4.0)], s);
            }
            let text = tr.ta(
                "tl-finger",
                &args!["a" => amount_text(tr, &tl::amount(span, 2))],
            );
            let g = p.layout_no_wrap(text, FontId::proportional(12.0), c);
            let r = Rect::from_center_size(pos2(pos.x, y - 12.0), g.size()).expand(2.0);
            p.rect_filled(r, 2.0, pal.deep);
            p.galley(r.min + vec2(2.0, 2.0), g, c);
        }

        // hover and click on events
        let pointer = resp.hover_pos();
        let hit = pointer.and_then(|pos| {
            hits.iter()
                .filter(|(lr, dot, _)| lr.contains(pos) || dot.distance(pos) < 10.0)
                .min_by(|a, b| a.1.distance(pos).total_cmp(&b.1.distance(pos)))
                .map(|(_, _, id)| *id)
        });
        let resp = if let Some(id) = hit {
            let e = tl::event(id).expect("hit ids come from EVENTS");
            resp.on_hover_cursor(egui::CursorIcon::PointingHand)
                .on_hover_text_at_pointer(format!(
                    "{}\n{}",
                    tr.t(&format!("tl-ev-{id}")),
                    when_text(tr, e.when)
                ))
        } else {
            resp
        };
        if resp.clicked() {
            match hit {
                Some(id) => self.select(id),
                None => self.s.selected = None,
            }
        }
        if resp.double_clicked()
            && hit.is_none()
            && let Some(pos) = resp.interact_pointer_pos()
        {
            let at = u0 + ((pos.x - inner.left()) / inner.width()) as f64 * (u1 - u0);
            self.zoom(at, 0.4);
        }
    }

    /// The card of the selected event.
    fn card(&mut self, ui: &mut egui::Ui, tr: &I18n, pal: &Palette, e: &Event, calm: bool) {
        kit::inset(pal).show(ui, |ui| {
            ui.set_width(ui.available_width());
            let c = lane_colour(pal, e.lane);
            let key = format!("tl-ev-{}", e.id);
            ui.label(
                RichText::new(tr.t(&key))
                    .size(kit::SECTION_TITLE - 3.0)
                    .color(c),
            );
            ui.label(RichText::new(when_text(tr, e.when)).color(pal.weak));
            if let Some((a, b)) = e.range {
                ui.label(RichText::new(range_text(tr, a, b)).small().color(pal.weak));
            }
            ui.add_space(4.0);
            ui.label(tr.attr(&key, "text"));
            if let Some(slug) = e.wiki {
                ui.add(
                    egui::Hyperlink::from_label_and_url(
                        format!("📖 {} ↗", tr.t("tl-wiki")),
                        format!("https://en.wikipedia.org/wiki/{slug}"),
                    )
                    .open_in_new_tab(true),
                );
            }
            if e.uncertain {
                ui.add_space(2.0);
                ui.label(
                    RichText::new(format!("○ {}", tr.t("tl-uncertain")))
                        .small()
                        .color(pal.weak),
                );
            }
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                if ui.button(format!("🔍 {}", tr.t("tl-zoom-here"))).clicked() {
                    self.go_to(Self::around(e), calm);
                }
                if ui.button(format!("✖ {}", tr.t("tl-close"))).clicked() {
                    self.s.selected = None;
                }
            });
        });
    }
}

fn value(w: When) -> f64 {
    match w {
        When::After(v) | When::Ago(v) | When::Ahead(v) => v,
    }
}

/// The exponent when a tick value is a whole power of ten.
fn pow10(w: When) -> Option<i32> {
    let v = match w {
        When::After(y) => y * tl::SEC_PER_YR, // ticks there are in seconds first
        When::Ago(y) | When::Ahead(y) => y,
    };
    let v = if matches!(w, When::After(y) if y >= 1.0) {
        value(w)
    } else {
        v
    };
    if v <= 0.0 {
        return None;
    }
    let k = v.log10().round();
    ((v / 10f64.powf(k) - 1.0).abs() < 1e-6).then_some(k as i32)
}

/// "If all of time were a 1 km walk": a bar from the Big Bang to now with
/// the Earth and the dinosaurs marked, and where the rest would be.
fn walk(ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
    ui.label(tr.t("tl-walk-intro"));
    ui.add_space(6.0);
    let w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(w, 74.0), Sense::hover());
    let p = ui.painter_at(rect);
    draw::canvas(&p, pal, rect);
    let bar = Rect::from_min_max(
        pos2(rect.left() + 12.0, rect.top() + 30.0),
        pos2(rect.right() - 12.0, rect.top() + 40.0),
    );
    p.rect_filled(bar, 2.0, lerp(pal.deep, pal.line, 0.9));
    let x_at = |years_ago: f64| {
        bar.right() - (tl::walk_before_end(years_ago) / tl::WALK_M) as f32 * bar.width()
    };
    draw::text(
        &p,
        pos2(bar.left(), bar.bottom() + 6.0),
        Align2::LEFT_TOP,
        tr.t("tl-stripe-start"),
        12.0,
        pal.weak,
    );
    draw::text(
        &p,
        pos2(bar.right(), bar.bottom() + 6.0),
        Align2::RIGHT_TOP,
        tr.t("tl-stripe-now"),
        12.0,
        pal.strong,
    );
    for (key, years, c) in [
        ("tl-walk-mark-earth", 4.54e9, pal.series[1]),
        ("tl-walk-mark-dinosaurs", 66.0e6, pal.series[5]),
    ] {
        let x = x_at(years);
        p.line_segment(
            [pos2(x, bar.top() - 4.0), pos2(x, bar.bottom() + 4.0)],
            Stroke::new(2.0, c),
        );
        draw::text(
            &p,
            pos2(x, bar.top() - 6.0),
            Align2::RIGHT_BOTTOM,
            tr.t(key),
            12.0,
            c,
        );
    }
    ui.label(
        RichText::new(format!("↑ {}", tr.t("tl-walk-too-thin")))
            .small()
            .color(pal.note),
    );
    ui.add_space(6.0);
    for (key, years) in tl::WALK {
        let d = tr.num(&tl::walk_length(tl::walk_before_end(*years)));
        ui.label(tr.ta(&format!("tl-walk-{key}"), &args!["d" => d]));
    }
    ui.add_space(4.0);
    let reign = tr.num(&tl::walk_length(tl::walk_span(tl::DINOSAUR_REIGN_YR)));
    let us = tr.num(&tl::walk_length(tl::walk_span(tl::SAPIENS_SO_FAR_YR)));
    let times = (tl::DINOSAUR_REIGN_YR / tl::SAPIENS_SO_FAR_YR).round();
    ui.label(
        RichText::new(tr.ta(
            "tl-walk-reign",
            &args!["d" => reign, "s" => us, "x" => times],
        ))
        .color(pal.note),
    );
}

/// "4.54 billion years", "380 000 years", "10⁻³² s", in the visitor's language.
pub fn amount_text(tr: &I18n, a: &Amount) -> String {
    let key = match a.unit {
        Unit::Second => "tl-u-s",
        Unit::Minute => "tl-u-min",
        Unit::Hour => "tl-u-h",
        Unit::Day => "tl-u-d",
        Unit::Year => "tl-u-yr",
        Unit::MillionYears => "tl-u-myr",
        Unit::BillionYears => "tl-u-byr",
        Unit::TrillionYears => "tl-u-tyr",
    };
    tr.ta(key, &args!["v" => tr.num(&a.value), "n" => a.count])
}

/// "about 4.54 billion years ago", "380 000 years after the Big Bang", …
fn when_text(tr: &I18n, w: When) -> String {
    match w {
        When::Ago(0.0) => tr.t("tl-when-now"),
        When::After(t) => tr.ta(
            "tl-when-after",
            &args!["a" => amount_text(tr, &tl::amount(t, 3))],
        ),
        When::Ago(a) => tr.ta(
            "tl-when-ago",
            &args!["a" => amount_text(tr, &tl::amount(a, 3))],
        ),
        When::Ahead(y) => tr.ta(
            "tl-when-ahead",
            &args!["a" => amount_text(tr, &tl::amount(y, 3))],
        ),
    }
}

/// "Estimates: 4.53 billion years to 4.43 billion years ago", for a range in one form.
fn range_text(tr: &I18n, a: When, b: When) -> String {
    let amt = |w: When| amount_text(tr, &tl::amount(value(w), 3));
    let key = match (a, b) {
        (When::After(_), When::After(_)) => "tl-range-after",
        (When::Ago(_), When::Ago(_)) => "tl-range-ago",
        (When::Ahead(_), When::Ahead(_)) => "tl-range-ahead",
        _ => {
            return tr.ta(
                "tl-range-mixed",
                &args!["a" => when_text(tr, a), "b" => when_text(tr, b)],
            );
        }
    };
    tr.ta(key, &args!["a" => amt(a), "b" => amt(b)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn every_event_has_a_title_and_a_card_in_every_language() {
        for lang in Lang::ALL {
            let tr = I18n::new(lang);
            for e in tl::EVENTS {
                let key = format!("tl-ev-{}", e.id);
                assert!(tr.has(&key, None), "{}: {key}", lang.code());
                assert!(tr.has(&key, Some("text")), "{}: {key}.text", lang.code());
            }
        }
    }

    #[test]
    fn deep_links_select_and_zoom() {
        let mut t = Timeline::default();
        t.set_param("earth");
        assert_eq!(t.s.selected.as_deref(), Some("earth"));
        let u = When::Ago(4.54e9).u();
        assert!(t.s.u0 < u && u < t.s.u1);
        t.set_param("whole");
        assert_eq!((t.s.u0, t.s.u1), whole());
        t.set_param("moon/fair");
        assert_eq!(t.s.selected.as_deref(), Some("moon"));
        assert_eq!(t.s.fair, 1.0);
    }

    #[test]
    fn zoom_keeps_the_point_and_stays_in_bounds() {
        let mut t = Timeline::default();
        let at = When::Ago(5_000.0).u();
        for _ in 0..200 {
            t.zoom(at, 0.8);
        }
        assert!((t.width() - MIN_WIDTH).abs() < 1e-12);
        assert!(t.s.u0 <= at && at <= t.s.u1);
        for _ in 0..400 {
            t.zoom(at, 1.25);
        }
        let (a, b) = whole();
        assert!((t.s.u0 - a).abs() < 1e-9 && (t.s.u1 - b).abs() < 1e-9);
    }

    #[test]
    fn sentences_read_naturally() {
        let tr = I18n::new(Lang::En);
        assert_eq!(
            when_text(&tr, When::Ago(4.54e9)),
            "about 4.54 billion years ago"
        );
        assert_eq!(
            when_text(&tr, When::After(380_000.0)),
            "about 380\u{2009}000 years after the Big Bang"
        );
        let sk = I18n::new(Lang::Sk);
        assert_eq!(when_text(&sk, When::Ahead(5e9)), "asi o 5 mld. r.");
        assert_eq!(when_text(&sk, When::Ago(4.54e9)), "asi pred 4,54 mld. r.");
        assert_eq!(
            range_text(&tr, When::Ago(4.53e9), When::Ago(4.43e9)),
            "Estimates: from 4.53 billion years to 4.43 billion years ago"
        );
        assert_eq!(
            range_text(&sk, When::Ago(4.53e9), When::Ago(4.43e9)),
            "Odhady: pred 4,53 mld. r. až 4,43 mld. r."
        );
    }
}
