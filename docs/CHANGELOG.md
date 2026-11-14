# Changelog

What changed in each version, for visitors and for Peter. Details and the
reasons behind them are in STATUS.md (per working session) and DECISIONS.md.
Versions are git tags `vX.Y.Z` (D-036); the version shows in the site footer.

## Unreleased (on `develop` and feature branches)

- Branch flow: `main` = released, `develop` = development, feature branches
  off `develop` (D-036).

## 0.7.4 — 2026-11-14 (branch `timeline`)

**Added**
- Timeline: a big "⚠ Not to scale!" / "Nie je v mierke!" banner at the top of
  the timeline, saying the line is squeezed and stretched on purpose (Peter).
  New shared widget `kit::banner`; DESIGN.md "Banner".

## 0.7.3 — 2026-11-14 (branch `timeline`)

**Added**
- Timeline: four ways to feel the stretched scale (Q-032): ×10 / ÷10 arcs
  between neighbouring round ticks; a bracket under the mouse saying how much
  time fits in that width; an "Honest spacing" slider that squashes the view
  to an even scale (try it on "Everything"); a new section "If all of time
  were a 1 km walk" (the Earth 329 m before the finish, the dinosaurs 4.79 m,
  our species 2.2 cm, writing 0.38 mm).
- Every event card links a Wikipedia article (English for now) for reading
  on (D-043). Deep link `/fair` (e.g. `#/timeline/whole/fair`).

## 0.7.2 — 2026-11-14 (branch `timeline`)

**Changed** (Peter's review)
- Timeline: the names (Big Bang, now, heat death) sit above the stripe, the
  jump buttons below the zoomed view, and two amber lines with a faint fill
  link the marked window on the stripe to the zoomed view, like a
  magnifying glass. DESIGN.md: picture type "overview and zoom".

## 0.7.1 — 2026-11-14 (branch `timeline`)

**Added**
- Timeline: the young Earth. The Moon-forming impact, the oldest piece of
  Earth (4.404 Gyr zircon), the late rain of asteroids (late veneer), the
  first oceans and where their water came from, our last common ancestor
  (LUCA, at a hot vent or spring) and the oldest sure traces of life (3.48 Gyr
  stromatolites). A fourth lane, Life. Uncertainty ranges drawn as dashed
  bars and given on the card. Jump button "The young Earth".

**Changed**
- "Zoom here" and event links zoom just far enough to separate the event
  from its nearest neighbour (crowded places get a closer view).
- Lanes are taller (three label rows); ticks are hidden in a part of the
  view too narrow to say how it counts.

## 0.7.0 — 2026-11-14 (branch `timeline`)

**Added**
- History: **Timeline of everything** / Časová os všetkého (draft, `#/timeline`,
  D-042). From the Planck time to the heat death of the universe on one
  zoomable axis: mouse wheel or pinch zooms, drag moves, double-click zooms in.
  An overview stripe of the whole axis, coloured by how much happens where,
  marks the current view; click or drag it to move there. "Jump to" buttons
  (everything, first moments, stars switch on, the Sun and the Earth, the far
  future). Three lanes, 15 starter events with short cards in both languages;
  rings for estimates and forecasts, the future shaded. Deep links
  `#/timeline/<event>` and `#/timeline/whole`.
- docs/TIMELINE.md: every event with its value and source.

Version note: 0.6.0 is the `linear` branch (not yet in `develop`); this
branch is off `develop` and skips to 0.7.0.

## 0.5.2 — 2026-10-28 (branch `menu-markers`)

**Changed** (Peter's review of percent tricks)
- Slovak title "Triky s percentami" (was "Percentové triky").
- The hundred squares show what each square is worth, on both pages.
- "Swap the numbers" with real examples (a tip of 8 % on 50 € → 50 % of 8),
  no x and y.
- Up-and-down section titled "Zdražie o 20 % a potom zlacnie o 20 %: cena sa
  nevráti na pôvodnú hodnotu!" / "Up 20 %, then down 20 %: the price does not
  come back!".

## 0.5.1 — 2026-10-28 (branch `menu-markers`)

**Added**
- Menu markers (D-039): ● new (this or the previous minor version), ✏ draft,
  ⏳ planned with a dimmed title; the hover says what each means. Same markers
  on the section pages. Long menu titles end in "…" instead of wrapping.
- Screenshot tool: `ZGB_HOVER=x,y` to capture tooltips.

## 0.5.0 — 2026-10-28 (branch `percent`)

**Added**
- Percent tricks / Percentové triky (draft), six sections: percent in your
  head (50, 25, 10, 5, 1, 20, 15 % with the hundred squares), swap the
  numbers (8 % of 50 = 50 % of 8), up and down is not back (+20 % −20 % =
  −4 %), discounts don't add (30 % + 20 % = 44 %, with a guess), taking VAT
  out (123 € ÷ 1.23, not −23 %), headline tricks (percentage points; "risk
  doubles" on 10 000 people). EN and SK.
- `draw` module: shared drawing pieces (unit grid, brackets, dashed lines).

## 0.4.1 — 2026-10-28 (branch `rule-of-three`)

**Changed**
- Rule of three after Peter's first look: the page is three framed sections
  (More of one, more of the other | More hands, less time | When it lies);
  examples and pictures are big choice bars; inputs are labelled like a form
  ("Počet maliarov: [4]"), so no plural word sits next to a number field;
  smallest text 12.5 pt; charts stack on a phone.

**Added**
- DESIGN.md: one design system for the whole site (D-038), and the shared
  widgets `kit::section`, `choice_bar`, `inset`, `takeaway`.

## 0.4.0 — 2026-10-28 (branch `rule-of-three`)

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
