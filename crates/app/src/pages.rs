//! Non-topic pages and routing.

use egui::RichText;
use serde::{Deserialize, Serialize};

use crate::i18n::I18n;
use crate::kit;
use crate::theme::{Palette, bold_font, heading_font};
use crate::topics::{Group, Topic};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Page {
    #[default]
    Home,
    About,
    Privacy,
    License,
    Topic(String),
}

/// Routes taken by pages; topic ids must not use these (checked in tests).
#[cfg_attr(not(test), allow(dead_code))]
pub const RESERVED_ROUTES: [&str; 4] = ["", "about", "privacy", "license"];

impl Page {
    /// The part after `#/` in the URL.
    pub fn route(&self) -> String {
        match self {
            Page::Home => String::new(),
            Page::About => "about".into(),
            Page::Privacy => "privacy".into(),
            Page::License => "license".into(),
            Page::Topic(id) => id.clone(),
        }
    }

    /// Unknown routes fall back to Home. `topic-id/param` routes to the topic
    /// (the param is handed to it separately, see [`route_param`]).
    pub fn from_route(route: &str, is_topic: impl Fn(&str) -> bool) -> Page {
        let route = route.trim_matches('/');
        let head = route.split('/').next().unwrap_or("");
        match head {
            "" => Page::Home,
            "about" => Page::About,
            "privacy" => Page::Privacy,
            "license" => Page::License,
            id if is_topic(id) => Page::Topic(id.to_string()),
            _ => Page::Home,
        }
    }

    /// Catalog key of the page title (topics have their own).
    pub fn title_key(&self) -> Option<&'static str> {
        match self {
            Page::Home | Page::Topic(_) => None,
            Page::About => Some("footer-about"),
            Page::Privacy => Some("footer-privacy"),
            Page::License => Some("footer-license"),
        }
    }
}

/// `"powers-of-ten/9"` → `Some("9")`.
pub fn route_param(route: &str) -> Option<&str> {
    route
        .trim_matches('/')
        .split_once('/')
        .map(|(_, p)| p)
        .filter(|p| !p.is_empty())
}

pub fn home(
    ui: &mut egui::Ui,
    tr: &I18n,
    pal: &Palette,
    topics: &[Box<dyn Topic>],
) -> Option<Page> {
    let mut go = None;
    ui.add_space(6.0);
    ui.label(RichText::new(tr.t("home-title")).font(heading_font(40.0)));
    ui.label(RichText::new(tr.t("home-intro")).size(19.0).color(pal.weak));
    ui.add_space(18.0);

    for group in Group::ALL {
        let metas: Vec<_> = topics
            .iter()
            .map(|t| t.meta())
            .filter(|m| m.group == group)
            .collect();
        if metas.is_empty() {
            continue;
        }
        let color = pal.group(group);
        ui.horizontal(|ui| {
            let (r, _) = ui.allocate_exact_size(egui::vec2(6.0, 22.0), egui::Sense::hover());
            ui.painter().rect_filled(r, 3, color);
            ui.label(
                RichText::new(tr.t(group.key()))
                    .font(bold_font(20.0))
                    .color(color),
            );
        });
        ui.add_space(6.0);
        // As many ~270 pt columns as fit, sharing the width evenly.
        let gap = 14.0;
        let frame_extra = 2.0 * 14.0 + 2.0; // card margins + stroke
        let avail = ui.available_width();
        let cols = ((avail + gap) / (250.0 + frame_extra + gap))
            .floor()
            .max(1.0);
        let inner_w = ((avail - gap * (cols - 1.0)) / cols - frame_extra).floor();
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(gap, gap);
            for m in metas {
                let card = kit::card(pal).show(ui, |ui| {
                    ui.set_width(inner_w);
                    ui.set_min_height(118.0);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            kit::icon_badge(ui, pal, m.icon, color, 38.0);
                            ui.vertical(|ui| {
                                ui.label(RichText::new(tr.topic_title(m.id)).font(bold_font(18.0)));
                                m.maturity.badge(ui, pal, tr);
                            });
                        });
                        ui.add_space(4.0);
                        ui.label(RichText::new(tr.topic_blurb(m.id)).color(pal.weak));
                    });
                });
                if kit::clickable(ui, card.response.rect, ui.id().with(m.id), color).clicked() {
                    go = Some(Page::Topic(m.id.to_string()));
                }
            }
        });
        ui.add_space(20.0);
    }
    go
}

fn page_title(ui: &mut egui::Ui, text: String) {
    ui.add_space(6.0);
    ui.label(RichText::new(text).font(heading_font(34.0)));
    ui.add_space(8.0);
}

/// A titled section from a catalog message: value = body, `.title` = heading.
fn section(ui: &mut egui::Ui, tr: &I18n, key: &str) {
    ui.add_space(8.0);
    ui.label(RichText::new(tr.attr(key, "title")).font(bold_font(18.0)));
    ui.label(tr.t(key));
}

pub fn about(ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
    page_title(ui, tr.t("footer-about"));
    ui.label(tr.t("about-body"));
    ui.add_space(6.0);
    ui.label(tr.t("about-tech"));
    ui.label(RichText::new(tr.t("about-source")).color(pal.weak));
}

pub fn privacy(ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
    page_title(ui, tr.t("footer-privacy"));
    ui.label(RichText::new(tr.t("draft-note")).color(pal.weak).italics());
    for key in [
        "privacy-tracking",
        "privacy-device",
        "privacy-logs",
        "privacy-changes",
        "privacy-operator",
    ] {
        section(ui, tr, key);
    }
}

pub fn license(ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
    page_title(ui, tr.t("footer-license"));
    ui.label(RichText::new(tr.t("draft-note")).color(pal.weak).italics());
    ui.add_space(6.0);
    ui.label(tr.t("license-code"));
    ui.label(tr.t("license-content"));
    section(ui, tr, "license-third");
    ui.label("egui / eframe — MIT OR Apache-2.0");
    ui.label("Nunito — SIL Open Font License 1.1 (The Nunito Project Authors)");
    ui.label("Hack — MIT; Noto Emoji — OFL 1.1; Emoji Icon Font — MIT / OFL");
    ui.label("Project Fluent (fluent-bundle) — MIT OR Apache-2.0");
}
