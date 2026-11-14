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
- **Lanes:** Universe, Stars and galaxies, Sun and Earth, Life (v0.7.1),
  People (v0.7.5). Planned: Science (what was known when: the Jules Verne question).
- **Side view** (v0.7.6, Peter's idea): real time (linear across the view)
  against screen position; slope = squeeze = years per pixel here ÷ average.
  A squeeze f reads as a ruler tilted by θ with cos θ = 1/f; a stretch as a
  magnifying glass. `View`, `tilt_deg` in `sims::timeline`.
- **Ranges:** an event may carry the span of its estimates, drawn as a dashed
  bar under the lane line (one row per ranged event, so bars never overlap).
- **Importance:** 1 = turning point, 2 = big, 3 = detail. Drives the stripe;
  later also which labels show at which zoom.
- **Uncertain** events (models, forecasts) are drawn as rings, measured or
  well-dated ones as filled dots.

## Events (v0.7.0 starter set, v0.7.1 young Earth, v0.7.5 continents, dinosaurs, people)

| id | when | lane | imp | sure? | value and source |
|---|---|---|---|---|---|
| `start` | 5.39 × 10⁻⁴⁴ s after | Universe | 1 | — | Planck time; the axis starts here, not at zero (no physics at t = 0) |
| `transparent` | 380 000 yr after | Universe | 1 | yes | Recombination / CMB, Planck 2018 (z ≈ 1090, T ≈ 3000 K) |
| `first-stars` | 150 Myr after | Stars | 1 | model | Population III stars, models put them at ~100–200 Myr; none observed yet |
| `first-galaxies` | 300 Myr after | Stars | 2 | rough | JWST: JADES-GS-z14-0 at ~290 Myr (2024), MoM-z14 at ~280 Myr (2025, NASA/STScI) |
| `second-stars` | 500 Myr after | Stars | 2 | rough | Placement is a teaching choice: second-generation (Pop II) stars began within tens of Myr after the first; oldest Milky Way stars ~13 Gyr |
| `sun` | 4.567 Gyr ago | Earth | 2 | yes | Oldest meteorite inclusions (CAIs), 4567 Myr |
| `earth` | 4.54 Gyr ago | Earth | 1 | yes | Radiometric age of Earth and meteorites, 4.54 ± 0.05 Gyr |
| `moon` | 4.51 Gyr ago (range 4.53–4.43) | Earth | 1 | rough | Giant impact (Theia). Lunar zircons: 4.51 ± 0.01 Gyr (Barboni et al. 2017, Apollo 14); at least 4.46 Gyr (Greer et al. 2023, Apollo 17, atom-probe tomography); 4.43–4.53 with remelting at 4.35 (Nimmo et al. 2024) |
| `crust` | 4.404 Gyr ago | Earth | 2 | yes | Jack Hills detrital zircon, 4404 ± 8 Myr; δ¹⁸O hints at liquid water (Wilde et al., Nature 2001) |
| `late-veneer` | dot 4.35 Gyr ago (range 4.5–3.8) | Earth | 3 | debated | ~0.5 % of Earth's mass of chondritic material after core formation, inferred from highly siderophile elements (Pt, Au, Ir) in the mantle. Re-enrichment complete before 3.81 Ga (Coggon et al. 2013). The dot is a placement, not a date. Challenged by Anslow et al. 2026 (arXiv 2603.17961): metal from impactors > ~1 km should sink to the core |
| `oceans` | 4.3 Gyr ago | Earth | 2 | rough | Jack Hills zircons: liquid water before 4.3 Gyr (Mojzsis et al. 2001; Valley et al. 2014). Source of water debated: carbonaceous asteroids (D/H), some comets (12P/Pons-Brooks D/H matches oceans, Nature Astronomy 2025; 67P re-analysis 2024), or water in the building material from the start |
| `luca` | 4.2 Gyr ago (range 4.33–4.09) | Life | 2 | model | Last universal common ancestor, molecular clock on 700 genomes (Moody et al., Nature Ecology & Evolution 2024); probably at a hydrothermal vent or hot spring, anaerobic |
| `first-life` | 3.48 Gyr ago | Life | 1 | yes | Dresser Formation stromatolites, Pilbara (Hickman-Lewis et al. 2023); older claims disputed: Isua 3.7 Gyr (Greenland), Nuvvuagittuq 3.77–4.28 Gyr (Quebec) |
| `pangaea` | 335 Myr ago (range 335–175) | Earth | 1 | yes | Supercontinent assembled in the Carboniferous ~335 Ma, began to break up ~200 Ma (Central Atlantic), largely apart by ~175 Ma |
| `todays-map` | 2.8 Myr ago | Earth | 2 | rough | "The map from school": continents roughly in today's places; Isthmus of Panama closes ~2.8 Ma (O'Dea et al., Science Advances 2016; debated). India–Asia collision ~50 Ma |
| `first-dinosaurs` | 231 Myr ago | Life | 1 | rough | Oldest unequivocal dinosaurs ~230 Ma, Ischigualasto (Argentina), e.g. Eoraptor |
| `stegosaurus` | 155 Myr ago (range 155–150) | Life | 2 | yes | Late Jurassic, Morrison Formation; extinct ~80 Myr before T. rex appeared (USGS) |
| `trex` | 68 Myr ago (range 68–66) | Life | 2 | yes | Latest Cretaceous, coexisted with Triceratops |
| `dinosaurs-end` | 66.0 Myr ago | Life | 1 | yes | Chicxulub impact, K–Pg boundary 66.0 Ma (Renne et al., Science 2013); birds survive |
| `habilis` | 2.3 Myr ago (range 2.3–1.5) | People | 2 | yes | East and South Africa; overlapped with H. erectus ~0.5 Myr |
| `erectus` | 2.0 Myr ago (range 2.0 Myr–110 kyr) | People | 1 | yes | First outside Africa (Dmanisi, Georgia, 1.8 Ma); last on Java ~110 ka (Ngandong) |
| `neanderthals` | 400 kyr ago (range 400–40 kyr) | People | 2 | yes | Europe and western Asia; 1–2 % of non-African genomes today |
| `sapiens` | 300 kyr ago | People | 1 | yes | Jebel Irhoud, Morocco (Hublin et al., Nature 2017) |
| `denisovans` | 285 kyr ago (range 285–30 kyr) | People | 2 | rough | Known from Denisova Cave DNA (2010); Harbin skull (≥146 ka) identified as Denisovan by proteins and mtDNA (Fu et al., Science and Cell, June 2025; disputed by some) |
| `out-of-africa` | 60 kyr ago (range 60–43.5 kyr) | People | 1 | rough | Main dispersal 60–40 ka (MPI-EVA); Neanderthal gene flow ~50.5–43.5 ka, mean ~47 ka (Iasi et al., Science 2024; Sümer et al., Nature 2024) |
| `sumer` | 5 200 yr ago (range 6 000–5 000) | People | 1 | yes | Uruk period cities; proto-cuneiform ~3200 BC |
| `babylon` | 3 800 yr ago | People | 2 | yes | Hammurabi, reigned c. 1792–1750 BC (middle chronology); code stele with 282 laws, Louvre |
| `now` | now | Earth | 1 | — | — |
| `andromeda` | in 4.5 Gyr | Stars | 2 | forecast | Classic forecast ~4.5 Gyr; Sawala et al., Nature Astronomy 2025 (Hubble + Gaia): only ~50 % chance of a merger within 10 Gyr (ESA/Hubble heic2508, 2 June 2025). A 2026 follow-up with newer Gaia data reportedly put it higher (~90 %, median ~6.5 Gyr): **to verify before the card is final** |
| `red-giant` | in 5 Gyr | Earth | 1 | forecast | Sun leaves the main sequence in ~5 Gyr; red-giant-branch tip ~7.6 Gyr from now. Oceans boil away in ~1 Gyr (Sun ~10 % brighter) |
| `white-dwarf` | in 8 Gyr | Earth | 2 | forecast | After the red giant and planetary-nebula phases, ~8 Gyr from now |
| `last-stars` | in 10¹⁴ yr | Stars | 1 | forecast | End of star formation, red dwarfs burn out (Adams & Laughlin, "A dying universe", 1997) |
| `bh-stellar` | in 10⁶⁷ yr | Universe | 2 | theory | Hawking evaporation of a 1 solar-mass black hole, ~10⁶⁷ yr (lifetime ∝ M³) |
| `bh-biggest` | in 10¹⁰⁰ yr | Universe | 2 | theory | Supermassive black holes (~10¹¹ M☉) take up to ~10¹⁰⁰ yr |
| `heat-death` | in 10¹⁰⁶ yr | Universe | 1 | — | Not a date: an asymptotic approach. The dot marks the end of the drawn line |

## The walk (v0.7.3)

All of time as 1 km: metres before the finish = years ago ÷ 13.787 × 10⁹ ×
1000. One 75 cm step ≈ 10 million years.

| mark | years ago | before the finish | source |
|---|---|---|---|
| Earth forms | 4.54 × 10⁹ | 329 m | as `earth` above |
| dinosaurs die out | 66 × 10⁶ | 4.79 m | Chicxulub impact, K–Pg boundary 66.0 Ma (Renne et al., Science 2013) |
| our species | 300 000 | 2.2 cm | Jebel Irhoud fossils, ~300 ka (Hublin et al., Nature 2017) |
| writing | 5 200 | 0.38 mm | Sumerian proto-cuneiform, Uruk, ~3200 BC |
| dinosaur reign vs us | 165 Myr vs 300 kyr | 12 m vs 2.2 cm | 231 → 66 Ma; ×550 |

## Read-more links (D-043)

Each event's `wiki` is an English Wikipedia article name, chosen by hand.
Wikipedia is where a curious reader goes next; the table above holds the
actual sources.

## Next (Peter's plan: sparse end to end, then fill in)

1. Peter reviews the starter set and the feel of the zoom (`trunk serve`, `#/timeline`).
2. Life after the first traces (Peter, 2026-11-14): photosynthesis
   (cyanobacteria), the Great Oxidation Event (~2.4 Gyr ago: oxygen poisons
   much anaerobic life and changes the air), first complex cells, snowball
   Earth, Cambrian, dinosaurs and their end, first humans. Maybe the Late
   Heavy Bombardment (contested) and the Moon drifting away.
3. People after Babylon (Peter, 2026-11-14): other continents too (Egypt,
   Indus, China, the Americas, Africa, Europe; Peter wants help with these),
   farming (~11 700 years ago), Greeks, Romans, … up to now; then a Science
   lane ("what was known when"). Calendar years (BC/AD) for this part: Q-031c.
4. Future: Moon drifting away (3.8 cm per year), oceans boiling.
5. Labels by importance and zoom (level of detail), uncertainty as ranges.
