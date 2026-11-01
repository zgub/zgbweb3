//! Look and feel: the old zgbweb layout (DECISIONS D-018) in the Ember palette (D-034).
//!
//! egui's bundled fonts, slightly larger text for reading. Dark mode is Ember:
//! warm charcoal panels, one amber accent for selection and links. Light mode is
//! stock egui light until it gets its own warm palette (Q-027). Most colour still
//! lives inside the simulations: `series` and `ramp` below are for drawing.

use egui::{Color32, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Theme, Visuals};

/// Kept as a function so topics need not care which font family is used.
/// egui's bundled Ubuntu-Light has one weight; emphasis comes from size and colour.
pub fn heading_font(size: f32) -> FontId {
    FontId::proportional(size)
}

pub fn bold_font(size: f32) -> FontId {
    FontId::proportional(size)
}

/// Colours that are not part of egui's `Visuals`.
#[derive(Clone, Copy)]
pub struct Palette {
    /// Panels: menu, top bar, page.
    pub bg: Color32,
    /// Behind canvases and text fields, a step darker than `bg`.
    pub deep: Color32,
    /// Grouped boxes inside a page.
    pub card: Color32,
    pub line: Color32,
    pub text: Color32,
    pub strong: Color32,
    pub weak: Color32,
    /// The one accent of the chrome: selection, links, the known value in pictures.
    pub accent: Color32,
    /// 🐄 model alerts and other callouts.
    pub note: Color32,
    /// Chart series colours, in order of use: amber, teal, coral, sky, plum, olive gold.
    pub series: [Color32; 6],
    /// Sequential ramp for speed, heat, height: low → high.
    pub ramp: [Color32; 5],
    /// Accents for drawing inside sims (bars, curves, highlights).
    pub scale: Color32,
    pub chance: Color32,
    pub physics: Color32,
    pub maths: Color32,
    /// Puzzle-cube sticker colours, and the plastic body.
    pub stickers: [Color32; 6],
    pub cube_body: Color32,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

const fn gray(v: u8) -> Color32 {
    Color32::from_rgb(v, v, v)
}

const STICKERS: [Color32; 6] = [
    rgb(0xf4f1e8), // white
    rgb(0xffd23f), // yellow
    rgb(0xe63946), // red
    rgb(0xff8c1a), // orange
    rgb(0x2a6fdb), // blue
    rgb(0x2bb24c), // green
];

/// Ember (D-034).
pub const DARK: Palette = Palette {
    bg: rgb(0x1d1a17),
    deep: rgb(0x141210),
    card: rgb(0x27231f),
    line: rgb(0x3b352f),
    text: rgb(0xcfc6b8),
    strong: rgb(0xf3ece1),
    weak: rgb(0x8f8577),
    accent: rgb(0xf0a04b),
    note: rgb(0xe8b04f),
    series: [
        rgb(0xf0a04b),
        rgb(0x4fb3a2),
        rgb(0xe06c5a),
        rgb(0x6aa6d6),
        rgb(0xb58ad0),
        rgb(0xc9b458),
    ],
    ramp: [
        rgb(0x2a1b38),
        rgb(0x7a2a4f),
        rgb(0xc8463d),
        rgb(0xef8a32),
        rgb(0xf8d66d),
    ],
    scale: rgb(0xe0a64a),
    chance: rgb(0xa98be0),
    physics: rgb(0x4cc4b6),
    maths: rgb(0xe8808c),
    stickers: STICKERS,
    cube_body: rgb(0x111318),
};

/// Stock egui light with darker series colours that read on white (Q-027).
pub const LIGHT: Palette = Palette {
    bg: gray(248),
    deep: gray(255),
    card: gray(238),
    line: gray(200),
    text: gray(40),
    strong: gray(0),
    weak: gray(110),
    accent: rgb(0xb5651d),
    note: rgb(0x9a6a00),
    series: [
        rgb(0xc8741f),
        rgb(0x1f8a7a),
        rgb(0xc0483a),
        rgb(0x2f6fae),
        rgb(0x8a55b0),
        rgb(0x8f7d1e),
    ],
    ramp: [
        rgb(0x2a1b38),
        rgb(0x7a2a4f),
        rgb(0xc8463d),
        rgb(0xef8a32),
        rgb(0xe8b84a),
    ],
    scale: rgb(0xb07000),
    chance: rgb(0x6a4fc0),
    physics: rgb(0x00897b),
    maths: rgb(0xc0485a),
    stickers: STICKERS,
    cube_body: rgb(0x1d1d22),
};

pub fn palette(dark: bool) -> &'static Palette {
    if dark { &DARK } else { &LIGHT }
}

impl Palette {
    /// Sample the sequential ramp, `t` in 0..=1.
    pub fn ramp_at(&self, t: f32) -> Color32 {
        let t = t.clamp(0.0, 1.0) * 4.0;
        let i = (t.floor() as usize).min(3);
        lerp(self.ramp[i], self.ramp[i + 1], t - i as f32)
    }

    /// A soft tint of `c` over the card colour.
    pub fn tint(&self, c: Color32, amount: f32) -> Color32 {
        lerp(self.card, c, amount)
    }
}

pub fn lerp(a: Color32, b: Color32, t: f32) -> Color32 {
    let f = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(f(a.r(), b.r()), f(a.g(), b.g()), f(a.b(), b.b()))
}

/// egui's bundled fonts, plus Hack as a last fallback for proportional text:
/// it has a few symbols Ubuntu-Light lacks (the superscript minus in 10⁻³, →).
pub fn fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.push("Hack".to_owned());
    }
    fonts
}

/// egui visuals for Ember: stock dark with the palette's colours (D-034).
pub fn ember_visuals() -> Visuals {
    let p = &DARK;
    let mut v = Visuals::dark();
    v.panel_fill = p.bg;
    v.window_fill = p.card;
    v.window_stroke = Stroke::new(1.0, p.line);
    v.extreme_bg_color = p.deep;
    v.faint_bg_color = p.card;
    v.code_bg_color = p.card;
    v.hyperlink_color = p.accent;
    v.warn_fg_color = p.note;
    v.error_fg_color = p.series[2];
    v.selection.bg_fill = lerp(p.bg, p.accent, 0.35);
    v.selection.stroke = Stroke::new(1.0, p.strong);

    let w = &mut v.widgets;
    w.noninteractive.bg_fill = p.bg;
    w.noninteractive.weak_bg_fill = p.bg;
    w.noninteractive.bg_stroke = Stroke::new(1.0, p.line);
    w.noninteractive.fg_stroke = Stroke::new(1.0, p.text);
    let rest = lerp(p.card, p.line, 0.6);
    w.inactive.bg_fill = rest;
    w.inactive.weak_bg_fill = rest;
    w.inactive.bg_stroke = Stroke::NONE;
    w.inactive.fg_stroke = Stroke::new(1.0, p.text);
    let hover = lerp(p.line, p.accent, 0.25);
    w.hovered.bg_fill = hover;
    w.hovered.weak_bg_fill = hover;
    w.hovered.bg_stroke = Stroke::new(1.0, lerp(p.line, p.accent, 0.6));
    w.hovered.fg_stroke = Stroke::new(1.5, p.strong);
    let active = lerp(p.line, p.accent, 0.45);
    w.active.bg_fill = active;
    w.active.weak_bg_fill = active;
    w.active.bg_stroke = Stroke::new(1.0, p.accent);
    w.active.fg_stroke = Stroke::new(2.0, p.strong);
    w.open.bg_fill = p.card;
    w.open.weak_bg_fill = p.card;
    w.open.bg_stroke = Stroke::new(1.0, p.line);
    w.open.fg_stroke = Stroke::new(1.0, p.strong);
    v
}

/// Install fonts, the Ember look and text sizes. Call once at start-up.
pub fn install(ctx: &egui::Context) {
    ctx.set_fonts(fonts());
    ctx.set_visuals_of(Theme::Dark, ember_visuals());
    for theme in [Theme::Dark, Theme::Light] {
        ctx.style_mut_of(theme, |style| {
            use TextStyle::*;
            style.text_styles = [
                (Small, FontId::proportional(12.5)),
                (Body, FontId::proportional(14.5)),
                (Button, FontId::proportional(14.5)),
                (Monospace, FontId::monospace(13.0)),
                (Heading, FontId::proportional(22.0)),
            ]
            .into();
        });
    }
}
