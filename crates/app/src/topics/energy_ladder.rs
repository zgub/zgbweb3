//! The energy ladder: from one jiggling molecule to the ISS, on one log scale,
//! counted in whatever unit you like — joules, electronvolts, sugar cubes,
//! or flying mosquitoes.

use egui::{CornerRadius, RichText, Sense, Stroke, vec2};
use serde::{Deserialize, Serialize};
use sims::energy::{EnergyUnit, examples, kinetic, kmh};
use sims::fmt::{engineering, sci, sci_count, sig_digits, superscript};

use super::{Cx, Topic, heading, storage_key};
use crate::args;
use crate::i18n::{I18n, lower_first};
use crate::kit;

const ID: &str = "energy-ladder";

const MY_ID: &str = "mine";

/// One row of the ladder, with its text already looked up.
struct Row {
    id: &'static str,
    name: String,
    joules: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct EnergyLadder {
    unit: EnergyUnit,
    compare_a: String,
    compare_b: String,
    my_mass_kg: f64,
    my_speed_kmh: f64,
}

impl Default for EnergyLadder {
    fn default() -> Self {
        Self {
            unit: EnergyUnit::Joule,
            compare_a: "b747_fuel".into(),
            compare_b: "iss".into(),
            my_mass_kg: 30.0,
            my_speed_kmh: 15.0,
        }
    }
}

impl EnergyLadder {
    fn my_joules(&self) -> f64 {
        kinetic(self.my_mass_kg, kmh(self.my_speed_kmh))
    }

    fn rows(&self, tr: &I18n) -> Vec<Row> {
        let mut rows: Vec<Row> = examples()
            .into_iter()
            .map(|e| Row {
                id: e.id,
                name: tr.t(&format!("energy-ex-{}", e.id)),
                joules: e.joules,
            })
            .collect();
        rows.push(Row {
            id: MY_ID,
            name: tr.t("energy-ex-mine"),
            joules: self.my_joules(),
        });
        rows.sort_by(|a, b| a.joules.total_cmp(&b.joules));
        rows
    }
}

/// "12.3 kJ", "4.1 × 10²¹ eV", "3,4 kocky cukru" — in the chosen unit and language.
fn format_energy(tr: &I18n, unit: EnergyUnit, joules: f64) -> String {
    let v = unit.convert(joules);
    match unit.symbol() {
        Some(sym) if unit.takes_prefix() && (1e-24..1e27).contains(&v.abs()) => {
            let (m, p) = engineering(v);
            tr.num(&format!("{} {}{sym}", sig_digits(m, 3), p.symbol))
        }
        Some(sym) => tr.num(&format!("{} {sym}", sci(v, 3))),
        None => tr.ta(
            &format!("amount-{}", unit.key()),
            &args!["shown" => tr.num(&sci(v, 3)), "count" => sci_count(v, 3)],
        ),
    }
}

impl Topic for EnergyLadder {
    fn id(&self) -> &'static str {
        ID
    }

    fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &storage_key(ID), self);
    }

    fn load(&mut self, storage: &dyn eframe::Storage) {
        if let Some(s) = eframe::get_value::<Self>(storage, &storage_key(ID)) {
            *self = s;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Cx) {
        let (tr, pal) = (cx.tr, cx.pal);
        heading(
            ui,
            cx.tr,
            cx.pal,
            content::page(ID).expect("on the site map"),
        );

        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(tr.t("energy-count-in")).size(18.0));
            egui::ComboBox::from_id_salt("energy-unit")
                .selected_text(tr.t(&format!("unit-{}", self.unit.key())))
                .width(260.0)
                .height(400.0)
                .show_ui(ui, |ui| {
                    for u in EnergyUnit::ALL {
                        ui.selectable_value(&mut self.unit, u, tr.t(&format!("unit-{}", u.key())));
                    }
                });
        });

        ui.add_space(8.0);
        kit::panel(ui, pal, Some(&tr.t("energy-make-own")), |ui| {
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
            ui.label(
                RichText::new(tr.ta(
                    "energy-own-energy",
                    &args!["energy" => format_energy(tr, self.unit, self.my_joules())],
                ))
                .size(18.0)
                .color(pal.maths),
            );
        });

        ui.add_space(10.0);
        let rows = self.rows(tr);
        ladder_chart(ui, cx, &rows, |j| format_energy(tr, self.unit, j));
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(tr.t("energy-grid")).color(pal.weak));
            cx.link(ui, "powers-of-ten", "hook-what-is-1e9");
        });

        ui.add_space(10.0);
        self.comparison(ui, cx, &rows);

        ui.add_space(10.0);
        kit::model_alert(ui, pal, tr, &tr.t("energy-model"));

        ui.add_space(8.0);
        kit::grown_ups(ui, pal, tr, "energy-notes", |ui| {
            ui.label(tr.t("energy-notes-intro"));
            ui.add_space(4.0);
            egui::Grid::new("energy-notes-grid")
                .striped(true)
                .show(ui, |ui| {
                    for ex in examples() {
                        let key = format!("energy-ex-{}", ex.id);
                        ui.label(tr.t(&key));
                        ui.label(tr.num(&format!("{} J", sci(ex.joules, 3))));
                        ui.label(RichText::new(tr.attr(&key, "note")).color(pal.weak));
                        ui.end_row();
                    }
                });
        });
    }
}

impl EnergyLadder {
    fn comparison(&mut self, ui: &mut egui::Ui, cx: &Cx, rows: &[Row]) {
        let (tr, pal) = (cx.tr, cx.pal);
        let name_of = |id: &str| {
            rows.iter()
                .find(|r| r.id == id)
                .map(|r| r.name.clone())
                .unwrap_or_default()
        };
        kit::panel(ui, pal, Some(&tr.t("energy-compare")), |ui| {
            ui.horizontal_wrapped(|ui| {
                for (salt, sel) in [
                    ("cmp-a", &mut self.compare_a),
                    ("cmp-b", &mut self.compare_b),
                ] {
                    egui::ComboBox::from_id_salt(salt)
                        .selected_text(name_of(sel))
                        .width(280.0)
                        .show_ui(ui, |ui| {
                            for r in rows {
                                ui.selectable_value(sel, r.id.to_string(), &r.name);
                            }
                        });
                }
            });
            let get = |id: &str| rows.iter().find(|r| r.id == id);
            if let (Some(a), Some(b)) = (get(&self.compare_a), get(&self.compare_b)) {
                let (big, small) = if a.joules >= b.joules { (a, b) } else { (b, a) };
                let ratio = big.joules / small.joules;
                let orders = ratio.log10();
                ui.label(
                    RichText::new(tr.ta(
                        "energy-ratio",
                        &args![
                            "big" => big.name.clone(),
                            "small" => lower_first(&small.name),
                            "ratio" => tr.num(&sci(ratio, 3)),
                        ],
                    ))
                    .size(17.0),
                );
                if orders >= 1.0 {
                    let rounded = (orders * 10.0).round() / 10.0;
                    ui.label(tr.ta(
                        "energy-orders",
                        &args![
                            "orders" => tr.num(&format!("{rounded:.1}")),
                            "n" => rounded,
                            "pow" => format!("10{}", superscript(orders.round() as i32)),
                            "times" => orders.round() as i64,
                        ],
                    ));
                }
            }
        });
    }
}

/// Horizontal log-scale bars, one row per example.
fn ladder_chart(ui: &mut egui::Ui, cx: &Cx, rows: &[Row], fmt: impl Fn(f64) -> String) {
    let pal = cx.pal;
    let lo = rows
        .first()
        .map(|r| r.joules.log10().floor())
        .unwrap_or(-22.0);
    let hi = rows.last().map(|r| r.joules.log10().ceil()).unwrap_or(14.0);
    let row_h = 28.0;
    let label_w = (ui.available_width() * 0.32).clamp(140.0, 320.0);
    let font = egui::FontId::proportional(15.0);
    let values: Vec<String> = rows.iter().map(|r| fmt(r.joules)).collect();
    let value_w = values
        .iter()
        .map(|v| {
            ui.painter()
                .layout_no_wrap(v.clone(), font.clone(), pal.text)
                .size()
                .x
        })
        .fold(80.0, f32::max)
        + 12.0;
    let (resp, painter) = ui.allocate_painter(
        vec2(ui.available_width(), row_h * rows.len() as f32 + 24.0),
        Sense::hover(),
    );
    let r = resp.rect;
    let bar_x0 = r.left() + label_w + 10.0;
    let bar_x1 = (r.right() - value_w - 8.0).max(bar_x0 + 40.0);
    let x_of = |j: f64| bar_x0 + ((j.log10() - lo) / (hi - lo)) as f32 * (bar_x1 - bar_x0);

    // Decade grid: one line per ×10, labelled every 10³.
    let grid = Stroke::new(1.0, pal.line);
    let mut d = lo as i32;
    while d as f64 <= hi {
        let x = x_of(10f64.powi(d));
        painter.line_segment(
            [egui::pos2(x, r.top()), egui::pos2(x, r.bottom() - 20.0)],
            grid,
        );
        if d % 3 == 0 {
            painter.text(
                egui::pos2(x, r.bottom()),
                egui::Align2::CENTER_BOTTOM,
                format!("10{} J", superscript(d)),
                egui::FontId::proportional(12.0),
                pal.weak,
            );
        }
        d += 1;
    }

    for (i, row) in rows.iter().enumerate() {
        let y = r.top() + i as f32 * row_h;
        let mid = y + row_h / 2.0;
        let mine = row.id == MY_ID;
        let color = if mine { pal.maths } else { pal.scale };
        painter.text(
            egui::pos2(r.left() + label_w, mid),
            egui::Align2::RIGHT_CENTER,
            &row.name,
            font.clone(),
            if mine { color } else { pal.text },
        );
        let bar = egui::Rect::from_min_max(
            egui::pos2(bar_x0, y + 5.0),
            egui::pos2(x_of(row.joules).max(bar_x0 + 6.0), y + row_h - 5.0),
        );
        let fill = if mine {
            color
        } else {
            // Bars get warmer as energy grows.
            let t = ((row.joules.log10() - lo) / (hi - lo)) as f32;
            crate::theme::lerp(pal.physics, pal.scale, t)
        };
        painter.rect_filled(bar, CornerRadius::same(9), fill);
        painter.text(
            egui::pos2(bar_x1 + 10.0, mid),
            egui::Align2::LEFT_CENTER,
            &values[i],
            font.clone(),
            if mine { color } else { pal.text },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Lang;

    #[test]
    fn every_example_and_unit_has_text() {
        for lang in Lang::ALL {
            let tr = I18n::new(lang);
            for ex in examples() {
                let key = format!("energy-ex-{}", ex.id);
                assert!(tr.has(&key, None), "{}: {key}", lang.code());
                assert!(tr.has(&key, Some("note")), "{}: {key}.note", lang.code());
            }
            for u in EnergyUnit::ALL {
                assert!(
                    tr.has(&format!("unit-{}", u.key()), None),
                    "{}: unit-{}",
                    lang.code(),
                    u.key()
                );
                if u.symbol().is_none() {
                    assert!(
                        tr.has(&format!("amount-{}", u.key()), None),
                        "{}: amount-{}",
                        lang.code(),
                        u.key()
                    );
                }
            }
        }
    }

    #[test]
    fn energies_read_naturally() {
        let sk = I18n::new(Lang::Sk);
        let en = I18n::new(Lang::En);
        assert_eq!(format_energy(&en, EnergyUnit::Joule, 66_000.0), "66 kJ");
        assert_eq!(format_energy(&sk, EnergyUnit::Joule, 1.23e13), "12,3 TJ");
        assert_eq!(
            format_energy(&sk, EnergyUnit::SugarCube, 66_000.0),
            "1 kocka cukru"
        );
        assert_eq!(
            format_energy(&sk, EnergyUnit::SugarCube, 2.0 * 66_000.0),
            "2 kocky cukru"
        );
        assert_eq!(
            format_energy(&en, EnergyUnit::SugarCube, 3.4 * 66_000.0),
            "3.4 sugar cubes"
        );
        assert_eq!(
            format_energy(&sk, EnergyUnit::SugarCube, 4.12e-21),
            "6,24 × 10⁻²⁶ kocky cukru"
        );
    }
}
