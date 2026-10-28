# Changelog

What changed in each version, for visitors and for Peter. Details and the
reasons behind them are in STATUS.md (per working session) and DECISIONS.md.
Versions are git tags `vX.Y.Z` (D-036); the version shows in the site footer.

## Unreleased (on `develop` and feature branches)

- Branch flow: `main` = released, `develop` = development, feature branches
  off `develop` (D-036).

## 0.4.0 — 2026-10-28 (branch `rule-of-three`, in testing)

**Added**
- Rule of three / Trojčlenka (draft): "If 180 is 30 %, what is 100 %?" with
  four examples (percent, pancakes, fuel, the sale trap), a guess box, four
  pictures (pieces, two rulers, a straight line through zero, a hundred
  squares), "What school calls this", the inverse case (painters) and two
  cases where the rule lies (phone charging, a taxi start fee). EN and SK.
- "What changes what" at the top of Explore (D-035): percent tricks, same
  step, twice as fast, more hands, doubling and counting zeros as planned pages.
- Headless screenshots: `cargo run --example shot -- <route>` (D-037).

**Changed**
- Dark theme is the Ember palette: warm charcoal with an amber accent (D-034).
- The menu is Peter's list of 2026-10-25; every other page shows the "Nothing
  but sawdust" placeholder (D-033).

**Removed**
- Powers of ten, energy ladder and dice bell (old code; they come back rebuilt,
  D-033). Still in git history before 06c8e26.

**Fixed**
- Tests failed on Peter's machine because removed files were still on disk (B-006).

## 0.3.0 — 2026-10-21

**Changed**
- Five-section site (Projects, Playground, Workbench, Explore, History) in the
  old zgbweb look: top bar, fixed 200 pt right menu, one-line footer (D-017,
  D-018). Stock egui visuals; the 0.2 playful theme removed.
- Site map and texts moved into the `content` crate, compiled in (D-032).

**Added**
- Server: crawler measures (robots.txt, `X-Robots-Tag`, TDM opt-out, AI
  crawlers refused), `/api` reserved (D-021, D-032).
- Privacy page in Peter's voice (D-024), content licence CC BY 4.0 (D-010).

**Fixed (after release)**
- Trunk found two wasm artifacts; dropped the cdylib (B-001, 17b57db).

## 0.2.0 — 2026-10-11

**Added**
- English and Slovak with an EN | SK switch (Fluent, D-013).
- Powers of ten with puzzle cubes in 10 cm boxes (D-014); deep links (D-015).

**Changed**
- Playful look: indigo night / warm paper, Nunito (D-012, later superseded).

## 0.1.0 — 2026-10-11

**Added**
- Restart: workspace (`sims`, `app`, `server`), shell with right menu and
  footer, consent-gated local saving, hash routing.
- Draft topics: powers of ten, energy ladder, dice bell.
