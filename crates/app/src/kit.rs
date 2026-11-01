//! Shared widgets, so every page looks the same (docs/DESIGN.md, D-038):
//! sections, choice bars, insets, callouts, fold-outs.

use egui::{Color32, CornerRadius, Frame, Margin, Response, RichText, Stroke, vec2};

use crate::i18n::I18n;
use crate::theme::{Palette, lerp};

/// A small outlined label.
pub fn pill(ui: &mut egui::Ui, _pal: &Palette, text: impl Into<String>, color: Color32) {
    Frame::new()
        .stroke(Stroke::new(1.0, color))
        .corner_radius(CornerRadius::same(3))
        .inner_margin(Margin::symmetric(5, 0))
        .show(ui, |ui| {
            ui.label(RichText::new(text.into()).small().color(color));
        });
}

/// An action button. The colour argument is kept so sims can say which button
/// belongs to which part of a drawing, but the button itself stays plain.
pub fn bright_button(
    ui: &mut egui::Ui,
    _pal: &Palette,
    text: RichText,
    _color: Color32,
) -> Response {
    ui.button(text)
}

/// Box around grouped content inside a page.
pub fn card(pal: &Palette) -> Frame {
    Frame::new()
        .fill(pal.card)
        .stroke(Stroke::new(1.0, pal.line))
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::same(10))
}

/// One part of a page, framed so it stands apart from the next (DESIGN.md
/// "Sections"): a raised card with a heading. Pages with several parts use one
/// per part; canvases and insets sit inside.
pub fn section(ui: &mut egui::Ui, pal: &Palette, title: &str, add: impl FnOnce(&mut egui::Ui)) {
    Frame::new()
        .fill(pal.card)
        .stroke(Stroke::new(1.0, pal.line))
        .corner_radius(CornerRadius::same(6))
        .inner_margin(Margin::symmetric(16, 12))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(title).size(SECTION_TITLE).color(pal.strong));
            ui.add_space(2.0);
            ui.separator();
            ui.add_space(4.0);
            add(ui);
        });
    ui.add_space(SECTION_GAP);
}

/// Heading size of a [`section`].
pub const SECTION_TITLE: f32 = 21.0;
/// Space between sections.
pub const SECTION_GAP: f32 = 16.0;

/// A box inside a section (guess box, small charts), a step darker than the section.
pub fn inset(pal: &Palette) -> Frame {
    Frame::new()
        .fill(pal.bg)
        .stroke(Stroke::new(1.0, pal.line))
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::same(10))
}

/// A short highlighted statement: the rule of a page, the lesson of a section.
pub fn takeaway(ui: &mut egui::Ui, pal: &Palette, text: impl Into<String>) {
    Frame::new()
        .fill(lerp(pal.bg, pal.note, 0.10))
        .stroke(Stroke::new(1.0, lerp(pal.bg, pal.note, 0.45)))
        .corner_radius(CornerRadius::same(4))
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(RichText::new(text.into()).color(pal.note));
        });
}

/// One option of a [`choice_bar`].
pub struct Choice<T> {
    pub value: T,
    pub label: String,
    /// `Some(reason)` greys the option out and explains why on hover.
    pub disabled: Option<String>,
}

/// The main switches of a page (examples, pictures, modes): a caption and a row
/// of big buttons, the chosen one filled with the accent (DESIGN.md "Choice bar").
/// Returns true when the choice changed.
pub fn choice_bar<T: PartialEq + Copy>(
    ui: &mut egui::Ui,
    pal: &Palette,
    caption: &str,
    choices: &[Choice<T>],
    current: &mut T,
) -> bool {
    let mut changed = false;
    ui.label(RichText::new(caption).size(CAPTION).color(pal.weak));
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
        for c in choices {
            let on = *current == c.value;
            let text = RichText::new(&c.label).size(CHOICE_TEXT).color(if on {
                pal.deep
            } else {
                pal.text
            });
            let button = egui::Button::new(text)
                .fill(if on {
                    pal.accent
                } else {
                    lerp(pal.card, pal.line, 0.7)
                })
                .stroke(Stroke::new(1.0, if on { pal.accent } else { pal.line }))
                .corner_radius(CornerRadius::same(5))
                .min_size(vec2(0.0, CHOICE_HEIGHT));
            let r = ui.add_enabled(c.disabled.is_none(), button);
            let r = match &c.disabled {
                Some(why) => r.on_disabled_hover_text(why),
                None => r,
            };
            if r.clicked() && !on {
                *current = c.value;
                changed = true;
            }
        }
    });
    changed
}

/// Caption above a choice bar or a group of inputs.
pub const CAPTION: f32 = 13.0;
pub const CHOICE_TEXT: f32 = 16.5;
pub const CHOICE_HEIGHT: f32 = 32.0;

/// A box holding a group of controls, with an optional title.
pub fn panel(
    ui: &mut egui::Ui,
    pal: &Palette,
    title: Option<&str>,
    add: impl FnOnce(&mut egui::Ui),
) {
    card(pal).show(ui, |ui| {
        if let Some(t) = title {
            ui.strong(t);
        }
        add(ui);
    });
}

/// The "model alert" callout (🐄 spherical cow): where we simplify on purpose.
pub fn model_alert(ui: &mut egui::Ui, pal: &Palette, tr: &I18n, text: &str) {
    card(pal).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label("🐄");
            ui.strong(tr.t("model-alert"));
            ui.label(text);
        });
    });
}

/// Fold-out with formulas, assumptions and sources.
pub fn grown_ups(
    ui: &mut egui::Ui,
    _pal: &Palette,
    tr: &I18n,
    id: &str,
    add: impl FnOnce(&mut egui::Ui),
) {
    egui::CollapsingHeader::new(format!("🔍 {}", tr.t("grown-ups")))
        .id_salt(id)
        .default_open(false)
        .show(ui, add);
}

// ---------------------------------------------------------------- tries (D-041)

/// The visitor's own tries on a "what changes what" page: each setting of the
/// slider they settle on leaves a dot, so the shape appears from their own
/// experiments (WHAT-CHANGES-WHAT.md "rhythm", DESIGN.md "Tries plot").
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Tries(Vec<(f64, f64)>);

impl Tries {
    /// At most this many dots; the oldest goes first.
    pub const MAX: usize = 40;

    /// Add a try; a try at the same `x` replaces the old one.
    pub fn add(&mut self, x: f64, y: f64) {
        self.0.retain(|t| (t.0 - x).abs() > 1e-9);
        self.0.push((x, y));
        if self.0.len() > Self::MAX {
            self.0.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn points(&self) -> &[(f64, f64)] {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// A form-style slider for the one control of a scene: the quantity's name
/// before it, a fixed unit after it (DESIGN.md "Inputs"), decimal comma in
/// Slovak. Returns true when the visitor settled on a value (let go of the
/// handle, clicked, or used the keys): that is the moment a try is recorded.
pub fn try_slider(
    ui: &mut egui::Ui,
    tr: &I18n,
    name: &str,
    value: &mut f64,
    range: std::ops::RangeInclusive<f64>,
    step: f64,
    unit: &str,
) -> bool {
    let mut settled = false;
    ui.horizontal_wrapped(|ui| {
        ui.add(egui::Label::new(name).extend());
        ui.spacing_mut().slider_width = (ui.available_width() - 90.0).clamp(120.0, 320.0);
        let decimals = if step.fract() == 0.0 { 0 } else { 1 };
        let r = ui.add(
            egui::Slider::new(value, range)
                .step_by(step)
                .custom_formatter(|v, _| tr.num(&format!("{v:.decimals$}")))
                .custom_parser(|s| s.replace(',', ".").trim().parse().ok())
                .suffix(if unit.is_empty() {
                    String::new()
                } else {
                    format!(" {unit}")
                }),
        );
        settled = r.drag_stopped() || (r.changed() && !r.dragged());
    });
    settled
}

/// A revealed shape: the function and the range of `x` to draw it over.
pub type Shape<'a> = (&'a dyn Fn(f64) -> f64, f64, f64);

/// What a [`tries_plot`] shows.
pub struct TriesPlot<'a> {
    pub x: crate::draw::Axis<'a>,
    pub y: crate::draw::Axis<'a>,
    pub tries: &'a Tries,
    /// The slider's current setting, ringed in white.
    pub now: Option<(f64, f64)>,
    /// The shape the tries revealed, drawn dashed through them over `from..=to`
    /// once it is known (the page decides when: enough tries, all on it).
    pub shape: Option<Shape<'a>>,
    /// Legend rows: the tries, and the shape when it is drawn.
    pub tries_label: String,
    pub shape_label: String,
}

/// The tries plot (DESIGN.md "Tries plot"): a grid, one amber dot per try, the
/// current setting ringed in white, and the revealed shape as a dashed teal line.
pub fn tries_plot(p: &egui::Painter, pal: &Palette, r: egui::Rect, t: &TriesPlot) {
    use crate::draw;
    draw::canvas(p, pal, r);
    let (to, plot) = draw::grid_axes(p, pal, r.shrink(4.0), &t.x, &t.y);
    let inside =
        |x: f64, y: f64| x >= -1e-9 && x <= t.x.max + 1e-9 && y >= -1e-9 && y <= t.y.max + 1e-9;
    if let Some((f, from, upto)) = t.shape {
        let n = 80;
        let pts: Vec<egui::Pos2> = (0..=n)
            .map(|i| from + (upto - from) * i as f64 / n as f64)
            .map(|x| (x, f(x)))
            .filter(|&(x, y)| inside(x, y))
            .map(|(x, y)| to(x, y))
            .collect();
        if pts.len() > 1 {
            p.extend(egui::Shape::dashed_line(
                &pts,
                Stroke::new(2.0, pal.series[1]),
                6.0,
                4.0,
            ));
        }
    }
    for &(x, y) in t.tries.points() {
        if inside(x, y) {
            p.circle_filled(to(x, y), 5.0, pal.accent);
        }
    }
    if let Some((x, y)) = t.now.filter(|&(x, y)| inside(x, y)) {
        let c = to(x, y);
        draw::dashed(p, to(x, 0.0), c, 1.3, pal.strong);
        draw::dashed(p, to(0.0, y), c, 1.3, pal.strong);
        p.circle_stroke(c, 7.5, Stroke::new(2.0, pal.strong));
    }
    let mut rows = vec![(pal.accent, None, t.tries_label.clone())];
    if t.shape.is_some() {
        rows.push((pal.series[1], Some(true), t.shape_label.clone()));
    }
    // top left (DESIGN.md "Legend"), unless the dots are there: then top right
    let upper_left = t
        .tries
        .points()
        .iter()
        .filter(|&&(x, y)| x < t.x.max * 0.4 && y > t.y.max * 0.6)
        .count();
    let at = if upper_left == 0 {
        plot.left_top() + vec2(10.0, 10.0)
    } else {
        let w = rows
            .iter()
            .map(|r| {
                p.layout_no_wrap(r.2.clone(), egui::FontId::proportional(12.0), r.0)
                    .size()
                    .x
            })
            .fold(0.0_f32, f32::max);
        plot.right_top() + vec2(-(w + 34.0), 10.0)
    };
    draw::legend(p, at, &rows);
}
