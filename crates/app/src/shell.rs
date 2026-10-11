//! Page chrome: top bar, topic menu (right), footer, storage-consent banner.
//!
//! These functions never mutate app state directly beyond the settings they are
//! handed; navigation and other effects come back as [`Action`]s.

use egui::{Margin, RichText, Stroke};

use crate::i18n::{I18n, Lang};
use crate::kit;
use crate::pages::Page;
use crate::settings::{Settings, StorageConsent, ThemeChoice, ZOOM_RANGE};
use crate::theme::{Palette, bold_font};
use crate::topics::{Group, Topic, TopicMeta};

pub enum Action {
    Go(Page),
    ForgetEverything,
}

/// Background frame for the top bar, menu and footer.
pub fn chrome_frame(pal: &Palette, margin: Margin) -> egui::Frame {
    egui::Frame::new().fill(pal.chrome).inner_margin(margin)
}

pub struct Bar<'a> {
    pub tr: &'a I18n,
    pub pal: &'static Palette,
    pub settings: &'a mut Settings,
    pub consent: &'a mut StorageConsent,
    pub title: Option<String>,
    pub actions: &'a mut Vec<Action>,
}

pub fn top_bar(ui: &mut egui::Ui, bar: Bar) {
    let Bar {
        tr,
        pal,
        settings,
        consent,
        title,
        actions,
    } = bar;
    ui.horizontal(|ui| {
        if kit::logo(ui, pal).on_hover_text(tr.t("nav-home")).clicked() {
            actions.push(Action::Go(Page::Home));
        }
        if let Some(t) = title {
            ui.label(RichText::new("›").color(pal.weak).size(20.0));
            ui.label(RichText::new(t).font(bold_font(16.5)));
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.toggle_value(
                &mut settings.menu_open,
                format!("☰  {}", tr.t("nav-topics")),
            )
            .on_hover_text(tr.t("nav-topics-hover"));
            ui.menu_button("⚙", |ui| {
                settings_menu(ui, tr, settings, consent, actions)
            })
            .response
            .on_hover_text(tr.t("nav-settings"));
            language_switch(ui, tr, pal, settings);
        });
    });
}

/// Compact EN | SK switch, always visible. Picking one fixes the language;
/// "automatic" lives in the settings menu.
fn language_switch(ui: &mut egui::Ui, tr: &I18n, pal: &Palette, settings: &mut Settings) {
    // right-to-left layout: add in reverse
    for lang in Lang::ALL.iter().rev() {
        let current = tr.lang() == *lang;
        let text = RichText::new(lang.short()).font(bold_font(14.0));
        let btn = egui::Button::new(if current {
            text.color(pal.on_accent)
        } else {
            text.color(pal.weak)
        })
        .fill(if current { pal.scale } else { pal.chrome })
        .corner_radius(8);
        if ui.add(btn).on_hover_text(lang.native_name()).clicked() {
            settings.language = Some(*lang);
        }
    }
}

fn settings_menu(
    ui: &mut egui::Ui,
    tr: &I18n,
    settings: &mut Settings,
    consent: &mut StorageConsent,
    actions: &mut Vec<Action>,
) {
    ui.set_min_width(260.0);
    let title = |s: String| RichText::new(s).font(bold_font(15.5));

    ui.label(title(tr.t("settings-look")));
    for t in ThemeChoice::ALL {
        ui.radio_value(&mut settings.theme, t, tr.t(t.key()));
    }
    ui.horizontal(|ui| {
        ui.label(tr.t("settings-size"));
        ui.add(
            egui::Slider::new(&mut settings.zoom, ZOOM_RANGE)
                .step_by(0.05)
                .show_value(false),
        );
        if ui.small_button(tr.t("settings-size-reset")).clicked() {
            settings.zoom = 1.0;
        }
    });
    ui.checkbox(&mut settings.calm_motion, tr.t("settings-calm"));

    ui.separator();
    ui.label(title(tr.t("settings-language")));
    ui.radio_value(&mut settings.language, None, tr.t("lang-auto"));
    for lang in Lang::ALL {
        ui.radio_value(&mut settings.language, Some(lang), lang.native_name());
    }

    ui.separator();
    ui.label(title(tr.t("settings-remember-title")));
    let mut remember = consent.may_store();
    if ui
        .checkbox(&mut remember, tr.t("settings-remember"))
        .changed()
    {
        *consent = if remember {
            StorageConsent::Granted
        } else {
            StorageConsent::Denied
        };
    }
    if ui
        .button(tr.t("settings-forget"))
        .on_hover_text(tr.t("settings-forget-hover"))
        .clicked()
    {
        actions.push(Action::ForgetEverything);
        ui.close();
    }
}

pub fn topic_menu(
    ui: &mut egui::Ui,
    tr: &I18n,
    pal: &Palette,
    topics: &[Box<dyn Topic>],
    current: &Page,
    actions: &mut Vec<Action>,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(8.0);
        for group in Group::ALL {
            let in_group: Vec<&TopicMeta> = topics
                .iter()
                .map(|t| t.meta())
                .filter(|m| m.group == group)
                .collect();
            if in_group.is_empty() {
                continue;
            }
            let color = pal.group(group);
            ui.label(
                RichText::new(tr.t(group.key()).to_uppercase())
                    .font(bold_font(12.5))
                    .color(color),
            );
            ui.add_space(2.0);
            for m in in_group {
                let selected = matches!(current, Page::Topic(id) if id == m.id);
                let text = RichText::new(format!("{}  {}", m.icon, tr.topic_title(m.id)));
                let btn = egui::Button::new(if selected {
                    text.font(bold_font(16.0)).color(pal.on_accent)
                } else {
                    text
                })
                .fill(if selected {
                    color
                } else {
                    egui::Color32::TRANSPARENT
                })
                .stroke(Stroke::NONE)
                .min_size(egui::vec2(ui.available_width(), 30.0))
                .wrap_mode(egui::TextWrapMode::Truncate);
                let r = ui.add(btn).on_hover_text(tr.topic_blurb(m.id));
                if r.clicked() {
                    actions.push(Action::Go(Page::Topic(m.id.to_string())));
                }
            }
            ui.add_space(14.0);
        }
    });
}

/// "Where next?" links under a topic, from its `related` list.
pub fn where_next(
    ui: &mut egui::Ui,
    tr: &I18n,
    pal: &Palette,
    meta: &TopicMeta,
    topics: &[Box<dyn Topic>],
    actions: &mut Vec<Action>,
) {
    if meta.related.is_empty() {
        return;
    }
    ui.add_space(18.0);
    ui.label(RichText::new(tr.t("where-next")).font(bold_font(20.0)));
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        for r in meta.related {
            let Some(target) = topics.iter().map(|t| t.meta()).find(|m| m.id == r.id) else {
                continue;
            };
            let text = format!("{}  {}", target.icon, tr.t(r.hook));
            if kit::hook_button(ui, pal, &text, pal.group(target.group))
                .on_hover_text(tr.topic_title(target.id))
                .clicked()
            {
                actions.push(Action::Go(Page::Topic(r.id.to_string())));
            }
        }
    });
}

pub fn footer(ui: &mut egui::Ui, tr: &I18n, pal: &Palette, actions: &mut Vec<Action>) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 14.0;
        let small = |s: String| RichText::new(s).small();
        ui.label(small("© 2026 zgb".into()).color(pal.weak));
        for (key, page) in [
            ("footer-privacy", Page::Privacy),
            ("footer-license", Page::License),
            ("footer-about", Page::About),
        ] {
            if ui.link(small(tr.t(key))).clicked() {
                actions.push(Action::Go(page));
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(small(concat!("v", env!("CARGO_PKG_VERSION")).into()).color(pal.weak));
        });
    });
}

/// Shown at the top of the content area until the visitor answers.
pub fn consent_banner(
    ui: &mut egui::Ui,
    tr: &I18n,
    pal: &Palette,
    consent: &mut StorageConsent,
    actions: &mut Vec<Action>,
) {
    egui::Frame::new()
        .fill(pal.tint(pal.physics, 0.12))
        .stroke(Stroke::new(1.0, pal.tint(pal.physics, 0.5)))
        .corner_radius(14)
        .inner_margin(Margin::same(12))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new("💾").size(20.0));
                ui.label(tr.t("consent-text"));
            });
            ui.horizontal(|ui| {
                if kit::bright_button(ui, pal, RichText::new(tr.t("consent-yes")), pal.physics)
                    .clicked()
                {
                    *consent = StorageConsent::Granted;
                }
                if ui.button(tr.t("consent-no")).clicked() {
                    *consent = StorageConsent::Denied;
                }
                if ui.link(tr.t("footer-privacy")).clicked() {
                    actions.push(Action::Go(Page::Privacy));
                }
            });
        });
    ui.add_space(10.0);
}
