//! Look and feel: colours, fonts, widget styling.
//!
//! Direction: playful but not childish. Deep indigo night (dark) or warm paper
//! (light), one bright accent colour per subject area, rounded shapes, a
//! friendly rounded typeface (Nunito) in two weights.

use egui::epaint::text::{FontTweak, VariationCoords};
use egui::{
    Color32, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Margin, Shadow, Stroke,
    TextStyle, Theme, Visuals, style::Selection, style::WidgetVisuals,
};

use crate::topics::Group;

const NUNITO: &[u8] = include_bytes!("../assets/fonts/Nunito-Subset.ttf");

pub const FAMILY_HEADING: &str = "heading";
pub const FAMILY_BOLD: &str = "bold";

pub fn heading_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FAMILY_HEADING.into()))
}

pub fn bold_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(FAMILY_BOLD.into()))
}

/// Colours that are not part of egui's `Visuals`.
#[derive(Clone, Copy)]
pub struct Palette {
    pub dark: bool,
    /// Page background (central area).
    pub bg: Color32,
    /// Top bar, menu, footer.
    pub chrome: Color32,
    /// Cards and grouped boxes.
    pub card: Color32,
    pub line: Color32,
    pub text: Color32,
    pub weak: Color32,
    pub scale: Color32,
    pub chance: Color32,
    pub physics: Color32,
    pub maths: Color32,
    /// Text/icons drawn on a solid accent fill.
    pub on_accent: Color32,
    /// Puzzle-cube sticker colours, and the plastic body.
    pub stickers: [Color32; 6],
    pub cube_body: Color32,
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
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
    dark: true,
    bg: rgb(0x161a30),
    chrome: rgb(0x1e2342),
    card: rgb(0x232a4f),
    line: rgb(0x343c6b),
    text: rgb(0xeceef9),
    weak: rgb(0xa3a9cf),
    scale: rgb(0xffb547),
    chance: rgb(0xb98cff),
    physics: rgb(0x3fd6c6),
    maths: rgb(0xff7b8a),
    on_accent: rgb(0x14172b),
    stickers: STICKERS,
    cube_body: rgb(0x111318),
};

pub const LIGHT: Palette = Palette {
    dark: false,
    bg: rgb(0xfbf6ec),
    chrome: rgb(0xf3eadb),
    card: rgb(0xfffdf8),
    line: rgb(0xe3d6bf),
    text: rgb(0x2b2540),
    weak: rgb(0x6e6788),
    scale: rgb(0xd98200),
    chance: rgb(0x7c4ddb),
    physics: rgb(0x00897b),
    maths: rgb(0xd9485f),
    on_accent: rgb(0xffffff),
    stickers: STICKERS,
    cube_body: rgb(0x1d1d22),
};

pub fn palette(dark: bool) -> &'static Palette {
    if dark { &DARK } else { &LIGHT }
}

impl Palette {
    pub fn group(&self, g: Group) -> Color32 {
        match g {
            Group::ScaleAndNumbers => self.scale,
            Group::Chance => self.chance,
            Group::Physics => self.physics,
            Group::Maths => self.maths,
        }
    }

    /// A soft tint of `c` over the card colour. Keep amounts small on dark
    /// backgrounds: warm colours over navy turn muddy above ~0.15.
    pub fn tint(&self, c: Color32, amount: f32) -> Color32 {
        lerp(self.card, c, amount)
    }
}

pub fn lerp(a: Color32, b: Color32, t: f32) -> Color32 {
    let f = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t).round() as u8;
    Color32::from_rgb(f(a.r(), b.r()), f(a.g(), b.g()), f(a.b(), b.b()))
}

/// Install fonts and both light and dark styles. Call once at start-up.
pub fn install(ctx: &egui::Context) {
    ctx.set_fonts(fonts());
    for (theme, pal) in [(Theme::Dark, &DARK), (Theme::Light, &LIGHT)] {
        ctx.set_visuals_of(theme, visuals(pal));
        ctx.style_mut_of(theme, |style| {
            use TextStyle::*;
            style.text_styles = [
                (Small, FontId::proportional(13.0)),
                (Body, FontId::proportional(16.5)),
                (Button, FontId::proportional(16.0)),
                (Monospace, FontId::monospace(14.0)),
                (Heading, heading_font(30.0)),
            ]
            .into();
            let s = &mut style.spacing;
            s.item_spacing = egui::vec2(10.0, 8.0);
            s.button_padding = egui::vec2(12.0, 5.0);
            s.interact_size.y = 26.0;
            s.slider_width = 170.0;
            s.slider_rail_height = 6.0;
            s.menu_margin = Margin::same(10);
            s.window_margin = Margin::same(12);
        });
    }
}

fn fonts() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let weight = |w: f32| {
        FontData::from_static(NUNITO).tweak(FontTweak {
            coords: VariationCoords::new([(*b"wght", w)]),
            ..Default::default()
        })
    };
    fonts
        .font_data
        .insert("nunito-body".into(), weight(520.0).into());
    fonts
        .font_data
        .insert("nunito-bold".into(), weight(760.0).into());
    fonts
        .font_data
        .insert("nunito-heading".into(), weight(880.0).into());

    // Emoji/icons come from egui's bundled fonts; Hack covers →, ⁻ and a few more.
    let fallbacks = ["NotoEmoji-Regular", "emoji-icon-font", "Hack"].map(String::from);
    let family = |first: &str| {
        std::iter::once(first.to_owned())
            .chain(fallbacks.iter().cloned())
            .collect::<Vec<_>>()
    };
    fonts
        .families
        .insert(FontFamily::Proportional, family("nunito-body"));
    fonts
        .families
        .insert(FontFamily::Name(FAMILY_BOLD.into()), family("nunito-bold"));
    fonts.families.insert(
        FontFamily::Name(FAMILY_HEADING.into()),
        family("nunito-heading"),
    );
    fonts
}

fn visuals(p: &Palette) -> Visuals {
    let mut v = if p.dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    let r = CornerRadius::same(10);
    let (btn, btn_hover, btn_active) = if p.dark {
        (rgb(0x2b3263), rgb(0x3a4380), rgb(0x4a55a0))
    } else {
        (rgb(0xefe4d0), rgb(0xe6d6ba), rgb(0xdac59f))
    };
    let accent = if p.dark { rgb(0x6b7bff) } else { rgb(0x4b5cf0) };

    v.panel_fill = p.bg;
    v.window_fill = p.card;
    v.faint_bg_color = p.card;
    v.extreme_bg_color = if p.dark { rgb(0x0f1226) } else { rgb(0xefe6d6) };
    v.code_bg_color = v.extreme_bg_color;
    v.hyperlink_color = if p.dark { rgb(0xffc75e) } else { rgb(0xb4560a) };
    v.weak_text_color = Some(p.weak);
    v.window_corner_radius = CornerRadius::same(14);
    v.menu_corner_radius = CornerRadius::same(12);
    v.window_stroke = Stroke::new(1.0, p.line);
    v.popup_shadow = Shadow {
        offset: [0, 6],
        blur: 18,
        spread: 0,
        color: Color32::from_black_alpha(if p.dark { 110 } else { 40 }),
    };
    v.window_shadow = v.popup_shadow;
    v.slider_trailing_fill = true;
    v.selection = Selection {
        bg_fill: accent,
        stroke: Stroke::new(1.5, Color32::WHITE),
    };

    let w = |fill: Color32, stroke: Stroke, text: Color32, text_w: f32| WidgetVisuals {
        bg_fill: fill,
        weak_bg_fill: fill,
        bg_stroke: stroke,
        fg_stroke: Stroke::new(text_w, text),
        corner_radius: r,
        expansion: 0.0,
    };
    v.widgets.noninteractive = w(p.bg, Stroke::new(1.0, p.line), p.text, 1.0);
    v.widgets.inactive = w(btn, Stroke::NONE, p.text, 1.0);
    v.widgets.hovered = w(btn_hover, Stroke::new(1.0, accent), p.text, 1.5);
    v.widgets.active = w(btn_active, Stroke::new(1.5, accent), p.text, 2.0);
    v.widgets.open = w(btn, Stroke::new(1.0, p.line), p.text, 1.0);
    v
}
