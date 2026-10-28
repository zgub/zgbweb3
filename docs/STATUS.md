# Status

## 2026-10-25 — empty shell: menu and placeholders (D-033)

**Done**
- Site map replaced with Peter's menu: 3 projects, 2 playground, 5 workbench,
  5 explore pages, History empty. All `Planned`. Ids: `flysch`, `spatlers`,
  `electronics`, `game-of-life`, `gas-box`, `statistics`, `probability`, `music`,
  `linear-algebra`, `tensor-calculus`, `numbers`, `thermodynamics`, `energy`,
  `chemistry`, `biology`.
- Removed topic code (`powers_of_ten`, `energy_ladder`, `clt_dice`) and all their
  texts and cross-link hooks from both catalogs; `registry()` is empty.
- Placeholder on every page and on the empty History index: "Nothing but sawdust"
  + "right place, wrong time: x, y, z right, t off" (EN and SK).
- Glyphs: ▦ Game of Life, ⚛ Chemistry, 🌱 Biology, 🎲 Probability, 🔌 Electronics
  (⚗ is not in the fonts).
- Removed the Slovak-plurals test (its sugar-cube key went with the energy ladder;
  bring a plural test back with the first page that has a plural).

**Verified**
- `cargo clippy --workspace --all-targets` clean, `cargo test --workspace` passes.
- Screenshots (native, software GL): Game of Life EN, History SK.
- Not verified: wasm build (B-001).

**Next**
1. Content, page by page, as Peter provides it.
2. `trunk serve` on a real machine (B-001).

## 2026-10-21 — v0.3.0: five-section shell in the old zgbweb look, static-first

**Done**
- New crate `crates/content` (D-032): the site map — five sections (Projects,
  Playground, Workbench, Explore, History; D-017, D-027, D-030), every page with a
  permanent id, glyph, status and cross-links — and the EN/SK catalogs, compiled in.
- Pages on the map: 5 projects, 2 playground, 4 workbench, 7 explore (all planned
  except the three that already had code). History is empty until Q-024.
- Kept the three working topics under their ids: `powers-of-ten` and
  `energy-ladder` (Explore), `dice-bell` (Playground). Dropped the old planned
  stubs `numbers`, `linear-maps`; `gas-box` moved to Playground as planned.
- Look (D-018): stock egui visuals and fonts; Nunito, the indigo/paper theme,
  per-subject menu colours, rounded cards and coloured buttons removed (D-012
  superseded). Top bar: site name, theme buttons 🌙 ☀ 💻, EN | SK, ⚙. Right menu:
  200 pt fixed, "🖱 Menu", section headings, glyph rows; planned pages dimmed,
  drafts marked. Centre: heading, separator, page, "Where next?" links. One-line
  footer. Default theme now follows the device.
- Routing: `#/` = landing section (`content::LANDING` = Explore, D-022),
  `#/<section>`, `#/<page>[/<param>]`, about/privacy/license.
- Server (D-032): `/i18n` removed; `robots.txt` disallow-all; `X-Robots-Tag`,
  `tdm-reservation: 1`; named AI crawlers refused with 403; `/api` reserved (404 JSON).
  `index.html`: robots meta, neutral description, egui background colour.
- Texts: privacy page states the no-tracking stance in Peter's voice (D-024);
  content licence text CC BY 4.0 (D-010, D-025).
- Docs: D-032, notes on D-004/D-008/D-013/D-022, BRIEF tech section, Q-013
  partly answered, Q-017 built with the default, CURRICULUM marked outdated,
  README and CLAUDE.md updated.

**Verified**
- `cargo clippy --workspace --all-targets` clean; `cargo test --workspace`:
  5 content + 18 sims + 13 app + 1 server tests pass. New tests: site map (ids,
  reserved routes, links, ready pages link across sections), texts for every page
  and section in both languages, routes round-trip, every glyph and Slovak letter
  exists in the fonts (reads the font files; checked that it catches a missing 🧊),
  crawler user agents.
- Server probed with curl: headers on every response, robots.txt, /api 404 JSON,
  GPTBot user agent gets 403.
- Screenshots (native, software GL): landing EN 1280×800, powers of ten SK at 10³,
  planned page EN at 400×860 with the menu folded.
- Not verified: wasm build (B-001).

**Next**
1. `trunk serve` on a real machine (B-001).
2. Answer Q-021/Q-014 (keep Objavuj / Dielňa?), Q-022, Q-023, Q-024.
3. First new Explore page; then the energy ladder topic rebuilt on ENERGY-LADDER.md data.
4. Rewrite CURRICULUM.md for the five sections.

## 2026-10-11 (evening) — renamed to zgbweb3, handed over for a new GitHub repo

Packages renamed (`zgbweb3`, `zgbweb3-server`), native storage id `zgbweb3`, D-016.
Repo copied to `C:\devel\rust\zgbweb3` with its git history; Peter pushes it to GitHub.

## 2026-10-11 (afternoon) — v0.2.0: playful look, Slovak + English, puzzle cubes

**Done**
- New look (D-012): indigo night / warm paper themes, one accent colour per subject,
  rounded cards, Nunito typeface (subset, with an added superscript minus), logo of
  three growing squares, solid colourful action buttons, 🐄 "model alert" callout.
- Localisation (D-013): Fluent catalogs `locales/en.ftl` + `sk.ftl` hold every visible
  text; EN|SK switch in the top bar, "automatic" (browser language) in settings;
  Slovak plurals and decimal comma; `<html lang>` kept in sync; server serves `/i18n/*.ftl`.
- Powers of ten rebuilt (D-014): puzzle cube 5.7 cm in a 10 cm box with dimension
  lines; 10 in a row = 1 m (cube + gap labelled); 10³ draws all 1000 cubes; bigger
  powers as blocks with a colour key; a 1.4 m kid for scale; weight of the pile;
  size hints up to "a cube reaching the edge of space" (10¹⁸ → 100 km).
- Deep links `#/topic/param` (D-015).
- Home cards adapt to width; phone width (400 pt) checked.

**Verified**
- `cargo clippy --workspace --all-targets` clean; tests: 18 sims + 12 app pass
  (incl. catalog parity, keys used in code, Slovak plurals, every exponent worded).
- Screenshots (native, software GL): home EN; powers of ten SK at 10⁰, 10¹, 10², 10³,
  10⁴, 10⁹, 10⁻⁴; energy ladder EN dark + SK light in sugar cubes; dice SK; settings
  menu; phone width. Fixed during review: muddy tints, card overflow, superscript minus,
  tiny-value plurals ("6,24 × 10⁻²⁶ kocky"), value column overflow, kid too dominant at 10¹.
- Still not verified: wasm build (B-001).

**Next**
1. `trunk serve` on a real machine (B-001) — now also checks font loading and the
   browser-language detection.
2. Answer Q-010…Q-013 (Czech → Slovak?, "Rubik's" in text, tykanie on privacy page, font).
3. Native-speaker pass over `sk.ftl` (written by Claude; plausible but unreviewed).
4. Spine topics: `numbers`, `doubling`.

## 2026-10-11 — v0.1.0 skeleton (restart)

Fresh start; the previous hand-written code is dropped (Q-007 for the gas sim).

**Done**
- Cargo workspace: `sims` (pure logic + tests), `app` (eframe/egui, native + wasm), `server` (axum static placeholder).
- App shell: top bar (home, breadcrumb, ⚙ settings, ☰ topic toggle), right topic menu grouped by strand with maturity badges, footer (privacy / license / about / version), dark default theme with light/system option, UI size, calm-motion flag.
- Consent-gated local saving (banner → localStorage; "Forget everything"); hash routing `#/topic-id` with back/forward.
- Topic model: trait + static meta + registry, question-phrased `related` links ("Where next?") and inline `Nav::link`.
- Topics:
  - ✅ draft `powers-of-ten` — 10⁻¹⁸…10¹⁸, isometric blocks/rods/slabs, number names, sugar-cube scale story, full SI prefix table.
  - ✅ draft `energy-ladder` — 14 examples from kT to ISS, log chart, 6 units (J, eV, kcal, kWh, sugar cubes, mosquitoes), "your own moving thing", compare-two with orders of magnitude.
  - ✅ draft `dice-bell` — n dice × s sides, histogram + scaled normal curve, auto-roll.
  - 📝 planned `numbers`, `gas-box`, `linear-maps`.
- Docs: BRIEF, CURRICULUM (proposal), DECISIONS, QUESTIONS, BUGS.

**Verified**
- Native build, `cargo clippy --all-targets` clean, `cargo test`: 12 sims + 5 app tests pass.
- Ran the native app (software GL) and checked home, energy ladder, powers of ten, dice by screenshot;
  fixed: content overflowing the visible panel, missing glyphs (added Hack as proportional fallback),
  "10 mm" → "1 cm" length formatting.
- Not verified: wasm build (B-001).

**Polish noticed, not done**
- Energy ladder in joules shows "4.12 zJ" — kids won't know zepto; maybe default to eV or sugar cubes below 1 J.
- Powers-of-ten 3D cubes are still isometric 2D (wgpu version planned).
- Dice: "Keep rolling" ignores the calm-motion setting.

**Next**
1. First `trunk serve` on a real machine; fix any wasm-only issues.
2. Answer QUESTIONS Q-001…Q-004 (name, language, license, ads vs donations).
3. Spine: `numbers` topic, wgpu 3D cubes for `powers-of-ten`, `doubling`.
