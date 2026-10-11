# Decisions

Format: **D-nnn — title** (date, status). Why, and what it rules out.
Status: *accepted*, *tentative* (revisit before launch), *superseded by D-x*.

**D-001 — Rust + egui/eframe, compiled to wasm** (2026-10-11, accepted)
One language for sims and UI; same code runs natively for fast debugging.
Cost: canvas UI is not indexable by search engines and not a normal web page
(no text selection, ctrl-F, browser translate). Mitigated a little by the
`<meta description>` and `<noscript>` text.

**D-002 — Workspace split: `sims` (pure) / `app` (egui) / `server` (axum)** (2026-10-11, accepted)
Physics and maths live in `sims` with tests and no UI dependency, so they can
be checked natively and reused (e.g. by a future wgpu renderer or a CLI).

**D-003 — eframe with the wgpu renderer** (2026-10-11, accepted)
Needed for the planned 3D powers-of-ten cubes (custom wgpu paint callback,
same pattern as spatlers). egui-wgpu prefers WebGPU and falls back to WebGL2.
v0 topics draw with the egui painter only.

**D-004 — Topics as a trait + static metadata + registry** (2026-10-11, accepted)
`Topic { meta(), ui(ui, nav), save(), load() }`; `TopicMeta` is a static with
id, title, icon, group, blurb, maturity, related links. The id is permanent:
it is the URL route and the storage key. Tests enforce unique, URL-safe ids
and that every related link targets a registered topic.

**D-005 — Hash routing (`#/topic-id`)** (2026-10-11, accepted)
Shareable links and working back button without server routing; any static
host works. The app polls the hash each frame to follow back/forward.

**D-006 — Local saving only with consent; the answer itself is stored** (2026-10-11, accepted)
Nothing is written to localStorage until the visitor answers the banner.
"No" is stored too (as strictly necessary), so we don't ask every visit.
Withdrawing consent or "Forget everything" clears the origin's localStorage.
egui's own memory (window/scroll state) follows the same consent.
No CMP (CookieYes etc.) needed while there are no ads/analytics — see Q-004.

**D-007 — Ad slot lives in the HTML, not the canvas** (2026-10-11, tentative)
Ad networks insert DOM elements/iframes; they cannot draw into our canvas.
`index.html` has a hidden `<aside id="ad-top">` above the canvas. Turning ads
on will also require a consent platform (TCF) and an updated privacy page.

**D-008 — Server is a placeholder** (2026-10-11, accepted)
axum serving `dist/` with compression and basic headers. Accounts, sessions
(AAA), server-side saves are later; when they come, state already serialises
via serde per topic, so a server store can sit behind the same `save/load`.

**D-009 — No OS randomness; seeded `SmallRng`** (2026-10-11, accepted)
Avoids getrandom's wasm configuration and makes runs reproducible
("show me that again"). Seeds change on "Start over".

**D-010 — Code AGPL-3.0-or-later, content CC BY-SA 4.0** (2026-10-11, tentative)
Mirrors the Flysch choice. Open: Q-003.

**D-011 — Topic graph with question-phrased links** (2026-10-11, accepted)
`TopicMeta.related` renders a "Where next?" strip; `Nav::link` lets a topic
link inline from its text. Curriculum lives in CURRICULUM.md.

**D-012 — Playful look: indigo night / warm paper, colour per subject, Nunito** (2026-10-11, accepted)
"Not a Wikipedia page, not a textbook, not childish." Dark default is deep
indigo (`#161a30`), light is warm paper (`#fbf6ec`). Each subject area has one
bright accent (scale = sunflower, chance = violet, physics = teal, maths =
coral) used for its pills, menu entry, icon badge and charts. Solid fills with
dark text for action buttons, because tinted warm colours over navy turn
muddy brown. Rounded everything (10–16 pt radii). Typeface: Nunito variable
(OFL, no reserved name), subsetted to Latin + Latin Ext + symbols (175 KB),
weights 520 body / 760 bold / 880 headings, with a superscript-minus glyph
added (tools/build-font.sh). egui's emoji fonts and Hack are fallbacks.

**D-013 — Localisation with Fluent; catalogs embedded now, server-swappable later** (2026-10-11, accepted)
Answers Q-002. All visible text lives in `crates/app/locales/{en,sk}.ftl`
(Project Fluent). Fluent picks Slovak plural forms by CLDR rules inside the
translation (1 kocka / 2 kocky / 3,4 kocky / 5 kociek), so code never
handles grammar. Language = visitor's choice (EN|SK switch in the top bar,
"automatic" in settings) else the browser's language else English. Decimal
comma in Slovak via `I18n::num`. Catalogs are compiled in for now (works
offline, never shows raw keys); the server already serves `/i18n/<lang>.ftl`
and `I18n::replace_catalog` is the seam for fetching newer texts. English is
always the fallback for missing keys. Tests: both catalogs parse and have the
same keys; every literal key used in code exists; every topic, example, unit,
exponent and SI prefix has its text in every language.
Slovak style: tykanie; number names in the long scale (miliarda, bilión).

**D-014 — Powers of ten use a generic 3×3 puzzle cube in a 10 cm box** (2026-10-11, accepted)
Every cube (57 mm, 80 g) sits centred in its own invisible 10 cm box, so 10 in
a row = exactly 1 m with 4.3 cm of air between neighbours, 1000 = a 1 m cube,
and every third step repeats at 10× the size. Marked with a 🐄 model alert.
Drawn: e = 0…3 as real cubes with stickers (10³ draws all 1000), e ≥ 4 as
blocks whose faces show a 10×10 grid of the blocks one level down; a new
colour per 10³ level, with a colour key. A 1.4 m kid stands in for scale at
10²…10⁶; dimension lines and rulers with ticks per box/block.
Trademark: the drawing is a generic scrambled cube with no logo or name; the
text says "puzzle cube" / "kocka-hlavolam" (Q-011). Not legal advice: the EU
3D shape trade mark for the cube was struck down (CJEU, Simba Toys v EUIPO,
2016), but the "Rubik's" name and logo are still registered marks.

**D-015 — Deep links into topic state** (2026-10-11, accepted)
`#/powers-of-ten/9` opens the topic at 10⁹ (`Topic::set_param`). Shareable
("look at a billion!") and used for screenshots during development.
