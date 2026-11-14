# Timeline of everything — events, values, sources

The plan and the reference list for the `timeline` page (D-042), like
ENERGY-LADDER.md is for the energy ladder: every event shown on the site is
recorded here first, with its date, how sure we are, and where the number
comes from. Code: `crates/sims/src/timeline.rs` (`EVENTS`); texts:
`tl-ev-<id>` in both catalogs.

## How it works

- **Axis.** Past: `u = log₁₀ t − log₁₀ (ago + 1 yr)` (t = time since the Big
  Bang, ago = time before now). Future: `u = u_now + log₁₀ (1 + ahead / 1 yr)`.
  Near the Big Bang it is a log of time since the start, near now a log of
  time ago, so both the first second and the last thousand years get room.
  Zoomed in, it is locally almost linear. Starts at the Planck time
  (5.39 × 10⁻⁴⁴ s), ends 10¹¹⁰ years from now.
- **Dates are stored in the form they are known**: `After` (since the Big
  Bang), `Ago`, `Ahead`. Never a single absolute number: the age of the
  universe is uncertain by ±20 million years.
- **Ticks** count "since the Big Bang" up to 1 billion years after it, then
  "ago", then "from now" (`SWITCH_AFTER_YR`).
- **Overview stripe** (Peter's idea, 2026-11-14): the whole axis, always,
  coloured by event density (importance-weighted, smoothed, scaled to the
  busiest place). Shows where to zoom and which stretches are empty. Click or
  drag moves the view there.
- **Lanes:** Universe, Stars and galaxies, Sun and Earth. Planned: Life,
  People, Science (what was known when: the Jules Verne question).
- **Importance:** 1 = turning point, 2 = big, 3 = detail. Drives the stripe;
  later also which labels show at which zoom.
- **Uncertain** events (models, forecasts) are drawn as rings, measured or
  well-dated ones as filled dots.

## Events (v0.7.0 starter set, sparse on purpose)

| id | when | lane | imp | sure? | value and source |
|---|---|---|---|---|---|
| `start` | 5.39 × 10⁻⁴⁴ s after | Universe | 1 | — | Planck time; the axis starts here, not at zero (no physics at t = 0) |
| `transparent` | 380 000 yr after | Universe | 1 | yes | Recombination / CMB, Planck 2018 (z ≈ 1090, T ≈ 3000 K) |
| `first-stars` | 150 Myr after | Stars | 1 | model | Population III stars, models put them at ~100–200 Myr; none observed yet |
| `first-galaxies` | 300 Myr after | Stars | 2 | rough | JWST: JADES-GS-z14-0 at ~290 Myr (2024), MoM-z14 at ~280 Myr (2025, NASA/STScI) |
| `second-stars` | 500 Myr after | Stars | 2 | rough | Placement is a teaching choice: second-generation (Pop II) stars began within tens of Myr after the first; oldest Milky Way stars ~13 Gyr |
| `sun` | 4.567 Gyr ago | Earth | 2 | yes | Oldest meteorite inclusions (CAIs), 4567 Myr |
| `earth` | 4.54 Gyr ago | Earth | 1 | yes | Radiometric age of Earth and meteorites, 4.54 ± 0.05 Gyr |
| `now` | now | Earth | 1 | — | — |
| `andromeda` | in 4.5 Gyr | Stars | 2 | forecast | Classic forecast ~4.5 Gyr; Sawala et al., Nature Astronomy 2025 (Hubble + Gaia): only ~50 % chance of a merger within 10 Gyr (ESA/Hubble heic2508, 2 June 2025). A 2026 follow-up with newer Gaia data reportedly put it higher (~90 %, median ~6.5 Gyr): **to verify before the card is final** |
| `red-giant` | in 5 Gyr | Earth | 1 | forecast | Sun leaves the main sequence in ~5 Gyr; red-giant-branch tip ~7.6 Gyr from now. Oceans boil away in ~1 Gyr (Sun ~10 % brighter) |
| `white-dwarf` | in 8 Gyr | Earth | 2 | forecast | After the red giant and planetary-nebula phases, ~8 Gyr from now |
| `last-stars` | in 10¹⁴ yr | Stars | 1 | forecast | End of star formation, red dwarfs burn out (Adams & Laughlin, "A dying universe", 1997) |
| `bh-stellar` | in 10⁶⁷ yr | Universe | 2 | theory | Hawking evaporation of a 1 solar-mass black hole, ~10⁶⁷ yr (lifetime ∝ M³) |
| `bh-biggest` | in 10¹⁰⁰ yr | Universe | 2 | theory | Supermassive black holes (~10¹¹ M☉) take up to ~10¹⁰⁰ yr |
| `heat-death` | in 10¹⁰⁶ yr | Universe | 1 | — | Not a date: an asymptotic approach. The dot marks the end of the drawn line |

## Next (Peter's plan: sparse end to end, then fill in)

1. Peter reviews the starter set and the feel of the zoom (`trunk serve`, `#/timeline`).
2. Earth and life: Moon-forming impact, oceans, first life, oxygen, Cambrian,
   dinosaurs and their end, first humans (Q-031 decides the lanes).
3. People: farming, writing (Sumer), Greeks, Romans, … up to now; then a
   Science lane ("what was known when").
4. Future: Moon drifting away (3.8 cm per year), oceans boiling.
5. Labels by importance and zoom (level of detail), uncertainty as ranges.
