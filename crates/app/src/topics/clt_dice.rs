//! Roll more dice, get a bell: the central limit theorem with a histogram.

use egui::{Color32, Sense, Shape, Stroke, pos2, vec2};
use serde::{Deserialize, Serialize};
use sims::stats::{DiceSums, normal_pdf};

use super::{Group, Maturity, Nav, Related, Topic, TopicMeta, storage_key, widgets};

static META: TopicMeta = TopicMeta {
    id: "dice-bell",
    title: "Why dice make a bell",
    icon: "🎲",
    group: Group::Chance,
    blurb: "One die is flat and fair. Add up two, five, twenty dice and a bell curve appears out of nowhere.",
    maturity: Maturity::Draft,
    related: &[Related {
        id: "gas-box",
        hook: "Billions of molecules, one bell-shaped speed curve",
    }],
};

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
    fn meta(&self) -> &'static TopicMeta {
        &META
    }

    fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &storage_key(META.id), &self.cfg);
    }

    fn load(&mut self, storage: &dyn eframe::Storage) {
        if let Some(cfg) = eframe::get_value::<Saved>(storage, &storage_key(META.id)) {
            self.cfg = cfg;
            self.sim = DiceSums::new(self.cfg.dice, self.cfg.sides, self.seed);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _nav: &mut Nav) {
        widgets::heading(ui, &META);

        let before = (self.cfg.dice, self.cfg.sides);
        ui.horizontal_wrapped(|ui| {
            ui.add(egui::Slider::new(&mut self.cfg.dice, 1..=30).text("dice"));
            ui.add(egui::Slider::new(&mut self.cfg.sides, 2..=20).text("sides each"));
        });
        if before != (self.cfg.dice, self.cfg.sides) {
            self.reset();
        }

        ui.horizontal_wrapped(|ui| {
            for n in [1u64, 10, 100, 10_000] {
                if ui.button(format!("Roll ×{n}")).clicked() {
                    self.sim.roll(n);
                }
            }
            ui.toggle_value(&mut self.auto, "▶ Keep rolling");
            if ui.button("⟲ Start over").clicked() {
                self.reset();
            }
            ui.checkbox(&mut self.cfg.show_curve, "Show the bell (Gauss) curve");
        });
        if self.auto {
            self.sim.roll(50);
            ui.ctx().request_repaint();
        }

        ui.add_space(6.0);
        let h = (ui.available_height() - 120.0).clamp(220.0, 480.0);
        histogram(ui, &self.sim, self.cfg.show_curve, h);

        ui.add_space(4.0);
        let observed = self
            .sim
            .observed_mean()
            .map(|m| format!("{m:.2}"))
            .unwrap_or_else(|| "–".into());
        ui.label(format!(
            "{} rolls. Average sum: {} (theory says {:.2}).",
            self.sim.rolls(),
            observed,
            self.sim.mean()
        ));

        widgets::grown_ups(ui, "dice-notes", |ui| {
            ui.label(format!(
                "Sum of n fair s-sided dice: mean n(s+1)/2 = {:.2}, standard deviation √(n(s²−1)/12) = {:.2}. \
                 The curve is the normal density with that mean and spread, scaled by the number of rolls. \
                 The central limit theorem says the sum of many independent random things tends to this \
                 shape, whatever the shape of each one.",
                self.sim.mean(),
                self.sim.std_dev()
            ));
        });
    }
}

fn histogram(ui: &mut egui::Ui, sim: &DiceSums, curve: bool, height: f32) {
    let (resp, painter) = ui.allocate_painter(vec2(ui.available_width(), height), Sense::hover());
    let r = resp.rect.shrink2(vec2(8.0, 18.0));
    let vis = ui.visuals();
    let counts = sim.counts();
    let n = counts.len();
    let expected_peak = sim.rolls() as f64 * normal_pdf(sim.mean(), sim.mean(), sim.std_dev());
    let max_c = counts.iter().copied().max().unwrap_or(0) as f64;
    let top = max_c.max(if curve { expected_peak } else { 0.0 }).max(1.0) * 1.08;
    let bw = r.width() / n as f32;
    let y_of = |v: f64| r.bottom() - (v / top) as f32 * r.height();

    painter.line_segment(
        [r.left_bottom(), r.right_bottom()],
        Stroke::new(1.0, vis.weak_text_color()),
    );
    for (k, &c) in counts.iter().enumerate() {
        let x0 = r.left() + k as f32 * bw;
        let bar = egui::Rect::from_min_max(
            pos2(x0 + bw * 0.1, y_of(c as f64)),
            pos2(x0 + bw * 0.9, r.bottom()),
        );
        painter.rect_filled(bar, 2.0, vis.selection.bg_fill);
        // Label sums when there's room, else every few.
        let label_every = ((28.0 / bw).ceil() as usize).max(1);
        if k % label_every == 0 {
            painter.text(
                pos2(x0 + bw / 2.0, r.bottom() + 2.0),
                egui::Align2::CENTER_TOP,
                (sim.min_sum() as usize + k).to_string(),
                egui::FontId::proportional(11.0),
                vis.weak_text_color(),
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
        painter.add(Shape::line(
            pts,
            Stroke::new(2.0, Color32::from_rgb(0xe0, 0x8a, 0x3c)),
        ));
    }

    if sim.rolls() == 0 {
        painter.text(
            r.center(),
            egui::Align2::CENTER_CENTER,
            "Roll some dice!",
            egui::FontId::proportional(18.0),
            vis.weak_text_color(),
        );
    }
}
