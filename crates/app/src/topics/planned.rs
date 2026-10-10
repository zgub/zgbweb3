//! Placeholder for topics that exist as an idea and a plan, not yet as code.

use super::{Group, Maturity, Nav, Related, Topic, TopicMeta, widgets};

pub struct PlannedTopic {
    pub meta: TopicMeta,
    pub plan: &'static [&'static str],
}

pub static NUMBERS: PlannedTopic = PlannedTopic {
    meta: TopicMeta {
        id: "numbers",
        title: "What is a number?",
        icon: "🔢",
        group: Group::ScaleAndNumbers,
        blurb: "Counting, then zero, then minus, then halves, then √2 — and finally a number that turns things by 90°.",
        maturity: Maturity::Planned,
        related: &[
            Related {
                id: "powers-of-ten",
                hook: "How big is a billion, really?",
            },
            Related {
                id: "linear-maps",
                hook: "Turning by 90° with a matrix",
            },
        ],
    },
    plan: &[
        "One number line that grows with each idea: counting numbers → zero → negatives (thermometer, lift floors)",
        "Fractions and decimals: cut a bar into pieces; 0.1 is one slab of the powers-of-ten cube",
        "√2: the diagonal of a unit square is not any fraction (the classic proof, told as a story)",
        "π and friends: the line has no gaps — the real numbers",
        "i: multiplying by −1 turns around (180°); is there a number that turns only 90°? The line becomes a plane",
        "Every step answers a question the previous kind of number could not (subtract more than you have, share 1 cake by 3, …)",
    ],
};

pub static GAS_BOX: PlannedTopic = PlannedTopic {
    meta: TopicMeta {
        id: "gas-box",
        title: "Gas in a box",
        icon: "💨",
        group: Group::Physics,
        blurb: "Hundreds of bouncing balls make pressure and temperature — and slowly forget where they started.",
        maturity: Maturity::Planned,
        related: &[
            Related {
                id: "dice-bell",
                hook: "Why do random things make a bell?",
            },
            Related {
                id: "energy-ladder",
                hook: "How much energy is one jiggling molecule?",
            },
        ],
    },
    plan: &[
        "2D hard-disc gas, elastic collisions, adjustable number of particles and box size",
        "Speed histogram converging to the Maxwell–Boltzmann (2D: Rayleigh) distribution",
        "Pressure from wall hits; push a piston and watch it heat up",
        "Two gases separated by a wall: remove it, watch mixing (and never un-mixing)",
        "Physics in sims::gas, rendered with the egui painter first, wgpu if particle count needs it",
    ],
};

pub static LINEAR_MAPS: PlannedTopic = PlannedTopic {
    meta: TopicMeta {
        id: "linear-maps",
        title: "Matrices move the plane",
        icon: "🔷",
        group: Group::Maths,
        blurb: "A 2×2 matrix is just where the two arrows î and ĵ end up. Drag them and see the whole grid follow.",
        maturity: Maturity::Planned,
        related: &[Related {
            id: "numbers",
            hook: "Numbers that turn things: i",
        }],
    },
    plan: &[
        "Draggable basis vectors î, ĵ; the grid and a picture (house/smiley) transform live",
        "Determinant as area of the unit square; negative = the picture flips",
        "Composition: apply A then B, compare with B·A",
        "Eigenvectors: the arrows that only stretch, highlighted",
    ],
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

    fn ui(&mut self, ui: &mut egui::Ui, _nav: &mut Nav) {
        widgets::heading(ui, &self.def.meta);
        ui.label("This one is still on the drawing board. The plan:");
        ui.add_space(4.0);
        for step in self.def.plan {
            ui.horizontal_wrapped(|ui| {
                ui.label("•");
                ui.label(*step);
            });
        }
    }
}
