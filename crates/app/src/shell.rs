//! Page chrome: top bar, topic menu (right), footer, storage-consent banner.
//!
//! These functions never mutate app state directly beyond the settings they are
//! handed; navigation and other effects come back as [`Action`]s.

use egui::RichText;

use crate::pages::Page;
use crate::settings::{Settings, StorageConsent, ThemeChoice, ZOOM_RANGE};
use crate::topics::{Group, Topic, TopicMeta};

pub enum Action {
    Go(Page),
    ForgetEverything,
}

pub fn top_bar(
    ui: &mut egui::Ui,
    settings: &mut Settings,
    consent: &mut StorageConsent,
    current_title: Option<&str>,
    actions: &mut Vec<Action>,
) {
    ui.horizontal(|ui| {
        let home =
            ui.add(egui::Button::new(RichText::new("zgb ★").strong().size(18.0)).frame(false));
        if home.on_hover_text("Home").clicked() {
            actions.push(Action::Go(Page::Home));
        }
        if let Some(t) = current_title {
            ui.label(RichText::new("›").weak());
            ui.label(t);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.toggle_value(&mut settings.menu_open, "☰ Topics")
                .on_hover_text("Show or hide the topic list");
            ui.menu_button("⚙", |ui| settings_menu(ui, settings, consent, actions))
                .response
                .on_hover_text("Settings");
        });
    });
}

fn settings_menu(
    ui: &mut egui::Ui,
    settings: &mut Settings,
    consent: &mut StorageConsent,
    actions: &mut Vec<Action>,
) {
    ui.set_min_width(240.0);
    ui.label(RichText::new("Look").strong());
    for t in ThemeChoice::ALL {
        ui.radio_value(&mut settings.theme, t, t.label());
    }
    ui.horizontal(|ui| {
        ui.label("Size");
        ui.add(
            egui::Slider::new(&mut settings.zoom, ZOOM_RANGE)
                .step_by(0.05)
                .show_value(false),
        );
        if ui.small_button("reset").clicked() {
            settings.zoom = 1.0;
        }
    });
    ui.checkbox(&mut settings.calm_motion, "Calm motion (fewer animations)");

    ui.separator();
    ui.label(RichText::new("Remember on this device").strong());
    let mut remember = consent.may_store();
    if ui
        .checkbox(&mut remember, "Remember my settings and experiments")
        .changed()
    {
        *consent = if remember {
            StorageConsent::Granted
        } else {
            StorageConsent::Denied
        };
    }
    if ui
        .button("🗑 Forget everything")
        .on_hover_text("Delete everything this site stored in your browser")
        .clicked()
    {
        actions.push(Action::ForgetEverything);
        ui.close();
    }
}

pub fn topic_menu(
    ui: &mut egui::Ui,
    topics: &[Box<dyn Topic>],
    current: &Page,
    actions: &mut Vec<Action>,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(6.0);
        for group in Group::ALL {
            let in_group: Vec<_> = topics.iter().filter(|t| t.meta().group == group).collect();
            if in_group.is_empty() {
                continue;
            }
            ui.label(RichText::new(group.label().to_uppercase()).small().weak());
            for t in in_group {
                let m = t.meta();
                let selected = matches!(current, Page::Topic(id) if id == m.id);
                ui.horizontal(|ui| {
                    let r = ui.add(egui::Button::selectable(
                        selected,
                        format!("{} {}", m.icon, m.title),
                    ));
                    if r.on_hover_text(m.blurb).clicked() {
                        actions.push(Action::Go(Page::Topic(m.id.to_string())));
                    }
                    ui.label(m.maturity.badge());
                });
            }
            ui.add_space(10.0);
        }
    });
}

/// "Where next?" links under a topic, from its `related` list.
pub fn where_next(
    ui: &mut egui::Ui,
    meta: &TopicMeta,
    topics: &[Box<dyn Topic>],
    actions: &mut Vec<Action>,
) {
    if meta.related.is_empty() {
        return;
    }
    ui.add_space(16.0);
    ui.separator();
    ui.label(RichText::new("Where next?").strong());
    ui.horizontal_wrapped(|ui| {
        for r in meta.related {
            let Some(target) = topics
                .iter()
                .find(|t| t.meta().id == r.id)
                .map(|t| t.meta())
            else {
                continue;
            };
            let text = format!("{} {}", target.icon, r.hook);
            let resp = ui.add(egui::Button::new(text)).on_hover_text(target.title);
            if resp.clicked() {
                actions.push(Action::Go(Page::Topic(r.id.to_string())));
            }
        }
    });
}

pub fn footer(ui: &mut egui::Ui, actions: &mut Vec<Action>) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 10.0;
        let small = |s: &str| RichText::new(s.to_string()).small();
        ui.label(small("© 2026 zgbweb").weak());
        for (label, page) in [
            ("Privacy", Page::Privacy),
            ("License", Page::License),
            ("About", Page::About),
        ] {
            if ui.link(small(label)).clicked() {
                actions.push(Action::Go(page));
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(small(concat!("v", env!("CARGO_PKG_VERSION"))).weak());
        });
    });
}

/// Shown at the top of the content area until the visitor answers.
pub fn consent_banner(ui: &mut egui::Ui, consent: &mut StorageConsent, actions: &mut Vec<Action>) {
    egui::Frame::group(ui.style())
        .fill(ui.visuals().faint_bg_color)
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.label(
                "Should this page remember your settings and experiments on this device? \
                 They are saved only in your browser and never sent anywhere.",
            );
            ui.horizontal(|ui| {
                if ui.button("Yes, remember").clicked() {
                    *consent = StorageConsent::Granted;
                }
                if ui.button("No thanks").clicked() {
                    *consent = StorageConsent::Denied;
                }
                if ui.link("Privacy").clicked() {
                    actions.push(Action::Go(Page::Privacy));
                }
            });
        });
    ui.add_space(8.0);
}
