# Status

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
