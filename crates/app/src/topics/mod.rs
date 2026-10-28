//! Pages that have their own code. Each one is a [`Topic`].
//!
//! Where a page sits, its glyph, status and cross-links live in the site map
//! (`content::PAGES`); this module only adds behaviour. A page in the map with no
//! [`Topic`] here is drawn as a planned page by `pages::planned`.
//!
//! Adding a topic:
//! 1. put its maths/physics in the `sims` crate (with tests),
//! 2. give it an entry in `content::PAGES` and its texts in both catalogs,
//! 3. add a module here implementing [`Topic`] and register it in [`registry`].

pub mod percent;
pub mod rule_of_three;

use content::PageMeta;
use egui::RichText;

use crate::i18n::I18n;
use crate::theme::Palette;

/// What a topic gets while drawing: translations, colours, and a way to navigate.
pub struct Cx<'a> {
    pub tr: &'a I18n,
    pub pal: &'static Palette,
    /// The visitor asked for less animation.
    #[allow(dead_code)] // honoured by future auto-playing sims
    pub calm: bool,
    go: Option<&'static str>,
}

impl<'a> Cx<'a> {
    pub fn new(tr: &'a I18n, pal: &'static Palette, calm: bool) -> Self {
        Self {
            tr,
            pal,
            calm,
            go: None,
        }
    }

    /// An inline "question →" link to page `id`; `hook` is a catalog key.
    pub fn link(&mut self, ui: &mut egui::Ui, id: &'static str, hook: &str) {
        let r = ui.link(RichText::new(format!("{} →", self.tr.t(hook))).italics());
        if r.on_hover_cursor(egui::CursorIcon::PointingHand).clicked() {
            self.go = Some(id);
        }
    }

    pub fn navigation(&self) -> Option<&'static str> {
        self.go
    }
}

pub trait Topic {
    /// The page id in `content::PAGES`.
    fn id(&self) -> &'static str;

    /// Draw the topic into the central area. Called every frame while visible.
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut Cx);

    /// Persist this topic's state (only called when the visitor allowed storage).
    fn save(&self, _storage: &mut dyn eframe::Storage) {}

    /// Restore state saved by [`Topic::save`].
    fn load(&mut self, _storage: &dyn eframe::Storage) {}

    /// Deep link: the part after `#/<id>/`, e.g. `9` in `#/powers-of-ten/9`.
    fn set_param(&mut self, _param: &str) {}
}

/// Storage key for a topic's state.
pub fn storage_key(id: &str) -> String {
    format!("topic/{id}")
}

/// All pages that have code.
pub fn registry() -> Vec<Box<dyn Topic>> {
    vec![
        Box::new(rule_of_three::RuleOfThree::default()),
        Box::new(percent::Percent::default()),
    ]
}

/// Page title block: glyph and title as a heading, status marker, blurb.
/// Used by topics and by planned pages alike.
pub fn heading(ui: &mut egui::Ui, tr: &I18n, pal: &Palette, meta: &PageMeta) {
    ui.horizontal(|ui| {
        ui.heading(format!("{} {}", meta.glyph, tr.page_title(meta.id)));
        if let Some(key) = meta.status.key() {
            ui.label(RichText::new(tr.t(key)).small().color(pal.weak));
        }
    });
    ui.separator();
    if let Some(blurb) = tr.page_blurb(meta.id) {
        ui.label(RichText::new(blurb).color(pal.weak));
        ui.add_space(6.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::i18n::{I18n, Lang};
    use content::{PAGES, Section, Status};

    #[test]
    fn every_topic_is_on_the_site_map_and_not_planned() {
        for t in super::registry() {
            let m = content::page(t.id()).unwrap_or_else(|| panic!("{} not in PAGES", t.id()));
            assert_ne!(
                m.status,
                Status::Planned,
                "{} has code but is marked planned",
                m.id
            );
        }
    }

    #[test]
    fn every_page_has_its_texts_in_every_language() {
        for lang in Lang::ALL {
            let tr = I18n::new(lang);
            for p in PAGES {
                assert!(
                    tr.has(&p.title_key(), None),
                    "{}: {}",
                    lang.code(),
                    p.title_key()
                );
                for l in p.links {
                    assert!(tr.has(l.hook, None), "{}: {}", lang.code(), l.hook);
                }
                if let Some(k) = p.status.key() {
                    assert!(tr.has(k, None), "{}: {k}", lang.code());
                }
            }
            for s in Section::ALL {
                assert!(tr.has(s.key(), None), "{}: {}", lang.code(), s.key());
                assert!(
                    tr.has(s.key(), Some("intro")),
                    "{}: {}.intro",
                    lang.code(),
                    s.key()
                );
            }
        }
    }

    /// Glyphs and letters must exist in the fonts, or they render as boxes (Q-013).
    /// Reads the character maps of the proportional font family directly.
    #[test]
    fn glyphs_and_slovak_letters_exist_in_the_fonts() {
        use skrifa::MetadataProvider as _;
        let defs = crate::theme::fonts();
        let family = &defs.families[&egui::FontFamily::Proportional];
        let faces: Vec<skrifa::FontRef> = family
            .iter()
            .map(|name| {
                let data = &defs.font_data[name];
                skrifa::FontRef::from_index(&data.font, data.index).expect("font parses")
            })
            .collect();
        let has = |c: char| faces.iter().any(|f| f.charmap().map(c).is_some());

        let mut needed: Vec<String> = PAGES.iter().map(|p| p.glyph.to_owned()).collect();
        needed.extend(Section::ALL.iter().map(|s| s.glyph().to_owned()));
        // chrome glyphs used in shell.rs and kit.rs
        needed.extend(
            [
                "🖱", "☰", "⚙", "🗑", "🔍", "🐄", "→", "🌙", "☀", "💻", "💾", "·",
                // rule of three
                "💯", "🍳", "⛽", "🏷", "👁", "🎓", "÷", "€", "−", "⚠", "⚖", "👥", "▦", "📏", "📈",
                "💡", "🔄", "📉", "💶", "📰", "●", "⏳", "✏",
            ]
            .map(String::from),
        );
        // Slovak letters, both cases, and the symbols the sims print
        needed.extend(
            "áäčďéíĺľňóôŕšťúýžÁÄČĎÉÍĹĽŇÓÔŔŠŤÚÝŽ⁻⁰¹²³⁴⁵⁶⁷⁸⁹×·√"
                .chars()
                .map(String::from),
        );
        let missing: Vec<String> = needed
            .into_iter()
            .filter(|s| !s.chars().filter(|c| *c != '\u{fe0f}').all(has))
            .collect();
        assert!(missing.is_empty(), "missing in fonts: {missing:?}");
    }
}
