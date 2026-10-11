//! Placeholder for topics that exist as an idea and a plan, not yet as code.
//! The plan text is the catalog attribute `topic-<id>.plan`, one bullet per line.

use egui::RichText;

use super::{Cx, Group, Maturity, Related, Topic, TopicMeta, heading};

pub struct PlannedTopic {
    pub meta: TopicMeta,
}

pub static NUMBERS: PlannedTopic = PlannedTopic {
    meta: TopicMeta {
        id: "numbers",
        icon: "🔢",
        group: Group::ScaleAndNumbers,
        maturity: Maturity::Planned,
        related: &[
            Related {
                id: "powers-of-ten",
                hook: "hook-how-big-is-a-billion",
            },
            Related {
                id: "linear-maps",
                hook: "hook-turning-with-a-matrix",
            },
        ],
    },
};

pub static GAS_BOX: PlannedTopic = PlannedTopic {
    meta: TopicMeta {
        id: "gas-box",
        icon: "💨",
        group: Group::Physics,
        maturity: Maturity::Planned,
        related: &[
            Related {
                id: "dice-bell",
                hook: "hook-why-random-makes-a-bell",
            },
            Related {
                id: "energy-ladder",
                hook: "hook-energy-of-one-molecule",
            },
        ],
    },
};

pub static LINEAR_MAPS: PlannedTopic = PlannedTopic {
    meta: TopicMeta {
        id: "linear-maps",
        icon: "🔷",
        group: Group::Maths,
        maturity: Maturity::Planned,
        related: &[Related {
            id: "numbers",
            hook: "hook-numbers-that-turn",
        }],
    },
};

pub struct Planned {
    def: &'static PlannedTopic,
}

impl Planned {
    pub fn new(def: &'static PlannedTopic) -> Self {
        Self { def }
    }
}

impl Topic for Planned {
    fn meta(&self) -> &'static TopicMeta {
        &self.def.meta
    }

    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Cx) {
        let meta = &self.def.meta;
        heading(ui, cx, meta);
        crate::kit::panel(ui, cx.pal, Some(&cx.tr.t("planned-intro")), |ui| {
            let color = cx.pal.group(meta.group);
            for step in cx.tr.attr(&format!("topic-{}", meta.id), "plan").lines() {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("●").color(color));
                    ui.label(step.trim());
                });
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::i18n::{I18n, Lang};

    #[test]
    fn planned_topics_have_plans() {
        for lang in Lang::ALL {
            let tr = I18n::new(lang);
            for def in [&super::NUMBERS, &super::GAS_BOX, &super::LINEAR_MAPS] {
                let key = format!("topic-{}", def.meta.id);
                assert!(tr.has(&key, Some("plan")), "{}: {key}.plan", lang.code());
            }
        }
    }
}
