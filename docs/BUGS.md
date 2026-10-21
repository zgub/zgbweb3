# Bugs and known issues

Format: **B-nnn — title** (date, status: open / fixed in <commit>). Steps, expected, actual.

**B-001 — wasm build not yet verified** (2026-10-11, open)
The skeleton was type-checked and tested natively only; the build sandbox
could not download the `wasm32-unknown-unknown` std. First `trunk serve`
on a real machine may surface wasm-only compile errors (web.rs, main.rs wasm branch).
2026-10-21: still unverified after the v0.3 restructure (same sandbox limit).
First `trunk serve` (2026-10-21): "found more than one target artifact" — the app
crate built both a cdylib and a bin named `zgbweb3`. Fixed by dropping the cdylib
crate type. Further wasm-only errors may still show up.

**B-002 — "Forget everything" on native doesn't delete the file** (2026-10-11, open, low)
Native eframe keeps storage in its own dir and in memory; we only clear
browser localStorage. Native is a dev target, so: delete the storage dir by hand.

**B-003 — Text on the painter-drawn charts doesn't wrap** (2026-10-11, open, low; value column now sizes to its widest label)
Energy-ladder labels are right-aligned into a fixed column; long names can
overlap the bars on narrow screens.

**B-004 — eframe restores a saved native window size over `ZGB_SIZE`** (2026-10-11, open, dev-only)
`persist_window: false` stops saving but eframe still loads the saved size.
Workaround: delete `~/.local/share/zgbweb3/app.ron` before exact-size screenshots.

**B-005 — "→" comes from the Hack fallback font** (2026-10-11, obsolete 2026-10-21: Nunito removed by D-018; → still comes from Hack, which suits the stock egui look)
Nunito has no arrows; the hook links' → is drawn in Hack (slightly different
weight). Could add an arrow glyph the same way as superscript minus.
