//! Small shared widgets, kept plain (D-018): grouped boxes, callouts, fold-outs.

use egui::{Color32, CornerRadius, Frame, Margin, Response, RichText, Stroke};

use crate::i18n::I18n;
use crate::theme::Palette;

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
