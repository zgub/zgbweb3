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
