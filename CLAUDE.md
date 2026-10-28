# Working agreement (for Claude and any other contributor)

- Read `docs/STATUS.md` first; update it at the end of every working session.
- Docs: BRIEF (what and why), STATUS (per session), CHANGELOG (per version), DECISIONS,
  QUESTIONS, BUGS, plus topic plans (CURRICULUM, ENERGY-LADDER, WHAT-CHANGES-WHAT).
  Add every user-visible change to CHANGELOG under "Unreleased" or the version being built.
- Record choices in `docs/DECISIONS.md` (D-nnn), open points in `docs/QUESTIONS.md` (Q-nnn),
  defects in `docs/BUGS.md` (B-nnn). Don't delete history; mark superseded/fixed.
- Physics and maths go in `crates/sims` with tests. The `app` crate only draws and wires.
- The site map lives in `crates/content` (`PAGES`): section, glyph, status, cross-links.
  Page ids are permanent (URL + storage + catalog key). Never rename a published one without a redirect.
- Cross-link across sections (D-029): a page marked `Ready` must link outside its own section (tested).
- No names of private people anywhere on the site (D-023).
- Every topic: a "For the grown-ups" fold-out with formula, assumptions and sources;
  `links` phrased as questions; no agent language ("wants", "seeks", "tries").
- Energy values come from `docs/ENERGY-LADDER.md` (D-026): record a rung there first.
- Numbers shown to kids must be traceable: constants named, assumptions in `note`.
- Keep the wasm build working: no OS randomness, no threads, no `std::time::SystemTime`.
- Before committing: `cargo fmt`, `cargo clippy --all-targets`, `cargo test`.
- Icons/emoji must exist in egui's bundled fonts; unknown ones render as boxes. A test reads
  the fonts' character maps for every page/section glyph and the chrome glyphs: add new ones
  to it (e.g. 🧊 🫧 ✦ are missing; 🔟 💨 ★ 🐄 are fine).
- No user-visible text in Rust code: add it to **both** `crates/content/locales/en.ftl` and `sk.ftl`
  (tests fail otherwise). Numbers go through `I18n::num` for the decimal comma.
  Slovak: tykanie, long scale (miliarda = 10⁹), real plural forms via Fluent selectors.
- Look (D-018 layout, D-034 colours): the old zgbweb layout in the Ember palette (dark), stock
  egui light. Chrome uses only `accent` for selection/links; `series` and `ramp` are for drawing.
  Plain buttons, no rounded cards.
- egui gotcha: never put an empty `ui.label("")` in `horizontal_wrapped` (B-007).
- Texts are compiled in (D-032); the server serves no text files. Nothing needs a database yet.
- Deep links: `cargo run -- rule-of-three/fuel/rulers/answer`, `ZGB_LANG=sk`, `ZGB_SIZE=400x860`.
- Screenshots without a window or GPU (D-037):
  `ZGB_SHOT=shots/x.png cargo run --example shot -- rule-of-three/sale/squares`.
  Look at every page you change, in both languages and at phone width (400×860).
- Branches (D-036): `main` = released, `develop` = development; work only on feature branches
  off `develop`, merged back into `develop`. Never commit straight to `main` or `develop`.
- Releases (D-036): bump `version` in the root Cargo.toml, tag `vX.Y.Z`; then no code changes
  while Peter tests; findings go to BUGS/QUESTIONS for the next version.
