# Working agreement (for Claude and any other contributor)

- Read `docs/STATUS.md` first; update it at the end of every working session.
- Record choices in `docs/DECISIONS.md` (D-nnn), open points in `docs/QUESTIONS.md` (Q-nnn),
  defects in `docs/BUGS.md` (B-nnn). Don't delete history; mark superseded/fixed.
- Physics and maths go in `crates/sims` with tests. The `app` crate only draws and wires.
- Topic ids are permanent (URL + storage key). Never rename one without a redirect.
- Every topic: a "For the grown-ups" fold-out with formula, assumptions and sources;
  a `related` list phrased as questions; no agent language ("wants", "seeks", "tries").
- Numbers shown to kids must be traceable: constants named, assumptions in `note`.
- Keep the wasm build working: no OS randomness, no threads, no `std::time::SystemTime`.
- Before committing: `cargo fmt`, `cargo clippy --all-targets`, `cargo test`.
- Icons/emoji must exist in egui's bundled fonts (Noto Emoji subset); unknown ones render as boxes.
  Check before using (e.g. 🧊 🫧 ✦ are missing; 🔟 💨 ★ 🐄 are fine).
- Native dev: `cargo run -- <topic-id>` opens that topic directly.
