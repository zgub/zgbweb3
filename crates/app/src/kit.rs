//! Small shared widgets: pills, icon badges, cards, callouts, the logo.

use egui::{
    Align2, Color32, CornerRadius, Frame, Margin, Response, RichText, Sense, Stroke, StrokeKind,
    vec2,
};

use crate::i18n::I18n;
use crate::theme::{Palette, bold_font, heading_font};

/// A small rounded outlined label.
pub fn pill(ui: &mut egui::Ui, _pal: &Palette, text: impl Into<String>, color: Color32) {
    Frame::new()
        .stroke(Stroke::new(1.0, color))
        .corner_radius(CornerRadius::same(9))
        .inner_margin(Margin::symmetric(7, 1))
        .show(ui, |ui| {
            ui.label(RichText::new(text.into()).small().color(color));
        });
}

/// An emoji/icon on a solid colour disc.
pub fn icon_badge(ui: &mut egui::Ui, pal: &Palette, icon: &str, color: Color32, size: f32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let p = ui.painter();
    p.circle_filled(rect.center(), size / 2.0, color);
    p.text(
        rect.center(),
        Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(size * 0.52),
        pal.on_accent,
    );
}

/// A solid, colourful action button (dark text on a bright fill).
pub fn bright_button(ui: &mut egui::Ui, pal: &Palette, text: RichText, color: Color32) -> Response {
    ui.add(egui::Button::new(text.color(pal.on_accent)).fill(color))
        .on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// Card background used on the home page and for grouped controls.
pub fn card(pal: &Palette) -> Frame {
    Frame::new()
        .fill(pal.card)
        .stroke(Stroke::new(1.0, pal.line))
        .corner_radius(CornerRadius::same(16))
        .inner_margin(Margin::same(14))
}

/// A card holding a group of controls, with an optional bold title.
pub fn panel(
    ui: &mut egui::Ui,
    pal: &Palette,
    title: Option<&str>,
    add: impl FnOnce(&mut egui::Ui),
) {
    card(pal).inner_margin(Margin::same(12)).show(ui, |ui| {
        if let Some(t) = title {
            ui.label(RichText::new(t).font(bold_font(16.5)));
        }
        add(ui);
    });
}

/// Paint a hover outline on a clickable card and return the click response.
pub fn clickable(ui: &mut egui::Ui, rect: egui::Rect, id: egui::Id, color: Color32) -> Response {
    let resp = ui
        .interact(rect, id, Sense::click())
        .on_hover_cursor(egui::CursorIcon::PointingHand);
    if resp.hovered() {
        ui.painter()
            .rect_stroke(rect, 16, Stroke::new(2.0, color), StrokeKind::Inside);
    }
    resp
}

/// "Where next?"-style button: tinted with the target's colour.
pub fn hook_button(ui: &mut egui::Ui, pal: &Palette, text: &str, color: Color32) -> Response {
    ui.add(
        egui::Button::new(
            RichText::new(format!("{text}  →"))
                .font(bold_font(15.5))
                .color(pal.on_accent),
        )
        .fill(color)
        .corner_radius(CornerRadius::same(18)),
    )
    .on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// The "model alert" callout (🐄 spherical cow): where we simplify on purpose.
pub fn model_alert(ui: &mut egui::Ui, pal: &Palette, tr: &I18n, text: &str) {
    Frame::new()
        .fill(pal.tint(pal.scale, 0.12))
        .stroke(Stroke::new(1.0, pal.tint(pal.scale, 0.45)))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::symmetric(12, 8))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("🐄").size(20.0));
                ui.label(
                    RichText::new(tr.t("model-alert"))
                        .font(bold_font(15.5))
                        .color(pal.scale),
                );
                ui.label(text);
            });
        });
}

/// Fold-out with formulas and assumptions.
pub fn grown_ups(
    ui: &mut egui::Ui,
    pal: &Palette,
    tr: &I18n,
    id: &str,
    add: impl FnOnce(&mut egui::Ui),
) {
    Frame::new()
        .fill(pal.card)
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            egui::CollapsingHeader::new(
                RichText::new(format!("🔍  {}", tr.t("grown-ups"))).color(pal.weak),
            )
            .id_salt(id)
            .default_open(false)
            .show(ui, add);
        });
}

/// Logo: three squares growing ×10-ish, then "zgb".
pub fn logo(ui: &mut egui::Ui, pal: &Palette) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(96.0, 30.0), Sense::click());
    let p = ui.painter();
    let base = rect.left_bottom() + vec2(2.0, -6.0);
    let mut x = base.x;
    for (size, color) in [(6.0, pal.chance), (10.0, pal.physics), (16.0, pal.scale)] {
        let r =
            egui::Rect::from_min_max(egui::pos2(x, base.y - size), egui::pos2(x + size, base.y));
        p.rect_filled(r, 2.5, color);
        x += size + 3.0;
    }
    p.text(
        egui::pos2(x + 4.0, rect.center().y - 1.0),
        Align2::LEFT_CENTER,
        "zgb",
        heading_font(24.0),
        pal.text,
    );
    resp.on_hover_cursor(egui::CursorIcon::PointingHand)
}
