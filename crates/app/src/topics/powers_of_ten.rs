//! Powers of ten, built from puzzle cubes.
//!
//! One cube, ten in a row (exactly 1 m, because each cube sits in its own 10 cm
//! box), a hundred in a 1 m slab, a thousand in a 1 m cube — then that big cube
//! becomes the new "one" and the pattern repeats. Negative powers cut a 1 m cube
//! into slabs, sticks and little cubes.
//!
//! Drawn with the egui painter in isometric projection. Geometry and numbers
//! come from `sims::scale`; this file only draws and words them.

use std::f32::consts::FRAC_PI_2;

use egui::epaint::TextShape;
use egui::{Align2, Color32, Pos2, RichText, Sense, Shape, Stroke, Vec2, vec2};
use serde::{Deserialize, Serialize};
use sims::fmt::{SI_PREFIXES, group_thousands, length, mass, prefix_for_exponent, superscript};
use sims::scale::{Arrangement, CUBE_EDGE_M, GAP_M, KID_HEIGHT_M, PITCH_M, Shape as Layout};

use super::{Cx, Topic, heading, storage_key};
use crate::args;
use crate::i18n::I18n;
use crate::kit;
use crate::theme::{Palette, bold_font, heading_font, lerp};

const ID: &str = "powers-of-ten";

const MIN_EXP: i32 = -18;
const MAX_EXP: i32 = 18;

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PowersOfTen {
    exponent: i32,
}

impl Topic for PowersOfTen {
    fn id(&self) -> &'static str {
        ID
    }

    fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &storage_key(ID), self);
    }

    fn load(&mut self, storage: &dyn eframe::Storage) {
        if let Some(s) = eframe::get_value(storage, &storage_key(ID)) {
            *self = s;
        }
    }

    /// `#/powers-of-ten/9` opens at 10⁹.
    fn set_param(&mut self, param: &str) {
        if let Ok(e) = param.parse::<i32>() {
            self.exponent = e.clamp(MIN_EXP, MAX_EXP);
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

        // Controls
        ui.horizontal(|ui| {
            let big = |s: &str| RichText::new(s.to_string()).font(bold_font(18.0));
            if kit::bright_button(ui, pal, big("◀  ÷10"), pal.scale).clicked() {
                self.exponent = (self.exponent - 1).max(MIN_EXP);
            }
            ui.spacing_mut().slider_width = (ui.available_width() - 140.0).clamp(120.0, 420.0);
            ui.add(egui::Slider::new(&mut self.exponent, MIN_EXP..=MAX_EXP).show_value(false));
            if kit::bright_button(ui, pal, big("×10  ▶"), pal.scale).clicked() {
                self.exponent = (self.exponent + 1).min(MAX_EXP);
            }
        });

        let e = self.exponent;
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.label(
                RichText::new(headline(tr, e))
                    .font(heading_font(34.0))
                    .color(pal.scale),
            );
            if let Some(p) = prefix_for_exponent(e).filter(|_| e != 0) {
                kit::pill(
                    ui,
                    pal,
                    format!("{} · {}", tr.t(&si_key(p.exponent)), p.symbol),
                    pal.scale,
                );
            }
        });
        ui.label(RichText::new(tr.t(&name_key(e))).size(20.0));
        ui.add_space(6.0);

        if e >= 4 {
            legend(ui, cx, Arrangement::new(e as u32).level + 1);
        }

        let height = (ui.available_height() - 150.0).clamp(300.0, 560.0);
        let (response, painter) =
            ui.allocate_painter(vec2(ui.available_width(), height), Sense::hover());
        painter.rect_filled(response.rect, 16, pal.card);
        let rect = response.rect.shrink(48.0);
        if e >= 0 {
            draw_positive(&painter, rect, e as u32, pal, tr);
        } else {
            draw_negative(&painter, rect, -e as u32, pal, tr);
        }

        ui.add_space(8.0);
        ui.label(RichText::new(story(tr, e)).size(17.5));
        if e < 0 {
            cx.link(ui, "number-bases", "hook-what-kind-of-number");
        } else {
            ui.add_space(6.0);
            kit::model_alert(ui, pal, tr, &tr.t("pot-model"));
        }

        ui.add_space(8.0);
        egui::CollapsingHeader::new(tr.t("pot-prefix-table"))
            .id_salt("pot-prefixes")
            .show(ui, |ui| prefix_table(ui, tr, pal, e));

        kit::grown_ups(ui, pal, tr, "pot-notes", |ui| {
            for line in tr.t("pot-notes").lines() {
                ui.label(line.trim());
            }
        });
    }
}

fn si_key(exponent: i32) -> String {
    if exponent < 0 {
        format!("si-m{}", -exponent)
    } else {
        format!("si-p{exponent}")
    }
}

fn name_key(e: i32) -> String {
    if e < 0 {
        format!("pot-name-m{}", -e)
    } else {
        format!("pot-name-p{e}")
    }
}

/// "10⁹ = 1 000 000 000", "10⁻³ = 0,001", or just "10¹⁵" when the digits get silly.
fn headline(tr: &I18n, e: i32) -> String {
    let pow = format!("10{}", superscript(e));
    let plain = match e {
        0..=12 => group_thousands(&10u64.pow(e as u32).to_string()),
        -6..=-1 => tr.num(&format!("0.{}1", "0".repeat((-e - 1) as usize))),
        _ => return pow,
    };
    format!("{pow} = {plain}")
}

/// The count of cubes as words would show it: "1 000 000" or "10¹⁵".
fn count_text(e: u32) -> String {
    if e <= 12 {
        group_thousands(&10u64.pow(e).to_string())
    } else {
        format!("10{}", superscript(e as i32))
    }
}

fn story(tr: &I18n, e: i32) -> String {
    if e == 0 {
        return tr.ta(
            "pot-story-one",
            &args![
                "cube" => tr.num(&length(CUBE_EDGE_M, 2)),
                "box" => tr.num(&length(PITCH_M, 2)),
                "gap" => tr.num(&length(GAP_M, 2)),
            ],
        );
    }
    if e > 0 {
        let a = Arrangement::new(e as u32);
        let (x, _, _) = a.extent_m();
        let shape = match a.shape() {
            Layout::Row => "row",
            Layout::Slab => "slab",
            Layout::Cube => "cube",
        };
        let mut s = tr.ta(
            "pot-story",
            &args![
                "count" => count_text(e as u32),
                "shape" => shape,
                "len" => tr.num(&length(x, 3)),
            ],
        );
        if a.shape() == Layout::Cube {
            s.push(' ');
            s += &tr.ta(
                "pot-hint",
                &args!["thing" => tr.t(&format!("size-hint-k{}", a.level))],
            );
        }
        s.push(' ');
        s += &tr.ta(
            "pot-weight",
            &args!["mass" => tr.num(&mass(a.mass_kg(), 2))],
        );
        return s;
    }
    let n = -e as u32;
    let (k, r) = ((n - 1) / 3, (n - 1) % 3 + 1);
    let big = 10f64.powi(-(k as i32));
    let thin = big / 10.0;
    let shape = ["slab", "stick", "cube"][r as usize - 1];
    let mut s = tr.ta(
        "pot-story-neg",
        &args![
            "count" => count_text(n),
            "shape" => shape,
            "big" => tr.num(&length(big, 3)),
            "thin" => tr.num(&length(thin, 3)),
        ],
    );
    if r == 3 {
        s.push(' ');
        s += &tr.ta(
            "pot-hint",
            &args!["thing" => tr.t(&format!("size-hint-neg{n}"))],
        );
    }
    s
}

fn prefix_table(ui: &mut egui::Ui, tr: &I18n, pal: &Palette, e: i32) {
    egui::Grid::new("pot-prefix-grid")
        .striped(true)
        .spacing(vec2(24.0, 4.0))
        .show(ui, |ui| {
            ui.strong(tr.t("pot-col-power"));
            ui.strong(tr.t("pot-col-prefix"));
            ui.strong(tr.t("pot-col-symbol"));
            ui.end_row();
            for p in SI_PREFIXES.iter().rev() {
                let current = p.exponent == e;
                let txt = |s: String| {
                    let r = RichText::new(s);
                    if current {
                        r.color(pal.scale).font(bold_font(16.5))
                    } else {
                        r
                    }
                };
                ui.label(txt(format!("10{}", superscript(p.exponent))));
                ui.label(txt(if p.exponent == 0 {
                    "—".into()
                } else {
                    tr.t(&si_key(p.exponent))
                }));
                ui.label(txt(p.symbol.to_string()));
                ui.end_row();
            }
        });
}

/// Colour key for blocks: a puzzle cube, then one colour per 10³ step.
fn legend(ui: &mut egui::Ui, cx: &Cx, max_level: u32) {
    let (tr, pal) = (cx.tr, cx.pal);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(tr.t("pot-legend")).color(pal.weak));
        for level in 0..max_level {
            let (rect, _) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
            if level == 0 {
                let h = rect.width() / 2.0;
                for (i, c) in [0, 2, 4, 1].iter().enumerate() {
                    let min = rect.min + vec2((i % 2) as f32 * h, (i / 2) as f32 * h);
                    ui.painter().rect_filled(
                        egui::Rect::from_min_size(min, vec2(h - 1.0, h - 1.0)),
                        2,
                        pal.stickers[*c],
                    );
                }
                ui.label(tr.ta(
                    "pot-legend-cube",
                    &args!["side" => tr.num(&length(CUBE_EDGE_M, 2))],
                ));
            } else {
                ui.painter().rect_filled(rect, 4, level_color(pal, level));
                ui.label(tr.ta(
                    "pot-legend-block",
                    &args![
                        "count" => count_text(3 * level),
                        "side" => tr.num(&length(PITCH_M * 10f64.powi(level as i32), 2)),
                    ],
                ));
            }
            ui.add_space(8.0);
        }
    });
}

// ---------------------------------------------------------------- drawing ---

const COS30: f32 = 0.866_025_4;

/// Isometric projection: world metres → screen points.
#[derive(Clone, Copy)]
struct Iso {
    origin: Pos2,
    /// Screen points per metre.
    s: f32,
}

fn iso_unit(x: f64, y: f64, z: f64) -> Vec2 {
    vec2(((x - y) * COS30 as f64) as f32, ((x + y) * 0.5 - z) as f32)
}

impl Iso {
    /// Fit world points into `rect`.
    fn fit(rect: egui::Rect, points: &[(f64, f64, f64)]) -> Self {
        let mut min = vec2(f32::MAX, f32::MAX);
        let mut max = vec2(f32::MIN, f32::MIN);
        for &(x, y, z) in points {
            let p = iso_unit(x, y, z);
            min = min.min(p);
            max = max.max(p);
        }
        let size = (max - min).max(vec2(1e-9, 1e-9));
        let s = (rect.width() / size.x).min(rect.height() / size.y);
        let origin = rect.center() - (max - min) * s / 2.0 - min * s;
        Self { origin, s }
    }

    fn p(&self, x: f64, y: f64, z: f64) -> Pos2 {
        self.origin + iso_unit(x, y, z) * self.s
    }
}

fn shade(c: Color32, f: f32) -> Color32 {
    let m = |v: u8| (v as f32 * f).clamp(0.0, 255.0) as u8;
    Color32::from_rgb(m(c.r()), m(c.g()), m(c.b()))
}

fn hash(a: u64) -> u64 {
    let mut x = a.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

const FACE_SHADE: [f32; 3] = [1.0, 0.87, 0.74];

/// The three visible faces of an axis-aligned box: top, front-left (y+), front-right (x+).
/// Each as 4 corners: [origin, along u, opposite, along v].
fn faces(iso: &Iso, p: [f64; 3], d: [f64; 3]) -> [[Pos2; 4]; 3] {
    let [x, y, z] = p;
    let [dx, dy, dz] = d;
    let at = |x, y, z| iso.p(x, y, z);
    [
        [
            at(x, y, z + dz),
            at(x + dx, y, z + dz),
            at(x + dx, y + dy, z + dz),
            at(x, y + dy, z + dz),
        ],
        [
            at(x, y + dy, z + dz),
            at(x + dx, y + dy, z + dz),
            at(x + dx, y + dy, z),
            at(x, y + dy, z),
        ],
        [
            at(x + dx, y, z + dz),
            at(x + dx, y + dy, z + dz),
            at(x + dx, y + dy, z),
            at(x + dx, y, z),
        ],
    ]
}

/// Point on a parallelogram face at (u, v) ∈ [0,1]².
fn on_face(f: &[Pos2; 4], u: f32, v: f32) -> Pos2 {
    f[0] + (f[1] - f[0]) * u + (f[3] - f[0]) * v
}

/// Fill a face with an n×n grid of tiles, each coloured by `color(i, j)`.
fn tiles(
    painter: &egui::Painter,
    f: &[Pos2; 4],
    n: usize,
    inset: f32,
    color: impl Fn(usize, usize) -> Color32,
) {
    let step = 1.0 / n as f32;
    for i in 0..n {
        for j in 0..n {
            let (u0, v0) = (
                i as f32 * step + inset * step,
                j as f32 * step + inset * step,
            );
            let (u1, v1) = (
                (i + 1) as f32 * step - inset * step,
                (j + 1) as f32 * step - inset * step,
            );
            let quad = vec![
                on_face(f, u0, v0),
                on_face(f, u1, v0),
                on_face(f, u1, v1),
                on_face(f, u0, v1),
            ];
            painter.add(Shape::convex_polygon(quad, color(i, j), Stroke::NONE));
        }
    }
}

/// One puzzle cube: dark plastic body, 3×3 stickers per visible face (when big enough).
fn draw_puzzle_cube(painter: &egui::Painter, iso: &Iso, pal: &Palette, pos: [f64; 3], seed: u64) {
    let d = [CUBE_EDGE_M; 3];
    let px = (CUBE_EDGE_M as f32) * iso.s;
    for (fi, face) in faces(iso, pos, d).iter().enumerate() {
        painter.add(Shape::convex_polygon(
            face.to_vec(),
            pal.cube_body,
            Stroke::NONE,
        ));
        let sticker = |i: usize, j: usize| {
            let h = hash(seed ^ ((fi as u64) << 40) ^ ((i as u64) << 20) ^ j as u64);
            shade(pal.stickers[(h % 6) as usize], FACE_SHADE[fi])
        };
        if px >= 14.0 {
            tiles(painter, face, 3, 0.09, sticker);
        } else if px >= 3.0 {
            tiles(painter, face, 1, 0.06, |_, _| sticker(0, 0));
        }
    }
}

/// Colour for a block of 10^(3·level) cubes.
fn level_color(pal: &Palette, level: u32) -> Color32 {
    const EXTRA: [Color32; 2] = [
        Color32::from_rgb(0x5b, 0x8c, 0xff),
        Color32::from_rgb(0x5c, 0xc9, 0x6b),
    ];
    match level {
        0 => pal.stickers[0],
        1 => pal.scale,
        2 => pal.chance,
        3 => pal.physics,
        4 => pal.maths,
        5 => EXTRA[0],
        _ => EXTRA[1],
    }
}

/// A block of 10^(3·level) cubes (level ≥ 1) seen from outside: its faces show a
/// 10×10 grid of the blocks one level down (single cubes → sticker colours).
#[allow(clippy::too_many_arguments)]
fn draw_block(
    painter: &egui::Painter,
    iso: &Iso,
    pal: &Palette,
    pos: [f64; 3],
    side: f64,
    level: u32,
    seed: u64,
    visible: [bool; 3],
) {
    let d = [side; 3];
    let tile_px = (side / 10.0) as f32 * iso.s;
    let body = if level == 1 {
        pal.cube_body
    } else {
        shade(level_color(pal, level - 1), 0.35)
    };
    for (fi, face) in faces(iso, pos, d).iter().enumerate() {
        if !visible[fi] {
            continue;
        }
        if tile_px >= 6.0 {
            painter.add(Shape::convex_polygon(face.to_vec(), body, Stroke::NONE));
            tiles(painter, face, 10, 0.12, |i, j| {
                let h = hash(seed ^ ((fi as u64) << 40) ^ ((i as u64) << 20) ^ j as u64);
                let c = if level == 1 {
                    pal.stickers[(h % 6) as usize]
                } else {
                    shade(level_color(pal, level - 1), 0.9 + (h % 7) as f32 * 0.03)
                };
                shade(c, FACE_SHADE[fi])
            });
        } else {
            let c = shade(level_color(pal, level), FACE_SHADE[fi]);
            painter.add(Shape::convex_polygon(
                face.to_vec(),
                c,
                Stroke::new(0.6, shade(c, 0.6)),
            ));
        }
    }
}

/// Dimension line between screen points a→b, pushed `out` (unit vector) away from
/// the object, with an optional ruler of `ticks` divisions and a rotated label.
#[allow(clippy::too_many_arguments)]
fn dimension(
    painter: &egui::Painter,
    pal: &Palette,
    a: Pos2,
    b: Pos2,
    out: Vec2,
    dist: f32,
    ticks: usize,
    label: &str,
) {
    let color = pal.scale;
    let thin = Stroke::new(1.0, pal.tint(color, 0.6));
    let a2 = a + out * dist;
    let b2 = b + out * dist;
    painter.line_segment([a + out * 4.0, a2 + out * 5.0], thin);
    painter.line_segment([b + out * 4.0, b2 + out * 5.0], thin);
    painter.line_segment([a2, b2], Stroke::new(2.0, color));
    painter.circle_filled(a2, 3.0, color);
    painter.circle_filled(b2, 3.0, color);
    let len = (b2 - a2).length();
    if ticks > 1 && len / ticks as f32 >= 6.0 {
        for i in 1..ticks {
            let p = a2.lerp(b2, i as f32 / ticks as f32);
            painter.line_segment([p - out * 3.5, p + out * 3.5], Stroke::new(1.5, color));
        }
    }
    let galley = painter.layout_no_wrap(label.to_owned(), bold_font(15.0), color);
    let dir = b2 - a2;
    let mut angle = dir.y.atan2(dir.x);
    if angle > FRAC_PI_2 {
        angle -= std::f32::consts::PI;
    } else if angle < -FRAC_PI_2 {
        angle += std::f32::consts::PI;
    }
    let center = a2.lerp(b2, 0.5) + out * (galley.size().y / 2.0 + 5.0);
    let pos = center - galley.size() / 2.0;
    painter.add(
        TextShape::new(pos, galley, color).with_angle_and_anchor(angle, Align2::CENTER_CENTER),
    );
}

// Outward directions (screen) for the three edges we label.
const OUT_FRONT_LEFT: Vec2 = vec2(-0.5, COS30); // under the x-edge at y = max
const OUT_FRONT_RIGHT: Vec2 = vec2(0.5, COS30); // under the y-edge at x = max
const OUT_RIGHT: Vec2 = vec2(1.0, 0.0); // right of the vertical edge at (x max, y 0)
const OUT_BACK: Vec2 = vec2(0.5, -COS30); // above the x-edge at y = 0, top

fn draw_kid(painter: &egui::Painter, pal: &Palette, tr: &I18n, foot: Pos2, h: f32) {
    let ink = pal.weak;
    let w = |f: f32| Stroke::new((h * f).max(1.0), ink);
    let at = |dx: f32, dy: f32| foot + vec2(dx * h, -dy * h);
    painter.line_segment([at(-0.05, 0.47), at(-0.08, 0.0)], w(0.08));
    painter.line_segment([at(0.05, 0.47), at(0.08, 0.0)], w(0.08));
    painter.line_segment([at(-0.12, 0.78), at(-0.2, 0.5)], w(0.06));
    painter.line_segment([at(0.12, 0.78), at(0.2, 0.5)], w(0.06));
    let shirt = egui::Rect::from_min_max(at(-0.13, 0.82), at(0.13, 0.44));
    painter.rect_filled(shirt, (h * 0.06) as u8, pal.maths);
    painter.circle_filled(at(0.0, 0.91), h * 0.09, ink);
    if h >= 24.0 {
        painter.text(
            at(0.0, 1.03),
            Align2::CENTER_BOTTOM,
            tr.ta("pot-kid", &args!["h" => tr.num(&length(KID_HEIGHT_M, 2))]),
            egui::FontId::proportional(13.0),
            pal.weak,
        );
    }
}

fn draw_positive(painter: &egui::Painter, rect: egui::Rect, e: u32, pal: &Palette, tr: &I18n) {
    let a = Arrangement::new(e);
    let (x, y, z) = a.extent_m();
    let b = a.block_box_m;
    let (nx, ny) = a.blocks;
    let m = |v: f64| tr.num(&length(v, 3));

    // The kid stands just left of the front-left corner when the scene is kid-sized.
    let biggest = x.max(z);
    let kid = (2..=6).contains(&e).then_some((
        -0.5 * KID_HEIGHT_M.min(biggest),
        y + 0.4 * KID_HEIGHT_M.min(biggest),
    ));
    let mut pts = vec![
        (0.0, 0.0, 0.0),
        (x, 0.0, 0.0),
        (0.0, y, 0.0),
        (x, y, 0.0),
        (0.0, 0.0, z),
        (x, 0.0, z),
        (0.0, y, z),
        (x, y, z),
    ];
    if let Some((kx, ky)) = kid {
        pts.extend([
            (kx - 0.3, ky + 0.3, 0.0),
            (kx, ky, KID_HEIGHT_M * 1.12),
            (kx + 0.3, ky - 0.3, 0.0),
        ]);
    }
    let iso = Iso::fit(rect, &pts);

    if e == 0 {
        // The invisible box, dashed.
        let c = |x, y, z| iso.p(x, y, z);
        let p = PITCH_M;
        let edges = [
            [(0., 0., 0.), (p, 0., 0.)],
            [(p, 0., 0.), (p, p, 0.)],
            [(p, p, 0.), (0., p, 0.)],
            [(0., p, 0.), (0., 0., 0.)],
            [(0., 0., p), (p, 0., p)],
            [(p, 0., p), (p, p, p)],
            [(p, p, p), (0., p, p)],
            [(0., p, p), (0., 0., p)],
            [(0., 0., 0.), (0., 0., p)],
            [(p, 0., 0.), (p, 0., p)],
            [(p, p, 0.), (p, p, p)],
            [(0., p, 0.), (0., p, p)],
        ];
        for [s, t] in edges {
            painter.extend(Shape::dashed_line(
                &[c(s.0, s.1, s.2), c(t.0, t.1, t.2)],
                Stroke::new(1.2, pal.weak),
                6.0,
                5.0,
            ));
        }
    }

    // Blocks back to front.
    let mut order: Vec<(u32, u32)> = (0..nx).flat_map(|i| (0..ny).map(move |j| (i, j))).collect();
    order.sort_by_key(|&(i, j)| i + j);
    let g2 = GAP_M / 2.0;
    if a.level == 0 {
        for (i, j) in order {
            let pos = [i as f64 * b + g2, j as f64 * b + g2, g2];
            draw_puzzle_cube(painter, &iso, pal, pos, ((i as u64) << 8) | j as u64);
        }
    } else if e == 3 {
        // The 1 m cube: all 1000 puzzle cubes, back to front.
        let mut cubes: Vec<(u32, u32, u32)> = (0..10)
            .flat_map(|i| (0..10).flat_map(move |j| (0..10).map(move |l| (i, j, l))))
            .collect();
        cubes.sort_by_key(|&(i, j, l)| i + j + l);
        for (i, j, l) in cubes {
            let pos = [
                i as f64 * PITCH_M + g2,
                j as f64 * PITCH_M + g2,
                l as f64 * PITCH_M + g2,
            ];
            draw_puzzle_cube(
                painter,
                &iso,
                pal,
                pos,
                ((i as u64) << 16) | ((j as u64) << 8) | l as u64,
            );
        }
    } else {
        for (i, j) in order {
            let pos = [i as f64 * b + g2, j as f64 * b + g2, g2];
            // Only outer faces show; inner side faces are hidden by neighbours.
            let visible = [true, j == ny - 1, i == nx - 1];
            draw_block(
                painter,
                &iso,
                pal,
                pos,
                b - GAP_M,
                a.level,
                ((i as u64) << 8) | j as u64,
                visible,
            );
        }
    }

    // Dimensions.
    let ticks = 10;
    match (e, a.shape()) {
        (0, _) => {
            let (c0, c1) = (g2, g2 + CUBE_EDGE_M);
            dimension(
                painter,
                pal,
                iso.p(c0, c1, g2),
                iso.p(c1, c1, g2),
                OUT_FRONT_LEFT,
                34.0,
                0,
                &m(CUBE_EDGE_M),
            );
            dimension(
                painter,
                pal,
                iso.p(PITCH_M, 0.0, 0.0),
                iso.p(PITCH_M, PITCH_M, 0.0),
                OUT_FRONT_RIGHT,
                22.0,
                0,
                &m(PITCH_M),
            );
        }
        (1, _) => {
            // One cube and one gap, then the 1 m ruler on top.
            let yf = g2 + CUBE_EDGE_M;
            dimension(
                painter,
                pal,
                iso.p(g2, yf, g2),
                iso.p(g2 + CUBE_EDGE_M, yf, g2),
                OUT_FRONT_LEFT,
                20.0,
                0,
                &m(CUBE_EDGE_M),
            );
            dimension(
                painter,
                pal,
                iso.p(g2 + CUBE_EDGE_M, yf, g2),
                iso.p(PITCH_M + g2, yf, g2),
                OUT_FRONT_LEFT,
                52.0,
                0,
                &m(GAP_M),
            );
            dimension(
                painter,
                pal,
                iso.p(0.0, 0.0, z),
                iso.p(x, 0.0, z),
                OUT_BACK,
                18.0,
                ticks,
                &m(x),
            );
        }
        (_, Layout::Row) => {
            dimension(
                painter,
                pal,
                iso.p(0.0, y, 0.0),
                iso.p(x, y, 0.0),
                OUT_FRONT_LEFT,
                22.0,
                ticks,
                &m(x),
            );
            dimension(
                painter,
                pal,
                iso.p(x, 0.0, 0.0),
                iso.p(x, 0.0, z),
                OUT_RIGHT,
                18.0,
                0,
                &m(z),
            );
        }
        (_, Layout::Slab) => {
            dimension(
                painter,
                pal,
                iso.p(0.0, y, 0.0),
                iso.p(x, y, 0.0),
                OUT_FRONT_LEFT,
                22.0,
                ticks,
                &m(x),
            );
            dimension(
                painter,
                pal,
                iso.p(x, 0.0, 0.0),
                iso.p(x, y, 0.0),
                OUT_FRONT_RIGHT,
                22.0,
                ticks,
                &m(y),
            );
        }
        (_, Layout::Cube) => {
            let t = if e == 3 { ticks } else { 10 };
            dimension(
                painter,
                pal,
                iso.p(0.0, y, 0.0),
                iso.p(x, y, 0.0),
                OUT_FRONT_LEFT,
                22.0,
                t,
                &m(x),
            );
            dimension(
                painter,
                pal,
                iso.p(x, 0.0, 0.0),
                iso.p(x, y, 0.0),
                OUT_FRONT_RIGHT,
                22.0,
                t,
                &m(y),
            );
            dimension(
                painter,
                pal,
                iso.p(x, 0.0, 0.0),
                iso.p(x, 0.0, z),
                OUT_RIGHT,
                22.0,
                t,
                &m(z),
            );
        }
    }

    if let Some((kx, ky)) = kid {
        let h = KID_HEIGHT_M as f32 * iso.s;
        if h >= 10.0 {
            draw_kid(painter, pal, tr, iso.p(kx, ky, 0.0), h);
        }
    }
}

/// 10^-n of a 1 m cube: the cube faint, with one piece (slab, stick, little cube) lit.
/// Every third step the lit little cube becomes the new faint cube.
fn draw_negative(painter: &egui::Painter, rect: egui::Rect, n: u32, pal: &Palette, tr: &I18n) {
    let (k, r) = ((n - 1) / 3, (n - 1) % 3 + 1);
    let iso = Iso::fit(
        rect,
        &[
            (0.0, 0.0, 0.0),
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (1.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (1.0, 0.0, 1.0),
            (0.0, 1.0, 1.0),
            (1.0, 1.0, 1.0),
        ],
    );
    let ghost = lerp(pal.card, level_color(pal, k + 1), 0.28);
    for (fi, face) in faces(&iso, [0.0; 3], [1.0; 3]).iter().enumerate() {
        let c = shade(ghost, FACE_SHADE[fi]);
        painter.add(Shape::convex_polygon(
            face.to_vec(),
            c,
            Stroke::new(1.0, pal.line),
        ));
        // Show where the cuts go.
        let lines = Stroke::new(0.7, pal.tint(pal.weak, 0.5));
        for i in 1..10 {
            let t = i as f32 / 10.0;
            painter.line_segment([on_face(face, t, 0.0), on_face(face, t, 1.0)], lines);
            painter.line_segment([on_face(face, 0.0, t), on_face(face, 1.0, t)], lines);
        }
    }
    let (p, d) = match r {
        1 => ([0.9, 0.0, 0.0], [0.1, 1.0, 1.0]),
        2 => ([0.9, 0.9, 0.0], [0.1, 0.1, 1.0]),
        _ => ([0.9, 0.9, 0.9], [0.1, 0.1, 0.1]),
    };
    let lit = level_color(pal, k);
    let lit = if k == 0 { pal.scale } else { lit };
    for (fi, face) in faces(&iso, p, d).iter().enumerate() {
        let c = shade(lit, FACE_SHADE[fi]);
        painter.add(Shape::convex_polygon(
            face.to_vec(),
            c,
            Stroke::new(1.0, shade(c, 0.6)),
        ));
    }
    let big = 10f64.powi(-(k as i32));
    dimension(
        painter,
        pal,
        iso.p(0.0, 1.0, 0.0),
        iso.p(1.0, 1.0, 0.0),
        OUT_FRONT_LEFT,
        22.0,
        10,
        &tr.num(&length(big, 3)),
    );
    if k > 0 {
        painter.text(
            rect.left_top() - vec2(30.0, 30.0),
            Align2::LEFT_TOP,
            tr.ta("pot-neg-zoomed", &args!["size" => tr.num(&length(big, 3))]),
            egui::FontId::proportional(14.0),
            pal.weak,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{I18n, Lang};

    #[test]
    fn every_exponent_has_words_in_every_language() {
        for lang in Lang::ALL {
            let tr = I18n::new(lang);
            for e in MIN_EXP..=MAX_EXP {
                assert!(
                    tr.has(&name_key(e), None),
                    "{}: {}",
                    lang.code(),
                    name_key(e)
                );
                let s = story(&tr, e);
                assert!(!s.contains('['), "{}: story({e}) = {s}", lang.code());
            }
            for p in SI_PREFIXES.iter().filter(|p| p.exponent != 0) {
                assert!(
                    tr.has(&si_key(p.exponent), None),
                    "{}: {}",
                    lang.code(),
                    si_key(p.exponent)
                );
            }
        }
    }

    #[test]
    fn headlines() {
        let en = I18n::new(Lang::En);
        let sk = I18n::new(Lang::Sk);
        assert_eq!(headline(&en, 0), "10⁰ = 1");
        assert_eq!(headline(&en, 6), "10⁶ = 1\u{2009}000\u{2009}000");
        assert_eq!(headline(&sk, -3), "10⁻³ = 0,001");
        assert_eq!(headline(&en, 15), "10¹⁵");
    }
}
