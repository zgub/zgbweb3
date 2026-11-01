# Decisions

Format: **D-nnn — title** (date, status). Why, and what it rules out.
Status: *accepted*, *tentative* (revisit before launch), *superseded by D-x*.

**D-001 — Rust + egui/eframe, compiled to wasm** (2026-10-11, accepted)
One language for sims and UI; same code runs natively for fast debugging.
Cost: canvas UI is not indexable by search engines and not a normal web page
(no text selection, ctrl-F, browser translate). Mitigated a little by the
`<meta description>` and `<noscript>` text. Since D-021 the poor indexability
is wanted rather than a cost.

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
and that every related link targets a registered topic. Since D-029/D-030 this
covers pages in all five sections, not only Explore topics. Since D-032 the
metadata (`PageMeta`: id, section, glyph, status, links) lives in the `content`
crate's site map; the app's `Topic` trait only adds behaviour.

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
Since D-024, targeted ads are ruled out; only non-tracking ads could ever use
this slot.

**D-008 — Server is a placeholder** (2026-10-11, accepted)
axum serving `dist/` with compression and basic headers. Accounts, sessions
(AAA), server-side saves are later; when they come, state already serialises
via serde per topic, so a server store can sit behind the same `save/load`.
Refined by D-032.

**D-009 — No OS randomness; seeded `SmallRng`** (2026-10-11, accepted)
Avoids getrandom's wasm configuration and makes runs reproducible
("show me that again"). Seeds change on "Start over".

**D-010 — Code AGPL-3.0-or-later, content CC BY 4.0** (2026-10-11, tentative; content part revised 2026-10-21)
Content: Peter's rule is "reuse with a link or credit" (D-025), which is what
CC BY 4.0 says, so CC BY 4.0 replaces CC BY-SA. Code licence still open
(Q-003).

**D-011 — Topic graph with question-phrased links** (2026-10-11, accepted)
`TopicMeta.related` renders a "Where next?" strip; `Nav::link` lets a topic
link inline from its text. Curriculum lives in CURRICULUM.md. Extended across
sections by D-029.

**D-012 — Playful look: indigo night / warm paper, colour per subject, Nunito** (2026-10-11, superseded by D-018)
"Not a Wikipedia page, not a textbook, not childish." Dark default is deep
indigo (`#161a30`), light is warm paper (`#fbf6ec`). Each subject area has one
bright accent (scale = sunflower, chance = violet, physics = teal, maths =
coral) used for its pills, menu entry, icon badge and charts. Solid fills with
dark text for action buttons. Rounded everything (10–16 pt radii). Typeface:
Nunito variable (OFL), subsetted to Latin + Latin Ext + symbols (175 KB), with
a superscript-minus glyph added (tools/build-font.sh). egui's emoji fonts and
Hack are fallbacks. Kept here as a record; any code or assets already built to
this style are affected by D-018.

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
Reaffirmed 2026-10-21: the whole site is available in English and Slovak.
The "server-swappable later" half is superseded by D-032: catalogs are only
compiled in, the server no longer serves `/i18n/*.ftl`, `replace_catalog` is gone.
Catalogs moved to `crates/content/locales/`.

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

**D-016 — Project name zgbweb3; fresh repo** (2026-10-11, accepted)
Third generation. zgbweb (1) was for learning Rust/wasm/axum and the infra.
zgbweb (2) was hand-written with LLM help and stalled about a year ago on a
file-import abstraction. zgbweb3 starts clean from this skeleton in a new
GitHub repo; nothing is ported from 1 or 2 (gas-box code: Q-007; zgbweb2
code: Q-019, deferred).
Crates: `zgbweb3` (app, lib + bin), `zgbweb3-server`, `sims`.

**D-017 — The site is Peter's portfolio; education is one part of it** (2026-10-21, accepted)
Sections: Projects, Playground, Workbench, Explore, and since D-030 History.
The kids' explainer no longer defines the site. Everything is public,
including the Workbench. The right-hand menu is grouped by section instead of
by strand. A mock of the shell with alternative menu structures was reviewed
2026-10-19; the by-kind grouping was the starting point.

**D-018 — One look for the whole site: the old zgbweb (zgbweb2) style** (2026-10-21, accepted)
Supersedes D-012. The site uses egui styling as in zgbweb2: top bar with theme
buttons (plus the EN | SK switch), fixed 200 px right menu with a glyph
heading, section headings with glyphs, separators and full-width selectable
rows with glyphs, a centre panel with heading, separator and content, and a
one-line footer. Simulation controls sit in a row above the canvas, as on the
old Game of Life page. The same look for every section and every audience.
Visually appealing but not overly colourful: content first, and colour,
effects and fanciness only where they underline the topic. Fonts later (Q-013).
Colours: Ember since D-034 (the chrome is no longer stock grey).

**D-019 — Domain zgub.net; identities kept loosely apart** (2026-10-21, accepted)
Peter owns zgub.net and the site lives there. It may move to hunci.sk later;
not decided. "zgub" is his internet identity, known to many people who have
never met him; "hunci" is the grammar-school nickname everyone who knows him in
person uses. He wants to keep them apart, but not strictly. Consequences: the
site speaks as zgub; no hard-coded domain anywhere; a move would change the
browser origin, so local saves and consent (D-006) would reset and old links
would need a whole-site redirect. The operator identity on the privacy page
(Q-005) is where the two identities may meet.

**D-020 — Audience: any curious mind, of any age** (2026-10-21, accepted)
Replaces "curious kids 9–14". Anyone curious, whether 6, 16, 26, 40 or 60.
Pages start plain and simple with depth and assumptions one click away.
No childish framing anywhere.

**D-021 — Shared by word and links, not by crawlers** (2026-10-21, tentative)
Peter does not want search engines or AI crawlers harvesting the site.
Measures (details in BRIEF.md): canvas-only UI; robots.txt disallowing
everything with named AI crawlers; robots meta tag and `X-Robots-Tag: noindex,
nofollow, noarchive`; a TDM opt-out header; axum middleware refusing known
crawler user agents and rate-limiting; static title/description kept for link
previews. Limits: only well-behaved bots obey; text in `/i18n/*.ftl` and in
the wasm is readable; nothing stops a determined scraper. Tentative until
hosting (Q-006) is settled.

**D-022 — Landing page chosen in configuration** (2026-10-21, accepted)
The About page is short and not the landing page. Peter picks the landing
page in configuration: the Explore index or the Playground index. Proposed
mechanism (not built): a config value `landing = "explore" | "playground"`
compiled in by default, overridable by a small config file from the axum
server, the same seam as the i18n catalogs. An empty hash (`#/`) opens it.
Built 2026-10-21 as the compiled-in constant `content::LANDING` (Explore);
the server override is not built (D-032 keeps the server thin).

**D-023 — No names of private people on the site, ever** (2026-10-21, accepted)
Peter's own decision, not a GDPR precaution. Family, friends, pupils,
colleagues and the school are never named, in any section, in either
language, including captions and photo file names. Named on purpose:
historical figures (History, D-030) and authors credited for their work
(D-025). Scope of "private people" to confirm: Q-023.

**D-024 — No tracking, no targeted ads, stated as a conviction** (2026-10-21, accepted)
Peter does not like tracking and targeted advertising. The privacy page says
so in his voice, and says it is his choice rather than a legal minimum.
No analytics, no tracking pixels, no third-party scripts that track, no
targeted ads. Non-tracking ads are not ruled out here (Q-004).

**D-025 — Copyright and credit** (2026-10-21, accepted)
Everything on the site is under copyright, as the law gives it. Peter's own
work may be reused with a link or credit (hence CC BY 4.0, D-010). Other
people's work (images, quotes, data) is used only with proper credit and
within its licence; each use records author, source and licence next to it.
Correct attribution is the default for every page, not an extra step.

**D-026 — Energy ladder: 1 eV to 1 foe, one sourced reference list** (2026-10-21, accepted)
The main ladder runs from 1 eV to 1 foe (10⁴⁴ J, a supernova blast ≈ the
Sun's lifetime output), about 63 orders of magnitude. An "edge of the map"
annex goes on to the rest energy of the observable universe (~10⁷⁰ J) and
explains why there is no number for the Big Bang. All rungs, values,
assumptions and sources live in ENERGY-LADDER.md; every comparison on the
site uses only those rungs. Any rung can be the unit of measurement. The
ladder is also browsable on its own. Peter adds rungs over time; each is
recorded in the doc first, then in the app. Answers Q-016 except the
household figure.

**D-027 — Section names** (2026-10-21, accepted; Explore tentative)
Playground / Ihrisko (was "Fun & Games"). Workbench / Dielňa (Slovak name
proposed). Explore / Objavuj for the section of things Peter explains
(working name; meant to draw a curious mind in). History / História.
Projects / Projekty.

**D-028 — Where first topics live** (2026-10-21, accepted)
Explore: exponential growth and orders of magnitude, numbers and scales,
number bases, logic and gates, electricity, energy scales, basic
thermodynamics. Workbench: statistics and probability (Peter wants to revisit
them himself), music theory, linear algebra, tensor calculus.

**D-029 — Extensive cross-linking: put back together what school splits** (2026-10-21, accepted)
The core idea of the site. Every page links across sections: games ↔ topics
↔ History ↔ projects ↔ Workbench. Example: the jolt when you take off a
sweater links to a game, to Electricity (voltage, current, resistance,
capacitance), to a rung on the energy ladder, to History and to projects with
electronics in them. Cross-links are written with the page, in both
languages, phrased as the next question (D-011). Proposed check: a test that
warns when a page links only inside its own section.

**D-030 — History section** (2026-10-21, accepted)
Not extensive. Short, interesting pieces about past minds and their ideas,
for example Faraday, Galileo, Descartes, with some philosophy. Each piece
links to the topics, games and projects where its ideas live, and those link
back ("who figured this out?"). Shape and depth: Q-024.

**D-031 — Purpose: learning together, in public** (2026-10-21, accepted)
The ultimate goal of the site: Peter and his kids gaining knowledge the fun
way, learning on the go together, and publishing it so others benefit.
The kids shape what gets explored by asking, suggesting and steering.
Knowledge is the defence against ignorance and against manipulation by
those in power. When a choice is unclear, the option that serves this goal
wins. A possible video channel, far in the future, would show the kids'
hands only, never faces or names (D-023); open points in Q-025.

**D-032 — Static-first: everything compiled in, thin axum, Postgres later** (2026-10-21, accepted)
Site map, texts in both languages and (later) data such as the energy ladder
are compiled into the wasm. A backend does not protect public content; it
protects data, and there is none yet. Reasons: content is reviewed in git and
checked by tests; no plain-text files on the server for crawlers (D-021); the
same bundle runs on any host and offline. Supersedes the served-catalogs part of
D-013. New crate `crates/content` holds the site map (`Section`, `PageMeta`,
`PAGES`, `LANDING`) and the catalogs. axum stays, thin: serves `dist/`; sends
`X-Robots-Tag: noindex, nofollow, noarchive, noimageindex` and
`tdm-reservation: 1`; serves a disallow-all `robots.txt`; refuses named AI
crawlers by user agent with 403; `/api` is reserved and answers 404 JSON.
Not built yet: per-IP rate limiting (with hosting, Q-006). Postgres arrives with
the first feature that stores data (ideas inbox, comments, saved progress,
accounts), together with its moderation, retention and privacy decisions
(Q-020). Pages already save through `save()`/`load()` (D-004, D-008), so that
store can move to the server without touching the pages.

**D-033 — Empty shell: menu and placeholders, content added page by page** (2026-10-25, accepted)
Peter keeps the concept (five sections, cross-linking, both languages) but
starts the content from zero and provides it one page at a time. All topic
code is removed from the app (powers of ten, energy ladder, dice bell; it stays
in git history, commit 17b57db and earlier). Every page is `Planned` and shows
the placeholder "Nothing but sawdust" / "Nič, len piliny" with a line about
being at the right place at the wrong time (x, y, z right, t wrong). The menu
is Peter's list of 2026-10-25: Projects (Flysch shell journal, Spatlers,
Electronics), Playground (Conway's Game of Life, Gas in a box), Workbench
(Statistics, Probability, Music, Linear algebra, Tensor calculus), Explore
(Numbers, Thermodynamics, What is energy, Chemistry, Biology), History (empty).
The `sims` crate, `kit` widgets and the `Topic` trait stay as the toolkit for
new pages; dead-code warnings for them are allowed until pages use them.
Supersedes the page list in D-028 and in BRIEF.md "The five sections".

**D-034 — Colour palette: Ember, dark first** (2026-10-28, accepted)
Refines D-018: same layout, but the chrome is no longer stock grey. Peter and
his son compared four warm dark palettes rendered in real egui (Ember,
Lamplight, Dusk, Slate & Copper, plus stock egui) and chose **Ember**: warm
charcoal panels `#1d1a17`, canvases `#141210`, cards `#27231f`, lines
`#3b352f`; text `#cfc6b8`, strong `#f3ece1`, weak `#8f8577`; one accent, amber
`#f0a04b` (selection, links, the known value in pictures); note colour
`#e8b04f` (🐄 callouts). Chart series in order: amber `#f0a04b`, teal
`#4fb3a2`, coral `#e06c5a`, sky `#6aa6d6`, plum `#b58ad0`, olive gold
`#c9b458`. Sequential ramp (speed, heat, height): `#2a1b38` → `#7a2a4f` →
`#c8463d` → `#ef8a32` → `#f8d66d`. Dark mode is preferred because Peter finds
it easier on the eyes; light mode stays stock egui for now (Q-027). Colour
still lives mostly inside the pictures; text stays calm. Code:
`theme::DARK`, `theme::ember_visuals()`.

**D-035 — "What changes what": relationships taught from real examples** (2026-10-28, tentative)
Many people manage big and small numbers but struggle with the basics: rule
of three, percentages, and the shapes of linear, quadratic, inverse,
exponential and logarithmic relationships. A group of Explore pages covers
them, the other way round from school: a real situation first, the visitor
guesses and plays, the picture or curve comes from their tries, and only
then a box "What school calls this / Ako to volajú v škole" gives the names
(function, direct proportion, domain = definičný obor, range = obor hodnôt).
Menu titles say what happens, not the school term, so nobody is put off.
Page ids stay neutral so titles can change (Q-026): `rule-of-three`,
`percent`, `linear`, `quadratic`, `inverse`, `exponential`, `logarithm`.
They sit at the top of Explore. Games for the same curves go to Playground.
Order of work (Peter): this group, then trigonometry, then big and small
numbers. Rule of three first (v0.4.0). Plan: WHAT-CHANGES-WHAT.md.

**D-036 — Branches, versions, tags, and a quiet test period** (2026-10-28, accepted)
Branches: `main` is what is released; `develop` is the development branch;
all work happens on feature branches off `develop` (e.g. `rule-of-three`),
merged back into `develop`, and `develop` is merged into `main` after a
version has been tested. Nothing is committed straight to `main` or `develop`.
Every release bumps the workspace version (shown in the footer) and gets a git
tag `vX.Y.Z`. 0.4.0 = Ember + rule of three. After a release, Peter tests it
(with the kids, on real devices) while the code stays unchanged; findings go to
BUGS.md and QUESTIONS.md and are fixed in the next version, not in place.
Docs may change during the test period. Peter pushes to GitHub
(github.com/zgub/zgbweb3). Commits use his noreply address.

**D-037 — Headless screenshots: `examples/shot.rs`** (2026-10-28, accepted)
Screenshots no longer need a window or GPU: `cargo run --example shot --
<route>` runs the real app for a few frames, tessellates egui's output and
fills the triangles on the CPU into a PNG (`ZGB_SHOT`, `ZGB_SIZE`, `ZGB_LANG`).
Close to, not identical with, the wgpu output. Used for every page review;
the consent banner is answered "no" so it stays out of the picture.

**D-038 — One design system: DESIGN.md and the `kit` widgets** (2026-10-28, accepted)
Peter wants the whole site to look consistent: colours, fonts and sizes,
element types and plot types picked once and reused. DESIGN.md is the
rulebook (colour roles, type scale, page anatomy, elements, plot conventions,
wording, screens); `kit` holds the shared widgets (section, choice bar, inset,
takeaway, fold-outs). A page needing something new adds it to DESIGN.md and
`kit` first. First rules from Peter's review of v0.4.0: smallest text 12.5 pt;
main switches (examples, pictures) as big choice bars, not radio buttons; the
parts of a page in separate framed sections; inputs in form style ("Počet
maliarov: [4]") so no plural word sits next to a number field.
