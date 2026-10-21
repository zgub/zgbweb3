//! Roll more dice, get a bell: the central limit theorem with a histogram.

use egui::{CornerRadius, RichText, Sense, Shape, Stroke, pos2, vec2};
use serde::{Deserialize, Serialize};
use sims::stats::{DiceSums, normal_pdf};

use super::{Cx, Topic, heading, storage_key};
use crate::args;
use crate::kit;

const ID: &str = "dice-bell";

#[derive(Serialize, Deserialize)]
#[serde(default)]
struct Saved {
    dice: u32,
    sides: u32,
    show_curve: bool,
}

impl Default for Saved {
    fn default() -> Self {
        Self {
            dice: 2,
            sides: 6,
            show_curve: true,
        }
    }
}

pub struct CltDice {
    cfg: Saved,
    sim: DiceSums,
    seed: u64,
    auto: bool,
}

impl Default for CltDice {
    fn default() -> Self {
        let cfg = Saved::default();
        Self {
            sim: DiceSums::new(cfg.dice, cfg.sides, 1),
            cfg,
            seed: 1,
            auto: false,
        }
    }
}

impl CltDice {
    fn reset(&mut self) {
        self.seed = self
            .seed
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        self.sim = DiceSums::new(self.cfg.dice, self.cfg.sides, self.seed);
    }
}

impl Topic for CltDice {
    fn id(&self) -> &'static str {
        ID
    }

    fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &storage_key(ID), &self.cfg);
    }

    fn load(&mut self, storage: &dyn eframe::Storage) {
        if let Some(cfg) = eframe::get_value::<Saved>(storage, &storage_key(ID)) {
            self.cfg = cfg;
            self.sim = DiceSums::new(self.cfg.dice, self.cfg.sides, self.seed);
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

        let before = (self.cfg.dice, self.cfg.sides);
        kit::panel(ui, pal, None, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.add(egui::Slider::new(&mut self.cfg.dice, 1..=30).text(tr.t("dice-n")));
                ui.add(egui::Slider::new(&mut self.cfg.sides, 2..=20).text(tr.t("dice-sides")));
            });
            ui.horizontal_wrapped(|ui| {
                for n in [1u64, 10, 100, 10_000] {
                    let label = tr.ta("dice-roll", &args!["n" => n as i64]);
                    if kit::bright_button(ui, pal, egui::RichText::new(label), pal.chance).clicked()
                    {
                        self.sim.roll(n);
                    }
                }
                ui.toggle_value(&mut self.auto, tr.t("dice-keep"));
                if ui.button(tr.t("dice-restart")).clicked() {
                    self.reset();
                }
                ui.checkbox(&mut self.cfg.show_curve, tr.t("dice-curve"));
            });
        });
        if before != (self.cfg.dice, self.cfg.sides) {
            self.reset();
        }
        if self.auto {
            self.sim.roll(50);
            ui.ctx().request_repaint();
        }

        ui.add_space(8.0);
        let h = (ui.available_height() - 140.0).clamp(240.0, 480.0);
        histogram(ui, cx, &self.sim, self.cfg.show_curve, h);

        ui.add_space(4.0);
        let observed = self
            .sim
            .observed_mean()
            .map(|m| tr.num(&format!("{m:.2}")))
            .unwrap_or_else(|| "–".into());
        ui.label(
            RichText::new(tr.ta(
                "dice-summary",
                &args![
                    "rolls" => self.sim.rolls() as i64,
                    "mean" => observed,
                    "theory" => tr.num(&format!("{:.2}", self.sim.mean())),
                ],
            ))
            .size(17.0),
        );

        ui.add_space(8.0);
        kit::grown_ups(ui, pal, tr, "dice-notes", |ui| {
            ui.label(tr.ta(
                "dice-notes",
                &args![
                    "mean" => tr.num(&format!("{:.2}", self.sim.mean())),
                    "sd" => tr.num(&format!("{:.2}", self.sim.std_dev())),
                ],
            ));
        });
    }
}

fn histogram(ui: &mut egui::Ui, cx: &Cx, sim: &DiceSums, curve: bool, height: f32) {
    let pal = cx.pal;
    let (resp, painter) = ui.allocate_painter(vec2(ui.available_width(), height), Sense::hover());
    painter.rect_filled(resp.rect, CornerRadius::same(16), pal.card);
    let r = resp.rect.shrink2(vec2(14.0, 22.0));
    let counts = sim.counts();
    let n = counts.len();
    let expected_peak = sim.rolls() as f64 * normal_pdf(sim.mean(), sim.mean(), sim.std_dev());
    let max_c = counts.iter().copied().max().unwrap_or(0) as f64;
    let top = max_c.max(if curve { expected_peak } else { 0.0 }).max(1.0) * 1.08;
    let bw = r.width() / n as f32;
    let y_of = |v: f64| r.bottom() - (v / top) as f32 * r.height();

    painter.line_segment(
        [r.left_bottom(), r.right_bottom()],
        Stroke::new(1.0, pal.line),
    );
    for (k, &c) in counts.iter().enumerate() {
        let x0 = r.left() + k as f32 * bw;
        let bar = egui::Rect::from_min_max(
            pos2(x0 + bw * 0.12, y_of(c as f64)),
            pos2(x0 + bw * 0.88, r.bottom()),
        );
        let radius = (bw * 0.25).min(8.0) as u8;
        painter.rect_filled(
            bar,
            CornerRadius {
                nw: radius,
                ne: radius,
                sw: 0,
                se: 0,
            },
            pal.chance,
        );
        // Label sums when there's room, else every few.
        let label_every = ((28.0 / bw).ceil() as usize).max(1);
        if k % label_every == 0 {
            painter.text(
                pos2(x0 + bw / 2.0, r.bottom() + 3.0),
                egui::Align2::CENTER_TOP,
                (sim.min_sum() as usize + k).to_string(),
                egui::FontId::proportional(12.0),
                pal.weak,
            );
        }
    }

    if curve && sim.rolls() > 0 {
        let steps = 200;
        let lo = sim.min_sum() as f64 - 0.5;
        let hi = sim.max_sum() as f64 + 0.5;
        let pts: Vec<_> = (0..=steps)
            .map(|i| {
                let s = lo + (hi - lo) * i as f64 / steps as f64;
                let x = r.left() + ((s - lo) / (hi - lo)) as f32 * r.width();
                pos2(
                    x,
                    y_of(sim.rolls() as f64 * normal_pdf(s, sim.mean(), sim.std_dev())),
                )
            })
            .collect();
        painter.add(Shape::line(pts, Stroke::new(3.0, pal.scale)));
    }

    if sim.rolls() == 0 {
        painter.text(
            r.center(),
            egui::Align2::CENTER_CENTER,
            format!("🎲  {}", cx.tr.t("dice-empty")),
            crate::theme::heading_font(26.0),
            pal.weak,
        );
    }
}
