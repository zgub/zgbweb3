//! The site map of zgub.net and its texts, compiled into the app (DECISIONS D-032).
//!
//! - [`Section`]: the five sections of the menu (D-017, D-027, D-030).
//! - [`PAGES`]: every page that exists or is planned, with its permanent id,
//!   glyph, status and cross-links (D-004, D-011, D-029).
//! - [`locales`]: the English and Slovak Fluent catalogs (D-013).
//!
//! Adding a page: add a [`PageMeta`] to [`PAGES`], then `page-<id> = Title`
//! (and optionally `.blurb`) to **both** catalogs. Tests in this crate check the
//! map; tests in the app check that every text exists in every language.
//!
//! The id is permanent: it is the URL route (`#/<id>`), the storage key and the
//! catalog key. Never rename one that has been published without a redirect.

use serde::{Deserialize, Serialize};

pub mod locales;

/// A section of the site. Menu order is [`Section::ALL`] (Q-017 default).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Section {
    /// Hobby builds: electronics, radio, small computers, software.
    Projects,
    /// Games that play with an idea; each links to its Explore topic.
    Playground,
    /// Things Peter is still learning, written as he learns them.
    Workbench,
    /// Things Peter understands and explains.
    Explore,
    /// Short pieces about the people behind the ideas.
    History,
}

impl Section {
    pub const ALL: [Section; 5] = [
        Section::Projects,
        Section::Playground,
        Section::Workbench,
        Section::Explore,
        Section::History,
    ];

    /// URL route of the section's index page, e.g. `#/explore`.
    pub fn id(self) -> &'static str {
        match self {
            Section::Projects => "projects",
            Section::Playground => "playground",
            Section::Workbench => "workbench",
            Section::Explore => "explore",
            Section::History => "history",
        }
    }

    pub fn glyph(self) -> &'static str {
        match self {
            Section::Projects => "🛠",
            Section::Playground => "🎮",
            Section::Workbench => "🔧",
            Section::Explore => "🔭",
            Section::History => "📜",
        }
    }

    /// Catalog key of the section name; its `.intro` attribute is the one-line intro.
    pub fn key(self) -> &'static str {
        match self {
            Section::Projects => "section-projects",
            Section::Playground => "section-playground",
            Section::Workbench => "section-workbench",
            Section::Explore => "section-explore",
            Section::History => "section-history",
        }
    }

    pub fn from_id(id: &str) -> Option<Section> {
        Section::ALL.into_iter().find(|s| s.id() == id)
    }
}

/// The section shown at `#/` (D-022). Compiled in for now; a server-side
/// config file can override it later through the same seam.
pub const LANDING: Section = Section::Explore;

/// How finished a page is. Shown next to its menu row so visitors know what to expect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Only a title: the page is on the map, nothing is built yet.
    Planned,
    /// Works, still rough.
    Draft,
    /// Reviewed and polished.
    Ready,
}

impl Status {
    /// Catalog key of the marker shown in the menu; `Ready` shows none.
    pub fn key(self) -> Option<&'static str> {
        match self {
            Status::Planned => Some("status-planned"),
            Status::Draft => Some("status-draft"),
            Status::Ready => None,
        }
    }
}

/// A cross-link to another page, phrased as the next question a visitor would ask.
#[derive(Debug)]
pub struct Link {
    pub to: &'static str,
    /// Catalog key of the question, e.g. `hook-what-is-1e9`.
    pub hook: &'static str,
}

/// Language-independent facts about one page. Texts come from the catalogs.
#[derive(Debug)]
pub struct PageMeta {
    pub id: &'static str,
    pub section: Section,
    /// Must exist in the app's fonts (tested in the app).
    pub glyph: &'static str,
    pub status: Status,
    /// Shown as "Where next?" at the end of the page. Every target must exist (tested).
    pub links: &'static [Link],
}

impl PageMeta {
    /// Catalog key of the title; `.blurb` is an optional one-line description.
    pub fn title_key(&self) -> String {
        format!("page-{}", self.id)
    }
}

const fn planned(id: &'static str, section: Section, glyph: &'static str) -> PageMeta {
    PageMeta {
        id,
        section,
        glyph,
        status: Status::Planned,
        links: &[],
    }
}

/// Every page on the site, in menu order within each section.
pub static PAGES: &[PageMeta] = &[
    // --- Projects --------------------------------------------------------
    planned("lora-mesh", Section::Projects, "📡"),
    planned("home-lab", Section::Projects, "🖥"),
    planned("sensors", Section::Projects, "☀"),
    planned("flysch", Section::Projects, "✏"),
    planned("space-settlers", Section::Projects, "🚀"),
    // --- Playground ------------------------------------------------------
    PageMeta {
        id: "dice-bell",
        section: Section::Playground,
        glyph: "🎲",
        status: Status::Draft,
        links: &[
            Link {
                to: "statistics",
                hook: "hook-why-random-makes-a-bell",
            },
            Link {
                to: "gas-box",
                hook: "hook-molecules-make-a-bell",
            },
        ],
    },
    PageMeta {
        id: "gas-box",
        section: Section::Playground,
        glyph: "💨",
        status: Status::Planned,
        links: &[
            Link {
                to: "thermodynamics",
                hook: "hook-heat-is-jiggling",
            },
            Link {
                to: "dice-bell",
                hook: "hook-why-random-makes-a-bell",
            },
        ],
    },
    // --- Workbench -------------------------------------------------------
    planned("statistics", Section::Workbench, "📊"),
    planned("music-theory", Section::Workbench, "🎵"),
    planned("linear-algebra", Section::Workbench, "⊞"),
    planned("tensor-calculus", Section::Workbench, "∇"),
    // --- Explore ---------------------------------------------------------
    planned("exponential", Section::Explore, "📈"),
    PageMeta {
        id: "powers-of-ten",
        section: Section::Explore,
        glyph: "🔟",
        status: Status::Draft,
        links: &[
            Link {
                to: "energy-ladder",
                hook: "hook-what-is-1e13-joules",
            },
            Link {
                to: "exponential",
                hook: "hook-why-doubling-surprises",
            },
        ],
    },
    planned("number-bases", Section::Explore, "🔢"),
    planned("logic-gates", Section::Explore, "⚙"),
    planned("electricity", Section::Explore, "🔌"),
    PageMeta {
        id: "energy-ladder",
        section: Section::Explore,
        glyph: "⚡",
        status: Status::Draft,
        links: &[
            Link {
                to: "powers-of-ten",
                hook: "hook-what-is-1e9",
            },
            Link {
                to: "gas-box",
                hook: "hook-heat-is-jiggling",
            },
            Link {
                to: "electricity",
                hook: "hook-what-is-a-volt",
            },
        ],
    },
    planned("thermodynamics", Section::Explore, "🔥"),
    // --- History: nothing yet; its shape is Q-024. ----------------------
];

/// Routes taken by fixed pages and section indexes; page ids must not use them.
pub const RESERVED_ROUTES: [&str; 9] = [
    "",
    "about",
    "privacy",
    "license",
    "projects",
    "playground",
    "workbench",
    "explore",
    "history",
];

pub fn page(id: &str) -> Option<&'static PageMeta> {
    PAGES.iter().find(|p| p.id == id)
}

pub fn pages_in(section: Section) -> impl Iterator<Item = &'static PageMeta> {
    PAGES.iter().filter(move |p| p.section == section)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn ids_are_unique_url_safe_and_not_reserved() {
        let mut seen = BTreeSet::new();
        for p in PAGES {
            assert!(
                !p.id.is_empty()
                    && p.id
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "bad id {:?}",
                p.id
            );
            assert!(
                !RESERVED_ROUTES.contains(&p.id),
                "{} is a reserved route",
                p.id
            );
            assert!(seen.insert(p.id), "duplicate id {}", p.id);
        }
    }

    #[test]
    fn reserved_routes_cover_every_section() {
        for s in Section::ALL {
            assert!(RESERVED_ROUTES.contains(&s.id()), "{}", s.id());
            assert_eq!(Section::from_id(s.id()), Some(s));
        }
    }

    #[test]
    fn links_point_at_other_existing_pages() {
        for p in PAGES {
            for l in p.links {
                assert!(
                    page(l.to).is_some(),
                    "{} links to unknown page {}",
                    p.id,
                    l.to
                );
                assert_ne!(l.to, p.id, "{} links to itself", p.id);
            }
        }
    }

    /// D-029: a finished page must link outside its own section.
    /// Planned and draft pages are exempt while they are being built.
    #[test]
    fn ready_pages_link_across_sections() {
        for p in PAGES.iter().filter(|p| p.status == Status::Ready) {
            assert!(
                p.links
                    .iter()
                    .any(|l| page(l.to).is_some_and(|t| t.section != p.section)),
                "{} is ready but links only inside {:?}",
                p.id,
                p.section
            );
        }
    }

    #[test]
    fn glyphs_are_single_short_symbols() {
        for p in PAGES {
            assert!(
                !p.glyph.is_empty() && p.glyph.chars().count() <= 2,
                "{}: glyph {:?}",
                p.id,
                p.glyph
            );
        }
    }
}
