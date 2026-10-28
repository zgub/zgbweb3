# zgbweb — project brief

*Repo and crate names stay zgbweb3 (D-016). Domain: zgub.net (D-019). Site
name: Q-001.*

## Why (D-031)
The ultimate goal: Peter and his kids gaining knowledge the fun way, learning
on the go together, and making it public so others can benefit too. The kids
shape it without meaning to: they ask, suggest and steer what Peter explores
next. Knowledge is the defence against ignorance and against manipulation by
those in power. Everything else in this brief serves that.

## What
Peter's personal site and portfolio, written in Rust with egui and compiled to
WebAssembly. The simulations and visualisations run in the browser on the
wgpu renderer (WebGPU where available, WebGL2 fallback). Everything on it is
public, including the things Peter is still learning.

**The core idea: put back together what school splits apart** (D-029).
Schools teach physics, chemistry, maths, history and philosophy in separate
classes. This site does the opposite: everything links to everything it
touches. The jolt from taking off a sweater links to a game about static,
to the electricity topic (voltage, current, resistance, capacitance), to the
energy ladder, and to the people who first worked it out.

**Available in English and Slovak** (D-013): every page, game and label, with
an EN | SK switch in the top bar. A new page is not done until both
languages exist.

## The five sections

### 1. Projects
Things Peter builds as a hobby: electronics, radio and LoRa, Raspberry Pi and
home lab, sensors, the Flysch notebook, the space game. Each is a short
write-up with photos, status and links.

### 2. Playground / Ihrisko
Interactive educational games. Every game links to the Explore topics it plays
with, and those topics link back. The topic is where you read; the game is
where you try.

### 3. Workbench / Dielňa (Slovak name proposed)
The section Peter is most interested in. Topics he is still learning, written
as he learns them: music theory, linear algebra, tensor calculus, statistics
and probability, and other advanced material. It is allowed to be unfinished
and to be wrong in public. Each entry carries a status (notes, working
through, understood) so a visitor knows how far to trust it. When an entry is
solid enough to explain to others, it can move to Explore.

### 4. Explore / Objavuj (working name, D-027)
Things Peter already understands and wants to explain.

**First: "What changes what"** (D-035, plan in WHAT-CHANGES-WHAT.md). How
one quantity depends on another, starting from real examples and naming the
school terms last: rule of three (built, v0.4.0), percent tricks, linear,
quadratic, inverse, exponential, logarithmic. Then trigonometry, then big and
small numbers.

Further topics:

- **Exponential growth and orders of magnitude.** Why doubling surprises
  everyone, why the step from a thousand to a million is as big as the step
  from one to a thousand, and how to read a log scale.
- **Numbers and scales.** Orders of magnitude you can feel. The puzzle-cube
  picture (D-014): a curious kid can imagine a million cubes, and going down
  is harder than going up, so the topic covers both directions.
- **Number bases.** Decimal, octal, hex, binary. Convert from one to another,
  then add, subtract, shift and divide, plus the tricks that make it quick.
- **Logic and gates.** What gates are and what you can build from them.
- **Electricity.** Voltage, current, resistance, capacitance and basic
  electronics; the place the sweater jolt, batteries and Peter's electronics
  projects link to.
- **Energy scales.** What an eV, a joule, an erg and a kWh are, and how they
  compare. The visitor picks the unit to measure in (joules by default), or
  picks any rung of the ladder (a flying mosquito, a teaspoon of sugar, a
  phone battery, a full 747 tank) as the unit. The ladder itself is meant to
  be browsable and intriguing on its own, mixing the obvious with the
  surprising: a phone battery holds about as much as a teaspoon of sugar.
  It runs from 1 eV to a supernova, with an "edge of the map" beyond (D-026).
  All values, assumptions and sources live in ENERGY-LADDER.md; Peter adds
  rungs as he goes.
- **Basic thermodynamics**, and further topics Peter understands well,
  added over time.

The topic graph and question-phrased links live in CURRICULUM.md and will
need updating for the new topic list.

### 5. History / História (D-030)
Not extensive. Short, interesting pieces about past minds and their ideas:
Faraday, Galileo, Descartes and others, with some philosophy. Each piece links
to the topics, games and projects its ideas live on in, and topics link back
("who figured this out?").

## Cross-linking (D-029)
- Every page links across sections, not only inside its own. Typical shape:
  a game ↔ the topic it plays with ↔ the History piece behind the idea ↔ a
  project that uses it ↔ a Workbench entry that goes deeper.
- Example: the sweater jolt (a few kilovolts, a few millijoules) → Electricity
  (voltage, capacitance) → Energy scales (a new rung) → History (the people
  behind electrostatics, Franklin to Faraday) → Projects (anything with a
  capacitor in it).
- Links are phrased as the next question a visitor would ask (D-011).
- Links are part of the content, not an afterthought: writing a page means
  writing its cross-links in both languages.

## For whom
Any curious mind, whether 6, 16, 26, 40 or 60 (D-020). A young child will have
a parent next to them; an adult will want the real numbers. Pages start plain
and simple and keep depth and assumptions one click away.

## Principles
- **Interact, don't lecture.** A topic is a toy with a question attached.
- **Linked, not linear.** Across sections as well as inside them (D-029).
- **Honest numbers.** Every number has its assumptions one click away.
- **Careful language.** No teleology ("dice don't want anything").
- **Content first.** The site should look good, but colour, effects and
  fanciness exist to underline a topic, never to overshadow it.
- **No names of private people, ever** (D-023). Peter's own choice. Family,
  friends, pupils, colleagues: never named. Historical figures and credited
  authors are named, as History and attribution need them.
- **No tracking, no targeted ads** (D-024). A conviction, stated as such on
  the privacy page, not a legal minimum. No analytics, no cookies; local
  saving only after the visitor says yes.
- **Credit where it is due** (D-025). Everything on the site is under
  copyright. Peter's own work may be reused with a link or credit. Other
  people's work is used only with proper credit and within its licence.
  Correct attribution is the default, not an extra.
- **Spread by word and links.** Not built to be found by crawlers (D-021).
- **Small and fast.** One wasm bundle, static hosting possible.

## Look and layout: the old zgbweb (D-018)
One look for the whole site, for every audience, taken from zgbweb2: the egui
demo-app feel, tidied and kept restrained.
- **Top bar:** egui theme buttons (Dark / Light / System) and the EN | SK switch.
  No File menu on the web.
- **Right panel:** fixed 200 px, not resizable, scrollable. Heading with a
  glyph ("🖱 Menu"), a separator, then for each of the five sections a heading
  with a glyph, a separator, and one full-width selectable row per page, each
  row with its own Unicode glyph. Glyphs must exist in the embedded fonts or
  they show as empty boxes, so each one is checked on a screenshot. Status
  markers (Explore, Workbench) go as small text at the end of the row.
- **Centre panel:** a heading, a separator, then the page. Simulations put
  their controls (sliders, checkboxes, buttons with a glyph) in a row on top,
  then a separator, then the canvas, as the old Game of Life page does.
  Cross-links sit at the end of the page and inline in the text.
- **Footer:** one line at the bottom left: site name, version, "Powered by
  egui and eframe", plus links to privacy, license and about.
- **Colour and effects:** the Ember palette in dark mode (D-034): warm
  charcoal with one amber accent; light mode is stock egui for now (Q-027).
  Restrained additions where they help a topic. Colour mostly lives inside
  the simulations.
- **Mobile:** below about 720 pt the menu collapses (Q-008).

## Landing page and About
A short About page exists but is not the landing page. The landing page is
chosen by Peter in configuration: the Explore index or the Playground index
(D-022).

## Privacy page
States plainly, in Peter's voice: no tracking, no analytics, no targeted ads,
no cookies, because he does not like them, not because a law says so. Then
what is stored locally and only after consent (D-006), and who runs the site
(Q-005).

## Far future: a video channel
Possibly, much later: a YouTube channel linked from the site, where the kids
explain experiments and concepts on camera, hands only, never faces, never
names (D-023). Open points in Q-025. Nothing in the site depends on it.

## Later: comments
Peter may add a simple comment section later. Whether, where, and with what
moderation and access is open (Q-020). Nothing in the first version depends on it.

## Tech (static-first, D-032)
- `crates/content` — the site map (five sections; every page's id, glyph,
  status and cross-links) and the English/Slovak Fluent catalogs, compiled in.
- `crates/sims` — pure logic, no UI, unit-tested.
- `crates/app` — eframe/egui app; native window for dev, wasm for web (trunk).
  wgpu renderer for the simulations and the 3D cube scenes.
- `crates/server` — thin axum: serves `dist/` with the crawler measures below;
  `/api` reserved. Postgres arrives with the first feature that stores data.
- Tests check the map (ids, links, cross-section links for finished pages),
  that both catalogs have the same keys, and that every glyph exists in the fonts.
- Fonts: egui's default fonts unless Slovak letters or the menu glyphs need
  more; to be settled later (Q-013).
- The domain must not be hard-coded anywhere: it may move to hunci.sk (D-019).

## Visibility and crawlers (D-021)
Peter wants the site shared by word and links, not harvested by search
engines or AI crawlers. What helps, from strongest to weakest:
1. **Canvas-only UI** (D-001): a crawler fetching the page sees almost no text.
   The texts exist only inside the wasm (D-032), readable by someone who
   unpacks it, but not lying around as plain files.
2. **Polite signals:** `robots.txt` disallowing everything, with named AI
   crawlers listed from a maintained list (the ai.robots.txt project); a
   `noindex, nofollow, noarchive` robots meta tag and an `X-Robots-Tag` header
   from the axum server; a text-and-data-mining opt-out header.
3. **Enforcement we control:** axum middleware that answers known crawler
   user agents with 403 and rate-limits per IP; or, if hosted behind
   Cloudflare, its "block AI bots" setting (depends on Q-006).
4. **Link previews:** keep a small static title and description in
   `index.html` so shared links still show a preview.

None of this stops a determined scraper; it reduces, it does not prevent.
With noindex, nobody finds the site by searching, which is the intent.
Licence note: "reuse with credit" (D-025) is what CC BY 4.0 says, and CC BY
does not forbid AI training as long as credit is given.

## Not now (but designed not to block)
Accounts/login, server-side saves, more languages, comments, the video
channel. Ads: targeted ads are ruled out (D-024); whether any ads at all is Q-004.
