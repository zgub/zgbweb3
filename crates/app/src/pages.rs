//! Routing, section index pages, planned pages, and the fixed pages
//! (about, privacy, license).

use content::{PageMeta, Section};
use egui::RichText;
use serde::{Deserialize, Serialize};

use crate::i18n::I18n;
use crate::theme::Palette;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Page {
    /// `#/`: the index of [`content::LANDING`] (D-022).
    #[default]
    Landing,
    /// `#/explore`, `#/projects`, …
    Section(Section),
    About,
    Privacy,
    License,
    /// `#/<id>`: any page on the site map, built or planned.
    Item(String),
}

impl Page {
    /// The part after `#/` in the URL.
    pub fn route(&self) -> String {
        match self {
            Page::Landing => String::new(),
            Page::Section(s) => s.id().into(),
            Page::About => "about".into(),
            Page::Privacy => "privacy".into(),
            Page::License => "license".into(),
            Page::Item(id) => id.clone(),
        }
    }

    /// Unknown routes fall back to the landing page. `id/param` routes to the
    /// page (the param is handed to it separately, see [`route_param`]).
    pub fn from_route(route: &str) -> Page {
        let route = route.trim_matches('/');
        let head = route.split('/').next().unwrap_or("");
        match head {
            "" => Page::Landing,
            "about" => Page::About,
            "privacy" => Page::Privacy,
            "license" => Page::License,
            other => {
                if let Some(s) = Section::from_id(other) {
                    Page::Section(s)
                } else if content::page(other).is_some() {
                    Page::Item(other.to_string())
                } else {
                    Page::Landing
                }
            }
        }
    }

    /// Does this page still exist (e.g. after loading a saved page)?
    pub fn exists(&self) -> bool {
        match self {
            Page::Item(id) => content::page(id).is_some(),
            _ => true,
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

fn page_title(ui: &mut egui::Ui, text: String) {
    ui.heading(text);
    ui.separator();
}

/// A section's index: its intro and one row per page. Returns a page to open.
pub fn section(ui: &mut egui::Ui, tr: &I18n, pal: &Palette, section: Section) -> Option<Page> {
    let mut go = None;
    page_title(ui, format!("{} {}", section.glyph(), tr.t(section.key())));
    ui.label(RichText::new(tr.attr(section.key(), "intro")).color(pal.weak));
    ui.add_space(8.0);

    let mut any = false;
    for m in content::pages_in(section) {
        any = true;
        ui.horizontal_wrapped(|ui| {
            let title = format!("{} {}", m.glyph, tr.page_title(m.id));
            if ui.link(title).clicked() {
                go = Some(Page::Item(m.id.to_string()));
            }
            if let Some(key) = m.status.key() {
                ui.label(RichText::new(tr.t(key)).small().color(pal.weak));
            }
        });
        if let Some(blurb) = tr.page_blurb(m.id) {
            ui.indent(m.id, |ui| ui.label(RichText::new(blurb).color(pal.weak)));
        }
        ui.add_space(2.0);
    }
    if !any {
        ui.label(
            RichText::new(tr.t("section-empty"))
                .italics()
                .color(pal.weak),
        );
    }
    go
}

/// A page that is on the map but has no code yet.
pub fn planned(ui: &mut egui::Ui, tr: &I18n, pal: &Palette, meta: &PageMeta) {
    crate::topics::heading(ui, tr, pal, meta);
    ui.label(
        RichText::new(tr.t("planned-body"))
            .italics()
            .color(pal.weak),
    );
}

/// A titled section from a catalog message: value = body, `.title` = heading.
fn titled(ui: &mut egui::Ui, tr: &I18n, key: &str) {
    ui.add_space(8.0);
    ui.strong(tr.attr(key, "title"));
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
        titled(ui, tr, key);
    }
}

pub fn license(ui: &mut egui::Ui, tr: &I18n, pal: &Palette) {
    page_title(ui, tr.t("footer-license"));
    ui.label(RichText::new(tr.t("draft-note")).color(pal.weak).italics());
    ui.add_space(6.0);
    ui.label(tr.t("license-code"));
    ui.label(tr.t("license-content"));
    titled(ui, tr, "license-third");
    ui.label("egui / eframe — MIT OR Apache-2.0");
    ui.label("Ubuntu font — Ubuntu Font Licence 1.0; Hack — MIT; Noto Emoji — OFL 1.1; Emoji Icon Font — MIT / OFL");
    ui.label("Project Fluent (fluent-bundle) — MIT OR Apache-2.0");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_round_trip() {
        let mut pages = vec![Page::Landing, Page::About, Page::Privacy, Page::License];
        pages.extend(Section::ALL.map(Page::Section));
        pages.extend(content::PAGES.iter().map(|p| Page::Item(p.id.to_string())));
        for p in pages {
            assert_eq!(Page::from_route(&p.route()), p, "{}", p.route());
        }
    }

    #[test]
    fn unknown_routes_land_and_params_split() {
        assert_eq!(Page::from_route("no-such-page"), Page::Landing);
        assert_eq!(
            Page::from_route("/powers-of-ten/9"),
            Page::Item("powers-of-ten".into())
        );
        assert_eq!(route_param("powers-of-ten/9"), Some("9"));
        assert_eq!(route_param("powers-of-ten"), None);
    }
}
