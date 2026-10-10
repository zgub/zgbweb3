//! Powers of ten: one little cube, ten in a row, a hundred as a slab, a thousand
//! as a big cube — and then the same three steps again with the big cube as the
//! new "one".
//!
//! v0 draws with the egui painter in isometric 2D. The planned wgpu version
//! (rotate, fly through, zoom across scales) replaces `draw_*` only.

use egui::{Color32, Pos2, RichText, Sense, Shape, Stroke, Vec2, vec2};
use serde::{Deserialize, Serialize};
use sims::fmt::{SI_PREFIXES, length, prefix_for_exponent, superscript};

use super::{Group, Maturity, Nav, Related, Topic, TopicMeta, storage_key, widgets};

static META: TopicMeta = TopicMeta {
    id: "powers-of-ten",
    title: "Powers of ten",
    icon: "🔟",
    group: Group::ScaleAndNumbers,
    blurb: "One cube, ten in a row, a hundred in a slab, a thousand in a big cube. Then do it again — and again.",
    maturity: Maturity::Draft,
    related: &[
        Related {
            id: "energy-ladder",
            hook: "What is 10¹³ joules? Meet the energy ladder",
        },
        Related {
            id: "numbers",
            hook: "0.001 — what kind of number is that?",
        },
    ],
};

const MIN_EXP: i32 = -18;
const MAX_EXP: i32 = 18;

#[derive(Serialize, Deserialize)]
#[serde(default)]
#[derive(Default)]
pub struct PowersOfTen {
    exponent: i32,
}

impl Topic for PowersOfTen {
    fn meta(&self) -> &'static TopicMeta {
        &META
    }

    fn save(&self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &storage_key(META.id), self);
    }

    fn load(&mut self, storage: &dyn eframe::Storage) {
        if let Some(s) = eframe::get_value(storage, &storage_key(META.id)) {
            *self = s;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, nav: &mut Nav) {
        widgets::heading(ui, &META);

        ui.horizontal(|ui| {
            if ui.button("◀ ÷10").clicked() {
                self.exponent = (self.exponent - 1).max(MIN_EXP);
            }
            ui.add(
                egui::Slider::new(&mut self.exponent, MIN_EXP..=MAX_EXP)
                    .show_value(false)
                    .text(""),
            );
            if ui.button("×10 ▶").clicked() {
                self.exponent = (self.exponent + 1).min(MAX_EXP);
            }
        });

        let e = self.exponent;
        ui.add_space(6.0);
        ui.label(RichText::new(headline(e)).size(22.0).strong());
        ui.label(RichText::new(number_name(e)).size(16.0));
        ui.add_space(6.0);

        let height = (ui.available_height() - 140.0).clamp(220.0, 520.0);
        let (response, painter) =
            ui.allocate_painter(vec2(ui.available_width(), height), Sense::hover());
        let rect = response.rect.shrink(12.0);
        if e >= 0 {
            draw_positive(&painter, rect, e, ui.visuals().dark_mode);
        } else {
            draw_negative(
                &painter,
                rect,
                e,
                ui.visuals().dark_mode,
                ui.visuals().weak_text_color(),
            );
        }

        ui.add_space(4.0);
        ui.label(scale_story(e));
        if e < 0 {
            nav.link(ui, "numbers", "Numbers smaller than one — what are they?");
        }

        ui.add_space(8.0);
        egui::CollapsingHeader::new("All the SI prefixes, quecto to quetta")
            .id_salt("pot-prefixes")
            .show(ui, |ui| prefix_table(ui, e));

        widgets::grown_ups(ui, "pot-notes", |ui| {
            ui.label(
                "Every third step is a new SI prefix (kilo, mega, giga…). The picture repeats the \
                 same three shapes — one block, a rod of ten, a slab of a hundred — and the block \
                 itself grows 10× in every direction each time the pattern restarts. Grid lines on \
                 a block show it is made of 10×10×10 of the previous blocks.",
            );
            ui.label(
                "Naming: this page uses the short scale (billion = 10⁹). Slovak and most of Europe \
                 use the long scale (miliarda = 10⁹, bilión = 10¹²).",
            );
        });
    }
}

fn headline(e: i32) -> String {
    let plain = if (0..=9).contains(&e) {
        // 1 000 000 with thin-ish spaces for readability
        let s = 10u64.pow(e as u32).to_string();
        group_digits(&s)
    } else if (-6..0).contains(&e) {
        format!("0.{}1", "0".repeat((-e - 1) as usize))
    } else {
        String::new()
    };
    let pow = format!("10{}", superscript(e));
    match (plain.is_empty(), prefix_for_exponent(e)) {
        (false, Some(p)) if e != 0 => format!("{pow} = {plain}   ({} · {})", p.name, p.symbol),
        (false, _) => format!("{pow} = {plain}"),
        (true, Some(p)) => format!("{pow}   ({} · {})", p.name, p.symbol),
        (true, None) => pow,
    }
}

fn group_digits(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

/// Short-scale English names.
fn number_name(e: i32) -> String {
    const BIG: [&str; 7] = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];
    const SMALL: [&str; 3] = ["one", "ten", "one hundred"];
    if e >= 0 {
        let (k, r) = (e / 3, e % 3);
        let lead = SMALL[r as usize];
        if k == 0 {
            lead.to_string()
        } else {
            format!("{lead} {}", BIG[k as usize])
        }
    } else {
        // 10⁻ⁿ = "one <n-th>": tenth, hundredth, thousandth, ten-thousandth, …
        let n = -e;
        let (k, r) = (n / 3, n % 3);
        let unit = [
            "",
            "thousandth",
            "millionth",
            "billionth",
            "trillionth",
            "quadrillionth",
            "quintillionth",
        ];
        let name = match (r, k) {
            (1, 0) => "tenth".to_string(),
            (2, 0) => "hundredth".to_string(),
            (0, k) => unit[k as usize].to_string(),
            (1, k) => format!("ten-{}", unit[k as usize]),
            (_, k) => format!("hundred-{}", unit[k as usize]),
        };
        format!("one {name}")
    }
}

/// "If each little cube were a 1 cm sugar cube…"
fn scale_story(e: i32) -> String {
    if e >= 0 {
        let side_m = 0.01 * 10f64.powf(e as f64 / 3.0);
        let mut s = format!(
            "If every little cube were a 1 cm sugar cube, {} of them would fill a cube {} on each side",
            if e == 0 {
                "one".to_string()
            } else {
                format!("10{}", superscript(e))
            },
            length(side_m, 3)
        );
        if e % 3 == 0 {
            s.push_str(&format!(" — about the size of {}.", size_hint(e / 3 - 2)));
        } else {
            s.push('.');
        }
        s
    } else {
        let side_m = 10f64.powf(e as f64 / 3.0);
        let mut s = format!(
            "Cut a 1 m cube into 10{} equal little cubes: each one is {} on a side",
            superscript(-e),
            length(side_m, 3)
        );
        if e % 3 == 0 {
            s.push_str(&format!(" — about the size of {}.", size_hint(e / 3)));
        } else {
            s.push('.');
        }
        s
    }
}

/// Something about 10^p metres across.
fn size_hint(p: i32) -> &'static str {
    match p {
        ..=-6 => "a bacterium",
        -5 => "a cell in your body",
        -4 => "the thickness of a hair",
        -3 => "a grain of salt",
        -2 => "a sugar cube",
        -1 => "a big mug",
        0 => "a cube you could sit inside",
        1 => "a three-storey house",
        2 => "a 30-storey tower block",
        3 => "a hill",
        _ => "something taller than Mount Everest",
    }
}

fn prefix_table(ui: &mut egui::Ui, e: i32) {
    egui::Grid::new("pot-prefix-grid")
        .striped(true)
        .show(ui, |ui| {
            ui.strong("power");
            ui.strong("prefix");
            ui.strong("symbol");
            ui.end_row();
            for p in SI_PREFIXES.iter().rev() {
                let current = p.exponent == e;
                let txt = |s: String| {
                    let r = RichText::new(s);
                    if current { r.strong().underline() } else { r }
                };
                ui.label(txt(format!("10{}", superscript(p.exponent))));
                ui.label(txt(if p.name.is_empty() {
                    "—".into()
                } else {
                    p.name.into()
                }));
                ui.label(txt(p.symbol.to_string()));
                ui.end_row();
            }
        });
}

// ---------------------------------------------------------------- drawing ---

const COS30: f32 = 0.866_025_4;

/// Isometric projection of block-space (x, y, z) to screen offset in units of one block.
fn iso(x: f32, y: f32, z: f32) -> Vec2 {
    vec2((x - y) * COS30, (x + y) * 0.5 - z)
}

#[derive(Clone, Copy)]
struct BoxStyle {
    top: Color32,
    left: Color32,
    right: Color32,
    edge: Stroke,
    /// Draw a 10×10 grid on each face (the block is made of smaller blocks).
    grid: bool,
}

fn level_style(level: i32, dark: bool, grid: bool) -> BoxStyle {
    // A hue per "level" (every 10³), so the new "one" looks new.
    const HUES: [(u8, u8, u8); 7] = [
        (0x4f, 0x9d, 0xde),
        (0xe0, 0x8a, 0x3c),
        (0x5c, 0xb8, 0x7a),
        (0xb0, 0x6c, 0xd9),
        (0xd9, 0x5c, 0x6c),
        (0xd9, 0xc2, 0x4f),
        (0x4f, 0xc8, 0xc8),
    ];
    let (r, g, b) = HUES[(level.rem_euclid(HUES.len() as i32)) as usize];
    let shade = |f: f32| {
        Color32::from_rgb(
            (r as f32 * f) as u8,
            (g as f32 * f) as u8,
            (b as f32 * f) as u8,
        )
    };
    BoxStyle {
        top: shade(1.0),
        left: shade(0.78),
        right: shade(0.58),
        edge: Stroke::new(
            1.0,
            if dark {
                Color32::from_black_alpha(160)
            } else {
                Color32::from_black_alpha(90)
            },
        ),
        grid,
    }
}

/// Fit a set of axis-aligned boxes (block units) into `rect`, return (origin, px per block).
fn fit(rect: egui::Rect, extent: (f32, f32, f32)) -> (Pos2, f32) {
    let (nx, ny, nz) = extent;
    let corners = [
        iso(0.0, 0.0, 0.0),
        iso(nx, 0.0, 0.0),
        iso(0.0, ny, 0.0),
        iso(nx, ny, 0.0),
        iso(0.0, 0.0, nz),
        iso(nx, 0.0, nz),
        iso(0.0, ny, nz),
        iso(nx, ny, nz),
    ];
    let (mut min, mut max) = (corners[0], corners[0]);
    for c in corners {
        min = min.min(c);
        max = max.max(c);
    }
    let size = (rect.width() / (max.x - min.x))
        .min(rect.height() / (max.y - min.y))
        .min(160.0);
    let drawn = (max - min) * size;
    let origin = rect.center() - drawn / 2.0 - min * size;
    (origin, size)
}

fn draw_box(
    painter: &egui::Painter,
    origin: Pos2,
    size: f32,
    p: [f32; 3],
    d: [f32; 3],
    st: BoxStyle,
) {
    let [x, y, z] = p;
    let [dx, dy, dz] = d;
    let at = |x: f32, y: f32, z: f32| origin + iso(x, y, z) * size;

    let top = [
        at(x, y, z + dz),
        at(x + dx, y, z + dz),
        at(x + dx, y + dy, z + dz),
        at(x, y + dy, z + dz),
    ];
    let left = [
        at(x, y + dy, z),
        at(x + dx, y + dy, z),
        at(x + dx, y + dy, z + dz),
        at(x, y + dy, z + dz),
    ];
    let right = [
        at(x + dx, y, z),
        at(x + dx, y + dy, z),
        at(x + dx, y + dy, z + dz),
        at(x + dx, y, z + dz),
    ];

    for (face, fill) in [(top, st.top), (left, st.left), (right, st.right)] {
        painter.add(Shape::convex_polygon(face.to_vec(), fill, st.edge));
        if st.grid && size * dx.max(dy).max(dz) > 60.0 {
            let thin = Stroke::new(0.5, st.edge.color.gamma_multiply(0.6));
            for i in 1..10 {
                let t = i as f32 / 10.0;
                painter.line_segment([face[0].lerp(face[1], t), face[3].lerp(face[2], t)], thin);
                painter.line_segment([face[0].lerp(face[3], t), face[1].lerp(face[2], t)], thin);
            }
        }
    }
}

/// 10^e as blocks: e = 3k + r. A block is a cube of 10^k little cubes per side;
/// r picks the arrangement: one block, a rod of 10, a slab of 10×10.
fn draw_positive(painter: &egui::Painter, rect: egui::Rect, e: i32, dark: bool) {
    let (k, r) = (e / 3, e % 3);
    let (nx, ny) = match r {
        0 => (1, 1),
        1 => (10, 1),
        _ => (10, 10),
    };
    let (origin, size) = fit(rect, (nx as f32, ny as f32, 1.0));
    let style = level_style(k, dark, k > 0);
    for x in 0..nx {
        for y in 0..ny {
            draw_box(
                painter,
                origin,
                size,
                [x as f32, y as f32, 0.0],
                [1.0, 1.0, 1.0],
                style,
            );
        }
    }
}

/// 10^-n: the unit cube, faint, with one piece highlighted (slab, stick, tiny cube),
/// then the cycle repeats inside the tiny cube.
fn draw_negative(painter: &egui::Painter, rect: egui::Rect, e: i32, dark: bool, weak: Color32) {
    let n = -e;
    let (k, r) = ((n - 1) / 3, (n - 1) % 3 + 1); // r = 1..=3
    let (origin, size) = fit(rect, (1.0, 1.0, 1.0));
    let mut whole = level_style(-k, dark, false);
    for c in [&mut whole.top, &mut whole.left, &mut whole.right] {
        *c = c.gamma_multiply(0.25);
    }
    draw_box(painter, origin, size, [0.0; 3], [1.0; 3], whole);
    let piece = match r {
        1 => ([0.9, 0.0, 0.0], [0.1, 1.0, 1.0]),
        2 => ([0.9, 0.9, 0.0], [0.1, 0.1, 1.0]),
        _ => ([0.9, 0.9, 0.9], [0.1, 0.1, 0.1]),
    };
    draw_box(
        painter,
        origin,
        size,
        piece.0,
        piece.1,
        level_style(-k - 1, dark, false),
    );
    if k > 0 {
        painter.text(
            rect.left_top(),
            egui::Align2::LEFT_TOP,
            format!(
                "(the big faint cube is already 10{} of the original)",
                superscript(-3 * k)
            ),
            egui::FontId::proportional(13.0),
            weak,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(number_name(0), "one");
        assert_eq!(number_name(2), "one hundred");
        assert_eq!(number_name(4), "ten thousand");
        assert_eq!(number_name(9), "one billion");
        assert_eq!(number_name(-1), "one tenth");
        assert_eq!(number_name(-3), "one thousandth");
        assert_eq!(number_name(-4), "one ten-thousandth");
        assert_eq!(number_name(-18), "one quintillionth");
    }

    #[test]
    fn every_exponent_has_a_name_and_headline() {
        for e in MIN_EXP..=MAX_EXP {
            assert!(!number_name(e).is_empty());
            assert!(headline(e).starts_with("10"));
        }
    }

    #[test]
    fn digits_grouped() {
        assert_eq!(group_digits("1000000"), "1 000 000");
        assert_eq!(group_digits("100"), "100");
    }
}
