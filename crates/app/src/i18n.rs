//! Localisation with Project Fluent (`.ftl` catalogs).
//!
//! Catalogs live in `crates/app/locales/<lang>.ftl`. Today they are compiled in,
//! so the page works offline and never shows raw keys. The seam for later is
//! [`I18n::replace_catalog`]: fetch a newer `.ftl` from the server (it already
//! serves `/i18n/<lang>.ftl`) and swap it in at runtime. English is always kept
//! as the fallback for keys a catalog lacks.
//!
//! Why Fluent: Slovak has four plural forms (1 kocka, 2 kocky, 1,5 kocky,
//! 5 kociek); Fluent selects them by CLDR rules inside the translation, so code
//! never has to know.

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Lang {
    #[default]
    En,
    Sk,
}

impl Lang {
    pub const ALL: [Lang; 2] = [Lang::En, Lang::Sk];

    pub fn code(self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Sk => "sk",
        }
    }

    pub fn short(self) -> &'static str {
        match self {
            Lang::En => "EN",
            Lang::Sk => "SK",
        }
    }

    /// The language's name in itself (not translated on purpose).
    pub fn native_name(self) -> &'static str {
        match self {
            Lang::En => "English",
            Lang::Sk => "Slovenčina",
        }
    }

    pub fn decimal_sep(self) -> char {
        match self {
            Lang::En => '.',
            Lang::Sk => ',',
        }
    }

    /// Best match for a BCP-47 tag ("sk-SK", "en_GB.UTF-8", …). None = no match.
    pub fn from_tag(tag: &str) -> Option<Lang> {
        let primary = tag.split(['-', '_', '.']).next()?.to_ascii_lowercase();
        match primary.as_str() {
            "sk" => Some(Lang::Sk),
            "en" => Some(Lang::En),
            _ => None,
        }
    }

    pub fn builtin_catalog(self) -> &'static str {
        match self {
            Lang::En => include_str!("../locales/en.ftl"),
            Lang::Sk => include_str!("../locales/sk.ftl"),
        }
    }
}

type Bundle = FluentBundle<FluentResource>;

fn make_bundle(lang: Lang, source: String) -> Result<Bundle, Vec<String>> {
    let resource = FluentResource::try_new(source)
        .map_err(|(_, errs)| errs.iter().map(|e| format!("{e:?}")).collect::<Vec<_>>())?;
    let id: LanguageIdentifier = lang.code().parse().expect("language codes are valid");
    let mut bundle = FluentBundle::new(vec![id]);
    // Unicode isolation marks around arguments are for mixed-direction text;
    // we have none, and the fonts don't draw them.
    bundle.set_use_isolating(false);
    bundle
        .add_resource(resource)
        .map_err(|errs| errs.iter().map(|e| format!("{e:?}")).collect::<Vec<_>>())?;
    Ok(bundle)
}

/// Build `FluentArgs` inline: `args!["n" => 3.0, "name" => "Ana"]`.
#[macro_export]
macro_rules! args {
    ($($k:literal => $v:expr),* $(,)?) => {{
        #[allow(unused_mut)]
        let mut a = ::fluent_bundle::FluentArgs::new();
        $( a.set($k, $v); )*
        a
    }};
}

pub struct I18n {
    lang: Lang,
    bundle: Bundle,
    /// English, when `lang` is not English.
    fallback: Option<Bundle>,
}

impl I18n {
    pub fn new(lang: Lang) -> Self {
        let builtin = |l: Lang| {
            make_bundle(l, l.builtin_catalog().to_owned())
                .unwrap_or_else(|e| panic!("built-in {} catalog is broken: {e:?}", l.code()))
        };
        Self {
            lang,
            bundle: builtin(lang),
            fallback: (lang != Lang::En).then(|| builtin(Lang::En)),
        }
    }

    pub fn lang(&self) -> Lang {
        self.lang
    }

    /// Swap in a catalog for the current language (e.g. fetched from the server).
    /// On error the current catalog stays.
    #[allow(dead_code)] // used once catalogs are served (DECISIONS D-013)
    pub fn replace_catalog(&mut self, source: String) -> Result<(), Vec<String>> {
        self.bundle = make_bundle(self.lang, source)?;
        Ok(())
    }

    fn format(&self, key: &str, attr: Option<&str>, args: Option<&FluentArgs>) -> Option<String> {
        for bundle in std::iter::once(&self.bundle).chain(self.fallback.as_ref()) {
            let Some(msg) = bundle.get_message(key) else {
                continue;
            };
            let pattern = match attr {
                Some(a) => msg.get_attribute(a).map(|at| at.value()),
                None => msg.value(),
            };
            if let Some(pattern) = pattern {
                let mut errors = Vec::new();
                let s = bundle.format_pattern(pattern, args, &mut errors);
                if !errors.is_empty() {
                    log::warn!("i18n {key}: {errors:?}");
                }
                return Some(s.into_owned());
            }
        }
        None
    }

    /// Plain message.
    pub fn t(&self, key: &str) -> String {
        self.format(key, None, None)
            .unwrap_or_else(|| format!("[{key}]"))
    }

    /// Message attribute, e.g. `topic-dice-bell.blurb`.
    pub fn attr(&self, key: &str, attr: &str) -> String {
        self.format(key, Some(attr), None)
            .unwrap_or_else(|| format!("[{key}.{attr}]"))
    }

    /// Message with arguments.
    pub fn ta(&self, key: &str, args: &FluentArgs) -> String {
        self.format(key, None, Some(args))
            .unwrap_or_else(|| format!("[{key}]"))
    }

    #[cfg(test)]
    pub fn has(&self, key: &str, attr: Option<&str>) -> bool {
        self.bundle.get_message(key).is_some_and(|m| match attr {
            Some(a) => m.get_attribute(a).is_some(),
            None => m.value().is_some(),
        })
    }

    /// Localise the decimal point in a number string from `sims::fmt`
    /// ("1.65 × 10¹³ J" → "1,65 × 10¹³ J" in Slovak). Only dots between digits change.
    pub fn num(&self, s: &str) -> String {
        localize_decimal(s, self.lang.decimal_sep())
    }

    /// Topic title and blurb, from `topic-<id>` and its `.blurb`.
    pub fn topic_title(&self, id: &str) -> String {
        self.t(&format!("topic-{id}"))
    }

    pub fn topic_blurb(&self, id: &str) -> String {
        self.attr(&format!("topic-{id}"), "blurb")
    }
}

pub fn localize_decimal(s: &str, sep: char) -> String {
    if sep == '.' {
        return s.to_owned();
    }
    let chars: Vec<char> = s.chars().collect();
    chars
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            let digit_around = i > 0
                && chars[i - 1].is_ascii_digit()
                && chars.get(i + 1).is_some_and(|n| n.is_ascii_digit());
            if c == '.' && digit_around { sep } else { c }
        })
        .collect()
}

/// Lower-case the first letter for use mid-sentence, unless the word is an
/// acronym ("ISS", "LHC"): "A flying mosquito" → "a flying mosquito".
pub fn lower_first(s: &str) -> String {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(a), Some(b)) if b.is_lowercase() || b == ' ' => {
            a.to_lowercase().chain(s.chars().skip(1)).collect()
        }
        _ => s.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fluent_bundle::FluentResource;
    use fluent_syntax::ast::Entry;
    use std::collections::BTreeSet;

    /// Every message and attribute defined in a catalog, as "key" / "key.attr".
    fn keys(lang: Lang) -> BTreeSet<String> {
        let res = FluentResource::try_new(lang.builtin_catalog().to_owned())
            .unwrap_or_else(|(_, e)| panic!("{} catalog does not parse: {e:?}", lang.code()));
        let mut out = BTreeSet::new();
        for entry in res.entries() {
            if let Entry::Message(m) = entry {
                if m.value.is_some() {
                    out.insert(m.id.name.to_string());
                }
                for a in &m.attributes {
                    out.insert(format!("{}.{}", m.id.name, a.id.name));
                }
            }
        }
        out
    }

    #[test]
    fn catalogs_parse_and_have_the_same_keys() {
        let en = keys(Lang::En);
        let sk = keys(Lang::Sk);
        let only_en: Vec<_> = en.difference(&sk).collect();
        let only_sk: Vec<_> = sk.difference(&en).collect();
        assert!(only_en.is_empty(), "missing in sk.ftl: {only_en:?}");
        assert!(only_sk.is_empty(), "missing in en.ftl: {only_sk:?}");
    }

    /// Scan the source for literal keys passed to `.t(`, `.ta(` and `.attr(`.
    #[test]
    fn every_literal_key_in_code_exists() {
        let en = keys(Lang::En);
        let src_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
        let mut missing = Vec::new();
        let mut stack = vec![std::path::PathBuf::from(src_dir)];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.ends_with("i18n.rs") {
                    continue; // this file's own examples
                }
                let text = std::fs::read_to_string(&path).unwrap();
                for call in [".t(\"", ".ta(\"", ".attr(\""] {
                    for (i, _) in text.match_indices(call) {
                        let rest = &text[i + call.len()..];
                        let key = &rest[..rest.find('"').unwrap()];
                        let full = if call == ".attr(\"" {
                            // .attr("key", "attr") — skip calls whose attr isn't a literal
                            let Some(a) = rest[key.len() + 1..].strip_prefix(", \"") else {
                                continue;
                            };
                            format!("{key}.{}", &a[..a.find('"').unwrap()])
                        } else {
                            key.to_string()
                        };
                        if !en.contains(&full) {
                            missing.push(format!("{}: {full}", path.display()));
                        }
                    }
                }
            }
        }
        assert!(
            missing.is_empty(),
            "keys used in code but not in en.ftl:\n{}",
            missing.join("\n")
        );
    }

    #[test]
    fn slovak_plurals() {
        let i = I18n::new(Lang::Sk);
        let cubes = |n: f64| {
            i.ta(
                "amount-sugar-cube",
                &args!["count" => n, "shown" => i.num(&n.to_string())],
            )
        };
        assert_eq!(cubes(1.0), "1 kocka cukru");
        assert_eq!(cubes(3.0), "3 kocky cukru");
        assert_eq!(cubes(3.4), "3,4 kocky cukru");
        assert_eq!(cubes(7.0), "7 kociek cukru");
    }

    #[test]
    fn helpers() {
        assert_eq!(localize_decimal("1.65 × 10¹³ J", ','), "1,65 × 10¹³ J");
        assert_eq!(localize_decimal("v0.1.0 a. b", ','), "v0,1,0 a. b");
        assert_eq!(lower_first("A flying mosquito"), "a flying mosquito");
        assert_eq!(lower_first("ISS in orbit"), "ISS in orbit");
        assert_eq!(Lang::from_tag("sk-SK"), Some(Lang::Sk));
        assert_eq!(Lang::from_tag("en_GB.UTF-8"), Some(Lang::En));
        assert_eq!(Lang::from_tag("de-AT"), None);
    }
}
