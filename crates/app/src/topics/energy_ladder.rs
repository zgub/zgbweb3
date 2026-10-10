//! The energy ladder: from one jiggling molecule to the ISS, on one log scale,
//! counted in whatever unit you like — joules, electronvolts, sugar cubes,
//! or flying mosquitoes.

use egui::{Color32, RichText, Sense, Stroke, vec2};
use serde::{Deserialize, Serialize};
use sims::energy::{EnergyExample, EnergyUnit, examples, kinetic, kmh};
use sims::fmt::{sci, si, superscript};

use super::{Group, Maturity, Nav, Related, Topic, TopicMeta, storage_key, widgets};

static META: TopicMeta = TopicMeta {
    id: "energy-ladder",
    title: "The energy ladder",
    icon: "⚡",
    group: Group::ScaleAndNumbers,
    blurb: "How much energy is a flying mosquito? A car? The space station? Line them up and count them in sugar cubes.",
    maturity: Maturity::Draft,
    related: &[
        Related {
            id: "powers-of-ten",
            hook: "What does 10⁹ actually mean?",
        },
        Related {
            id: "gas-box",
            hook: "Where does heat come from? Molecules jiggling",
        },
    ],
};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct EnergyLadder {
    unit: EnergyUnit,
    compare_a: String,
    compare_b: String,
    my_mass_kg: f64,
    my_speed_kmh: f64,
    #[serde(skip)]
    ladder: Vec<EnergyExample>,
}

impl Default for EnergyLadder {
    fn default() -> Self {
        Self {
            unit: EnergyUnit::Joule,
            compare_a: "b747_fuel".into(),
            compare_b: "iss".into(),
            my_mass_kg: 30.0,
            my_speed_kmh: 15.0,
            ladder: examples(),
        }
    }
}

const MY_ID: &str = "mine";

impl EnergyLadder {
    fn my_thing(&self) -> EnergyExample {
        EnergyExample {
            id: MY_ID,
            name: "Your thing",
            joules: kinetic(self.my_mass_kg, kmh(self.my_speed_kmh)),
            note: "",
        }
    }

    fn rows(&self) -> Vec<EnergyExample> {
        let mut rows = self.ladder.clone();
        rows.push(self.my_thing());
        rows.sort_by(|a, b| a.joules.total_cmp(&b.joules));
        rows
    }

    fn format(&self, joules: f64) -> String {
        let v = self.unit.convert(joules);
        if self.unit.takes_prefix() {
            let s = si(v, self.unit.symbol(), 3);
            // Prefixes run out at the extremes; scientific notation is clearer there.
            if (1e-24..1e27).contains(&v.abs()) {
                s
            } else {
                format!("{} {}", sci(v, 3), self.unit.symbol())
            }
        } else {
            format!("{} {}", sci(v, 3), self.unit.symbol())
        }
    }
}

impl Topic for EnergyLadder {
    fn meta(&self) -> &'static TopicMeta {
        &META
    }

    fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &storage_key(META.id), self);
    }

    fn load(&mut self, storage: &dyn eframe::Storage) {
        if let Some(mut s) = eframe::get_value::<Self>(storage, &storage_key(META.id)) {
            s.ladder = examples();
            *self = s;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, nav: &mut Nav) {
        widgets::heading(ui, &META);

        ui.horizontal_wrapped(|ui| {
            ui.label("Count energy in:");
            egui::ComboBox::from_id_salt("energy-unit")
                .selected_text(self.unit.name())
                .show_ui(ui, |ui| {
                    for u in EnergyUnit::ALL {
                        ui.selectable_value(&mut self.unit, u, u.name());
                    }
                });
        });

        ui.add_space(6.0);
        ui.group(|ui| {
            ui.label(RichText::new("Make your own moving thing").strong());
            ui.horizontal_wrapped(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.my_mass_kg, 0.001..=500_000.0)
                        .logarithmic(true)
                        .text("kg"),
                );
                ui.add(
                    egui::Slider::new(&mut self.my_speed_kmh, 0.1..=30_000.0)
                        .logarithmic(true)
                        .text("km/h"),
                );
            });
            ui.label(format!(
                "Moving energy: {}",
                self.format(self.my_thing().joules)
            ));
        });

        ui.add_space(8.0);
        let rows = self.rows();
        ladder_chart(ui, &rows, |j| self.format(j));
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("Each grid line is ×10.").weak());
            nav.link(ui, "powers-of-ten", "What does 10⁹ mean?");
        });

        ui.add_space(8.0);
        self.comparison(ui, &rows);

        widgets::grown_ups(ui, "energy-notes", |ui| {
            ui.label("Moving energy is ½·m·v². Doubling speed means four times the energy.");
            ui.add_space(4.0);
            egui::Grid::new("energy-notes-grid")
                .striped(true)
                .show(ui, |ui| {
                    for ex in &self.ladder {
                        ui.label(ex.name);
                        ui.label(format!("{} J", sci(ex.joules, 3)));
                        ui.label(RichText::new(ex.note).weak());
                        ui.end_row();
                    }
                });
        });
    }
}

impl EnergyLadder {
    fn comparison(&mut self, ui: &mut egui::Ui, rows: &[EnergyExample]) {
        let name_of = |id: &str| {
            rows.iter()
                .find(|r| r.id == id)
                .map(|r| r.name)
                .unwrap_or("?")
        };
        ui.group(|ui| {
            ui.label(RichText::new("Compare two").strong());
            ui.horizontal_wrapped(|ui| {
                for (salt, sel) in [("cmp-a", &mut self.compare_a), ("cmp-b", &mut self.compare_b)] {
                    egui::ComboBox::from_id_salt(salt)
                        .selected_text(name_of(sel))
                        .width(260.0)
                        .show_ui(ui, |ui| {
                            for r in rows {
                                ui.selectable_value(sel, r.id.to_string(), r.name);
                            }
                        });
                }
            });
            let get = |id: &str| rows.iter().find(|r| r.id == id);
            if let (Some(a), Some(b)) = (get(&self.compare_a), get(&self.compare_b)) {
                let (big, small) = if a.joules >= b.joules { (a, b) } else { (b, a) };
                let ratio = big.joules / small.joules;
                let orders = ratio.log10();
                ui.label(format!(
                    "{} has {}× the energy of {}.",
                    big.name,
                    sci(ratio, 3),
                    small.name
                ));
                if orders >= 1.0 {
                    ui.label(format!(
                        "That is {:.1} orders of magnitude: about 10{} — multiply by ten, {} times in a row.",
                        orders,
                        superscript(orders.round() as i32),
                        orders.round() as i64
                    ));
                }
            }
        });
    }
}

/// Horizontal log-scale bars, one row per example.
fn ladder_chart(ui: &mut egui::Ui, rows: &[EnergyExample], fmt: impl Fn(f64) -> String) {
    let lo = rows
        .first()
        .map(|r| r.joules.log10().floor())
        .unwrap_or(-22.0);
    let hi = rows.last().map(|r| r.joules.log10().ceil()).unwrap_or(14.0);
    let row_h = 26.0;
    let label_w = (ui.available_width() * 0.32).clamp(140.0, 300.0);
    let value_w = 130.0;
    let (resp, painter) = ui.allocate_painter(
        vec2(ui.available_width(), row_h * rows.len() as f32 + 22.0),
        Sense::hover(),
    );
    let r = resp.rect;
    let bar_x0 = r.left() + label_w + 8.0;
    let bar_x1 = (r.right() - value_w - 8.0).max(bar_x0 + 40.0);
    let x_of = |j: f64| bar_x0 + ((j.log10() - lo) / (hi - lo)) as f32 * (bar_x1 - bar_x0);
    let vis = ui.visuals();
    let font = egui::FontId::proportional(14.0);

    // Decade grid: one line per ×10.
    let grid = Stroke::new(1.0, vis.weak_text_color().gamma_multiply(0.25));
    let mut d = lo as i32;
    while d as f64 <= hi {
        let x = x_of(10f64.powi(d));
        painter.line_segment(
            [egui::pos2(x, r.top()), egui::pos2(x, r.bottom() - 18.0)],
            grid,
        );
        if d % 3 == 0 {
            painter.text(
                egui::pos2(x, r.bottom()),
                egui::Align2::CENTER_BOTTOM,
                format!("10{} J", superscript(d)),
                egui::FontId::proportional(11.0),
                vis.weak_text_color(),
            );
        }
        d += 1;
    }

    for (i, row) in rows.iter().enumerate() {
        let y = r.top() + i as f32 * row_h;
        let mid = y + row_h / 2.0;
        let mine = row.id == MY_ID;
        let color = if mine {
            Color32::from_rgb(0xe0, 0x8a, 0x3c)
        } else {
            vis.selection.bg_fill
        };
        painter.text(
            egui::pos2(r.left() + label_w, mid),
            egui::Align2::RIGHT_CENTER,
            row.name,
            font.clone(),
            if mine { color } else { vis.text_color() },
        );
        let bar = egui::Rect::from_min_max(
            egui::pos2(bar_x0, y + 5.0),
            egui::pos2(x_of(row.joules), y + row_h - 5.0),
        );
        painter.rect_filled(bar, 3.0, color);
        painter.text(
            egui::pos2(bar_x1 + 8.0, mid),
            egui::Align2::LEFT_CENTER,
            fmt(row.joules),
            font.clone(),
            vis.text_color(),
        );
    }
}
