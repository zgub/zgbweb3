//! Topics: each explainer/simulation is one `Topic`.
//!
//! Adding a topic:
//! 1. put its maths/physics in the `sims` crate (with tests),
//! 2. add a module here implementing [`Topic`],
//! 3. register it in [`registry`].
//!
//! The `id` is permanent: it appears in URLs (`#/<id>`) and storage keys.

mod clt_dice;
mod energy_ladder;
mod planned;
mod powers_of_ten;

use egui::{Color32, RichText};

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

    pub fn label(self) -> &'static str {
        match self {
            Group::ScaleAndNumbers => "Scale & numbers",
            Group::Chance => "Chance & statistics",
            Group::Physics => "Physics",
            Group::Maths => "Maths",
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
    pub fn badge(self) -> RichText {
        let (text, color) = match self {
            Maturity::Planned => ("planned", Color32::from_rgb(0x8a, 0x8a, 0x8a)),
            Maturity::Draft => ("draft", Color32::from_rgb(0xd9, 0xa4, 0x41)),
            Maturity::Ready => ("ready", Color32::from_rgb(0x5c, 0xb8, 0x5c)),
        };
        RichText::new(text).small().color(color)
    }
}

/// A pointer to another topic, phrased as the question a kid might ask.
pub struct Related {
    pub id: &'static str,
    /// e.g. "What does 10⁹ actually mean?"
    pub hook: &'static str,
}

pub struct TopicMeta {
    pub id: &'static str,
    pub title: &'static str,
    pub icon: &'static str,
    pub group: Group,
    /// One or two sentences for menus and the home page.
    pub blurb: &'static str,
    pub maturity: Maturity,
    /// Shown as "Where next?" under the topic. Every id must be registered (tested).
    pub related: &'static [Related],
}

/// Lets a topic link to another topic from inside its text.
#[derive(Default)]
pub struct Nav {
    pub(crate) go: Option<&'static str>,
}

impl Nav {
    /// A link-styled "hook → " button that opens topic `id`.
    pub fn link(&mut self, ui: &mut egui::Ui, id: &'static str, hook: &str) {
        let r = ui.link(RichText::new(format!("{hook} →")).italics());
        if r.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            self.go = Some(id);
        }
    }
}

pub trait Topic {
    fn meta(&self) -> &'static TopicMeta;

    /// Draw the topic into the central area. Called every frame while visible.
    fn ui(&mut self, ui: &mut egui::Ui, nav: &mut Nav);

    /// Persist this topic's state (only called when the visitor allowed storage).
    fn save(&self, _storage: &mut dyn eframe::Storage) {}

    /// Restore state saved by [`Topic::save`].
    fn load(&mut self, _storage: &dyn eframe::Storage) {}
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

/// Shared small widgets for topics.
pub mod widgets {
    use egui::RichText;

    pub fn heading(ui: &mut egui::Ui, meta: &super::TopicMeta) {
        ui.horizontal(|ui| {
            ui.heading(format!("{} {}", meta.icon, meta.title));
            ui.label(meta.maturity.badge());
        });
        ui.label(RichText::new(meta.blurb).italics());
        ui.add_space(8.0);
    }

    /// A gentle "for the grown-ups" box for assumptions and maths details.
    pub fn grown_ups(ui: &mut egui::Ui, id: &str, add: impl FnOnce(&mut egui::Ui)) {
        egui::CollapsingHeader::new("For the grown-ups: how the numbers are made")
            .id_salt(id)
            .default_open(false)
            .show(ui, add);
    }
}

#[cfg(test)]
mod tests {
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
}
