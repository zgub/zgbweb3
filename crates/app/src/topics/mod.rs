//! Topics: each explainer/simulation is one `Topic`.
//!
//! Adding a topic:
//! 1. put its maths/physics in the `sims` crate (with tests),
//! 2. add a module here implementing [`Topic`],
//! 3. add `topic-<id> = Title` with `.blurb` (and its other texts) to every
//!    catalog in `crates/app/locales/`,
//! 4. register it in [`registry`].
//!
//! The `id` is permanent: it appears in URLs (`#/<id>`), storage keys and catalog keys.

mod clt_dice;
mod energy_ladder;
mod planned;
mod powers_of_ten;

use egui::RichText;

use crate::i18n::I18n;
use crate::kit;
use crate::theme::{Palette, heading_font};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    ScaleAndNumbers,
    Chance,
    Physics,
    Maths,
}

impl Group {
    pub const ALL: [Group; 4] = [
        Group::ScaleAndNumbers,
        Group::Chance,
        Group::Physics,
        Group::Maths,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Group::ScaleAndNumbers => "group-scale",
            Group::Chance => "group-chance",
            Group::Physics => "group-physics",
            Group::Maths => "group-maths",
        }
    }
}

/// How finished a topic is. Shown as a badge so visitors know what to expect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Maturity {
    /// Only a description and a plan.
    Planned,
    /// Works, still rough.
    Draft,
    /// Reviewed and polished.
    #[allow(dead_code)] // nothing is polished yet
    Ready,
}

impl Maturity {
    pub fn badge(self, ui: &mut egui::Ui, pal: &Palette, tr: &I18n) {
        let (key, color) = match self {
            Maturity::Planned => ("maturity-planned", pal.weak),
            Maturity::Draft => ("maturity-draft", pal.scale),
            Maturity::Ready => ("maturity-ready", pal.physics),
        };
        kit::pill(ui, pal, tr.t(key), color);
    }
}

/// A pointer to another topic, phrased as the question a kid might ask.
pub struct Related {
    pub id: &'static str,
    /// Catalog key of the question, e.g. `hook-what-is-a-billion`.
    pub hook: &'static str,
}

/// Language-independent facts about a topic. Texts come from the catalogs.
pub struct TopicMeta {
    pub id: &'static str,
    pub icon: &'static str,
    pub group: Group,
    pub maturity: Maturity,
    /// Shown as "Where next?" under the topic. Every id must be registered (tested).
    pub related: &'static [Related],
}

/// What a topic gets while drawing: translations, colours, and a way to navigate.
pub struct Cx<'a> {
    pub tr: &'a I18n,
    pub pal: &'static Palette,
    /// The visitor asked for less animation.
    #[allow(dead_code)] // honoured by future auto-playing sims
    pub calm: bool,
    go: Option<&'static str>,
}

impl<'a> Cx<'a> {
    pub fn new(tr: &'a I18n, pal: &'static Palette, calm: bool) -> Self {
        Self {
            tr,
            pal,
            calm,
            go: None,
        }
    }

    /// An inline "question →" link to topic `id`; `hook` is a catalog key.
    pub fn link(&mut self, ui: &mut egui::Ui, id: &'static str, hook: &str) {
        let r = ui.link(RichText::new(format!("{} →", self.tr.t(hook))).italics());
        if r.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            self.go = Some(id);
        }
    }

    pub fn navigation(&self) -> Option<&'static str> {
        self.go
    }
}

pub trait Topic {
    fn meta(&self) -> &'static TopicMeta;

    /// Draw the topic into the central area. Called every frame while visible.
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Cx);

    /// Persist this topic's state (only called when the visitor allowed storage).
    fn save(&self, _storage: &mut dyn eframe::Storage) {}

    /// Restore state saved by [`Topic::save`].
    fn load(&mut self, _storage: &dyn eframe::Storage) {}

    /// Deep link: the part after `#/<id>/`, e.g. `9` in `#/powers-of-ten/9`.
    fn set_param(&mut self, _param: &str) {}
}

/// Storage key for a topic's state.
pub fn storage_key(id: &str) -> String {
    format!("topic/{id}")
}

/// All topics, in menu order within each group.
pub fn registry() -> Vec<Box<dyn Topic>> {
    vec![
        Box::new(planned::Planned::new(&planned::NUMBERS)),
        Box::new(powers_of_ten::PowersOfTen::default()),
        Box::new(energy_ladder::EnergyLadder::default()),
        Box::new(clt_dice::CltDice::default()),
        Box::new(planned::Planned::new(&planned::GAS_BOX)),
        Box::new(planned::Planned::new(&planned::LINEAR_MAPS)),
    ]
}

/// Topic title block: icon badge, big title, group + maturity pills, blurb.
pub fn heading(ui: &mut egui::Ui, cx: &Cx, meta: &TopicMeta) {
    let color = cx.pal.group(meta.group);
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        kit::icon_badge(ui, cx.pal, meta.icon, color, 52.0);
        ui.vertical(|ui| {
            ui.label(RichText::new(cx.tr.topic_title(meta.id)).font(heading_font(32.0)));
            ui.horizontal(|ui| {
                kit::pill(ui, cx.pal, cx.tr.t(meta.group.key()), color);
                meta.maturity.badge(ui, cx.pal, cx.tr);
            });
        });
    });
    ui.add_space(6.0);
    ui.label(
        RichText::new(cx.tr.topic_blurb(meta.id))
            .size(18.0)
            .color(cx.pal.weak),
    );
    ui.add_space(10.0);
}

#[cfg(test)]
mod tests {
    use crate::i18n::{I18n, Lang};

    #[test]
    fn topic_ids_are_unique_and_url_safe() {
        let topics = super::registry();
        let mut ids: Vec<&str> = topics.iter().map(|t| t.meta().id).collect();
        for id in &ids {
            assert!(
                id.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "bad id {id}"
            );
            assert!(
                !crate::pages::RESERVED_ROUTES.contains(id),
                "id {id} clashes with a page route"
            );
        }
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), topics.len());
    }

    #[test]
    fn related_links_point_at_registered_topics() {
        let topics = super::registry();
        let known: Vec<&str> = topics.iter().map(|t| t.meta().id).collect();
        for t in &topics {
            for r in t.meta().related {
                assert!(
                    known.contains(&r.id),
                    "{} links to unknown topic {}",
                    t.meta().id,
                    r.id
                );
                assert_ne!(r.id, t.meta().id, "{} links to itself", r.id);
            }
        }
    }

    #[test]
    fn every_topic_has_its_texts_in_every_language() {
        for lang in Lang::ALL {
            let tr = I18n::new(lang);
            for t in super::registry() {
                let m = t.meta();
                let key = format!("topic-{}", m.id);
                assert!(tr.has(&key, None), "{}: {key}", lang.code());
                assert!(tr.has(&key, Some("blurb")), "{}: {key}.blurb", lang.code());
                for r in m.related {
                    assert!(tr.has(r.hook, None), "{}: {}", lang.code(), r.hook);
                }
            }
            for g in super::Group::ALL {
                assert!(tr.has(g.key(), None), "{}: {}", lang.code(), g.key());
            }
        }
    }
}
