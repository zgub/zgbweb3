# What changes what — plan (2026-10-28, D-035)

A group of Explore pages about how one quantity depends on another: the
things school calls functions, introduced the other way round. Each page
starts from a real situation, lets the visitor play with it, and only then
names what they just saw ("what school calls this"). Status: proposal;
names open in Q-026. Order of work (Peter, 2026-10-28): this group first,
then trigonometry, then big and small numbers. Built so far: rule of three
(v0.4.0, draft). Page ids are neutral so titles can change: `rule-of-three`,
`percent`, `linear`, `quadratic`, `inverse`, `exponential`, `logarithm`.

## Why
Many people are fine with big and small numbers but struggle with the basics:
rule of three, percentages, and the shapes of linear, quadratic, inverse,
exponential and logarithmic relationships. Percentages in particular are a
favourite tool for misleading people (D-031).

## Where
- **Explore**: the topics, under a small menu label "What changes what /
  Čo mení čo" inside the Explore section.
- **Playground**: games that use the same curves and link back (below).

## Pages

| Menu (EN) | Menu (SK) | Page heading | School name | Opening example |
|---|---|---|---|---|
| ⚖ Rule of three | ⚖ Trojčlenka | If 180 is 30 %, what is 100 %? | direct / inverse proportion | 180 = 30 % |
| 💯 Percent tricks | 💯 Percentové triky | Percent tricks | percentages | sale, VAT, "risk doubles" |
| 📏 Same step | 📏 Rovnaký krok | Same step every time | linear function | price per kilo, taxi fare |
| 🚗 Twice as fast | 🚗 Dvakrát rýchlejšie | Twice as fast, four times the crash | quadratic function | car energy at 50 vs 130 km/h |
| 👥 More hands | 👥 Viac rúk | More hands, less time | inverse proportion, hyperbola | painters and days, speed and travel time |
| 📈 Doubling | 📈 Zdvojovanie | Doubling beats adding | exponential function | +100 a day vs ×2 a day; halving |
| 🔢 Counting zeros | 🔢 Počítame nuly | Counting the zeros | logarithm | decibels, pH, octaves |

Menu rows stay short to fit the 200 px menu; the page heading carries the
full name or the question.

## The rhythm of every page
1. A scene with one control (a car and a speed slider), no axes yet.
2. Guess first ("double the speed: 2×, 3× or 4× the energy?"), then try.
3. Each try leaves a dot on a graph beside the scene; the curve appears from
   the visitor's own tries.
4. A box **"What school calls this / Ako to volajú v škole"** names it:
   the function, its formula, and the domain (definičný obor) and range
   (obor hodnôt), arrived at through questions the visitor already met
   ("can a speed be negative? can you have half a painter?").
5. A 🐄 model alert wherever the example simplifies.
6. "Where next?" links phrased as questions, across sections (D-011, D-029).

## Rule of three (built in v0.4.0)
One question, four pictures the visitor can switch between:
1. **Ten pieces** — 30 % is three of ten pieces = 180, one piece is 60,
   all ten are 600.
2. **Two rulers** (a double number line) — percent on top, amount below;
   whatever you do on top (÷ 3, then × 10) you do below.
3. **A straight line through zero** — every 10 % adds the same 60.
4. **A hundred squares** — one square is 1 % = 6.

Examples page ("the same trick, everywhere"):
- Pancakes for 6 when the recipe is for 4 (250 ml → 375 ml).
- Fuel: 6.5 l per 100 km, 340 km → 22.1 l.
- The sale trap: 42 € after 30 % off; the old price is 60 €, not 54.60 €.
- Inverse: 4 painters × 6 days = 24 painter-days → 3 painters need 8 days
  (the work is the area).
- When it lies: phone charging slows near full (typical shape, not
  measured); a taxi with a start fee is a straight line, but not through zero.

Closing line: rule of three works when twice the one means twice the other,
a straight line through zero.

## Percent tricks (planned)
10×10 grid as the base picture, bars for before/after:
x % of y = y % of x; +20 % then −20 % is not back; discounts don't add;
taking VAT out is not "minus the VAT rate" (check the current Slovak rate);
percent vs percentage point; "risk doubles" on a grid of 10,000 dots;
quick tricks (10 %, 5 %, 15 %).

## Real examples per relationship
- **Same step:** price per kilo (rule of three is a line through zero), taxi
  start fee (the q in y = kx + q), Ohm's law → Electricity.
- **Twice as fast:** car energy (ENERGY-LADDER rungs: 50 km/h 1.45×10⁵ J,
  130 km/h 9.8×10⁵ J; 2.6× the speed, 6.8× the energy), braking distance,
  pizza area; History: Galileo and falling bodies.
- **More hands:** fixed trip and speed, shared cottage, painters; inverse
  rule of three; later gas pressure × volume.
- **Doubling:** adder vs doubler race, halving (coffee cooling, decay,
  medicine), savings and inflation with the rule of 72; 🐄 real growth runs
  out of something.
- **Counting zeros:** how many doublings to a million, decibels, earthquakes,
  pH, octaves (→ Workbench: music theory), bridge to big and small numbers.

## Playground games (planned)
- **Guess the rule / Uhádni pravidlo** — a machine returns a number for each
  number you feed it; guess which shape it follows.
- **The race / Preteky** — adder, squarer, doubler: who leads after 5, 10, 20 steps?
- **Discount shop / Zľavový obchod** — two offers, which one is really cheaper?

## Texts
All texts in the Fluent catalogs (`crates/content/locales/{en,sk}.ftl`),
both languages, cross-links included (D-013, D-032). No teleology in the
wording. Colours from Ember (D-034): the known value amber, the asked value
teal, warnings coral.
