//! Look and feel: the old zgbweb style (DECISIONS D-018).
//!
//! Stock egui visuals and egui's bundled fonts, slightly larger text for reading,
//! nothing else in the chrome. Colour lives inside the simulations: the accents
//! below are for drawing sims and charts, not for menus or headings.

use egui::{Color32, FontDefinitions, FontFamily, FontId, TextStyle, Theme};

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
    /// Grouped boxes inside a page.
    pub card: Color32,
    pub line: Color32,
    pub text: Color32,
    pub weak: Color32,
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

pub const DARK: Palette = Palette {
    card: gray(35),
    line: gray(60),
    text: gray(210),
    weak: gray(140),
    scale: rgb(0xe0a64a),
    chance: rgb(0xa98be0),
    physics: rgb(0x4cc4b6),
    maths: rgb(0xe8808c),
    stickers: STICKERS,
    cube_body: rgb(0x111318),
};

pub const LIGHT: Palette = Palette {
    card: gray(238),
    line: gray(200),
    text: gray(40),
    weak: gray(110),
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

/// Install fonts and text sizes for both themes. Call once at start-up.
pub fn install(ctx: &egui::Context) {
    ctx.set_fonts(fonts());
    for theme in [Theme::Dark, Theme::Light] {
        ctx.style_mut_of(theme, |style| {
            use TextStyle::*;
            style.text_styles = [
                (Small, FontId::proportional(11.5)),
                (Body, FontId::proportional(14.5)),
                (Button, FontId::proportional(14.5)),
                (Monospace, FontId::monospace(13.0)),
                (Heading, FontId::proportional(22.0)),
            ]
            .into();
        });
    }
}
