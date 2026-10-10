# Status

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
