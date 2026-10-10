# Curriculum — proposal v0 (2026-10-11)

Audience: curious kids ~9–14, and the grown-up next to them. Every topic must
work for a kid who arrives cold from a link, and every topic hands them on to
the next question. Nothing here is fixed; reorder freely.

## The idea in one paragraph

There is no fixed path. Topics form a **graph**: each one answers one question
and raises two or three more, and those questions are the links ("What does
10⁹ actually mean? →"). A kid on the energy ladder who wonders about 10⁹ lands
on Powers of ten; from there "0.001 — what kind of number is that?" leads to
What is a number?, and so on. A few **spine** topics carry most links; the
rest hang off them.

## Strands and topics

Status: ✅ in code (draft) · 📝 planned stub in code · 💡 idea only

### A. Numbers & scale — the spine
| id | topic | the question it answers | status |
|---|---|---|---|
| `numbers` | What is a number? | counting → 0 → negatives → fractions → √2 → reals → *i* (a number that turns by 90°) | 📝 |
| `powers-of-ten` | Powers of ten | what do 10³, 10⁹, 10⁻⁶ look like? (cubes, rods, slabs; atto…peta) | ✅ |
| `doubling` | Doubling beats adding | why exponential growth surprises everyone (rice on a chessboard, folding paper 42×, bacteria) | 💡 |
| `log-scale` | Counting zeros | log scales: why a ladder of ×10 steps is fair; dB, earthquakes, pH | 💡 |
| `estimate` | Guess like a scientist | Fermi estimates: how many sugar cubes fill your room? | 💡 |
| `units` | Why a metre? | SI base units, how they were defined then and now | 💡 |

### B. Ladders — one engine, many quantities
The energy ladder's chart and unit picker become a reusable **ladder** widget
(log axis + examples + custom item + "compare two").

| id | topic | status |
|---|---|---|
| `energy-ladder` | Energy: molecule → mosquito → car → 747 fuel → ISS | ✅ |
| `size-ladder` | Lengths: proton → cell → hair → you → Earth → galaxy | 💡 |
| `time-ladder` | Times: light crossing an atom → heartbeat → flysch layers → age of the universe (deep time) | 💡 |
| `mass-ladder` | Masses: electron → ant → elephant → ISS → Sun | 💡 |

### C. Chance & statistics
| id | topic | status |
|---|---|---|
| `dice-bell` | Why dice make a bell (one die is flat; sums make a hump — "the dice don't want anything") | ✅ |
| `galton` | The Galton board (falling balls, left/right at each pin) — pairs with building a real one | 💡 |
| `random-walk` | The drunkard's walk → diffusion (bridge to the gas box) | 💡 |
| `streaks` | Is this coin cheating? Streaks, luck and what "random" looks like | 💡 |
| `average` | Mean, median, and the billionaire in the bar | 💡 |

### D. Physics
| id | topic | status |
|---|---|---|
| `gas-box` | Gas in a box: start every ball at the same speed, watch the bell appear (thermalisation) | 📝 |
| `speed-squared` | Twice as fast, four times the crash: kinetic energy and braking distance | 💡 |
| `orbit` | Why the ISS doesn't fall (Newton's cannon) | 💡 |
| `power-energy` | Watts vs joules: kettle, phone charger, the kWh on the bill | 💡 |
| `light` | Colours, photons and why UV burns but red light doesn't | 💡 |
| `square-cube` | Why ants can lift 50× their weight and giants can't exist | 💡 |
| `entropy` | Why the milk never un-mixes (follows `gas-box`) | 💡 |

### E. Maths & geometry
| id | topic | status |
|---|---|---|
| `linear-maps` | Matrices move the plane (drag î and ĵ) | 📝 |
| `vectors` | Arrows you can add: swimming across a river | 💡 |
| `pythagoras` | Pythagoras and the number that isn't a fraction (√2) | 💡 |
| `pi` | π by throwing needles (Buffon) — chance meets geometry | 💡 |
| `complex-turn` | Multiplying by *i* turns things (payoff of `numbers`; leads to Mandelbrot later) | 💡 |

### F. Chemistry & life (overlaps the Sunflower class)
| id | topic | status |
|---|---|---|
| `atom-models` | Dalton → Thomson → Bohr → cloud: four maps of the same town | 💡 |
| `mole` | 6 × 10²³: how chemists count atoms by weighing (powers-of-ten payoff) | 💡 |
| `cell-scale` | How small is a cell, a virus, a protein (size-ladder slice) | 💡 |

### Later, for older kids
Fourier (sound as sines), waves, chaos (double pendulum), Game of Life,
Mandelbrot (needs `complex-turn`).

## The link graph (spine only)

```
energy-ladder ──"what's 10⁹?"──▶ powers-of-ten ──"what's 0.001?"──▶ numbers
     │                                  │                               │
     │"why is heat jiggling?"           │"how fast can things grow?"    │"a number that turns?"
     ▼                                  ▼                               ▼
  gas-box ◀──"same bell!"── dice-bell  doubling                  complex-turn ── linear-maps
     │
     └──"why no un-mixing?"──▶ entropy
```

## Cross-cutting features (apply to every topic)

- **Hooks, not menus.** Links are phrased as the question a kid would ask.
  Implemented: `TopicMeta.related` (shown as "Where next?") and `Nav::link`
  for inline links. A test guarantees every link points at a real topic.
- **🐄 Model alert** (the "spherical cow"/krava card). A small consistent
  marker wherever a topic simplifies — "a mosquito is 2 mg flying at
  0.4 m/s", "gas molecules are perfect billiard balls". Same marker on every
  page so kids learn to spot it. Proposed, not built.
- **For the grown-ups** fold-out on every topic: the formula, the assumptions,
  the sources. Implemented as `widgets::grown_ups`.
- **No agent words.** Dice don't *want*, molecules don't *seek*, systems don't
  *try* to minimise energy. Review checklist item for every topic text.
- **Predict, then test.** Where possible, ask the kid to guess before showing
  (e.g. "How many mosquitoes is one sugar cube?" → slider → reveal).

## Proposed order of work

1. **Spine first:** finish `powers-of-ten` (wgpu 3D cubes), build `numbers`
   and `doubling`. These are what everything else links to.
2. **Chance:** polish `dice-bell`, add `galton`, then `gas-box` (thermalisation
   — reuses the bell idea).
3. **Ladders:** extract the ladder widget, add `size-ladder` and `time-ladder`.
4. **Maths:** `linear-maps`, `pythagoras`, `complex-turn`.
5. Everything else by demand.
