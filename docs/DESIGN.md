# Design — how every page looks, reads and draws

The site must look like one thing, whoever builds the next page. This document
is the rulebook: colours, type, page anatomy, element types, plots and wording.
Decided in D-038; colours in D-034, layout in D-018.

**Rule zero.** Use the shared widgets in `crates/app/src/kit.rs`, the drawing
pieces in `crates/app/src/draw.rs` (unit grid, brackets, dashed lines, wrapped
text) and the colours in `theme::Palette`. If a page needs something that is not here, add it here and
to `kit` first, then use it. No one-off colours, sizes or widgets inside a page.

## 1. Colours (Ember, dark first)

| Role | Field | Colour | Used for |
|---|---|---|---|
| page | `bg` | `#1d1a17` | panels, insets |
| canvas | `deep` | `#141210` | behind every drawing and text field |
| section | `card` | `#27231f` | section frames |
| line | `line` | `#3b352f` | borders, separators |
| text | `text` | `#cfc6b8` | body text |
| strong | `strong` | `#f3ece1` | headings, the reference point in a drawing |
| weak | `weak` | `#8f8577` | captions, hints, axis labels, notes |
| accent | `accent` | `#f0a04b` amber | selection, links, the chosen option, **what we know** |
| note | `note` | `#e8b04f` | takeaways, 🐄 callouts |
| series 1 | `series[1]` | `#4fb3a2` teal | **what we ask** / the result, "right" |
| series 2 | `series[2]` | `#e06c5a` coral | traps, wrong answers, "careful" |
| series 3–5 | `series[3..]` | sky, plum, olive gold | further series in charts |
| ramp | `ramp_at(t)` | `#2a1b38` → `#f8d66d` | continuous values: speed, heat, height |

Meaning stays the same on every page: amber is given, teal is asked, coral is
a warning. Colour never carries meaning alone: there is always a label or a
shape too. Light mode: stock egui with darker series colours (Q-027).

## 2. Type

One font family: egui's bundled Ubuntu-Light, Noto Emoji and emoji-icon-font,
with Hack as the last fallback (Q-013). Emphasis comes from size and colour, not
weight. Every glyph must exist in the fonts (a test checks the chrome, page
and widget glyphs; add new ones to it).

| Use | Size (pt) | Where set |
|---|---|---|
| small: notes, status markers, footer | 12.5 | `TextStyle::Small` |
| body, buttons | 14.5 | `TextStyle::Body`, `Button` |
| caption above a choice bar | 13 | `kit::CAPTION` |
| choice-bar buttons | 16.5 | `kit::CHOICE_TEXT` |
| answer line | 20 | page |
| section title | 21 | `kit::SECTION_TITLE` |
| page heading | 22 | `TextStyle::Heading` |
| the page's question | 24 | page |
| inside drawings: tick labels | 12 | painter |
| inside drawings: annotations | 13–14 | painter |
| inside drawings: key statements | 15–16 | painter |

Nothing on the site is smaller than 12 pt.

## 3. Page anatomy

Top to bottom, every topic page:

1. **Heading**: glyph, title, status marker, one-line blurb (`topics::heading`).
2. **Sections** (`kit::section`): each part of the page in its own raised frame
   with a glyph and a title, 16 pt apart. A page with one idea has one section;
   rule of three has three: *More of one, more of the other* | *More hands,
   less time* | *When it lies*.
3. **Takeaway** (`kit::takeaway`): the one sentence to remember.
4. **For the grown-ups** fold-out (`kit::grown_ups`): formulas, assumptions, sources.
5. **Where next?**: cross-links phrased as questions (D-011, D-029).

Inside a section, in this order: the choice bar(s) → the situation in one
sentence → the question in large type → inputs → the guess box → the picture
choice bar → the canvas → *What school calls this* fold-out.

## 4. Elements

**Choice bar** (`kit::choice_bar`). The page's main switches: examples,
pictures, modes. A small weak caption ("Pick an example"), then big buttons
with a glyph and a name; the chosen one is filled amber with dark text.
Options that make no sense right now are greyed out with a hover reason, not
hidden. Never radio buttons or small selectable labels for main switches.

**Inputs** (form style). The quantity's name before the field, a fixed symbol
after it, never a word that has to change with the number:

| Good | Bad |
|---|---|
| Počet maliarov: [4]  Počet dní: [6] | [4] maliari [6] dní |
| Ak je počet maliarov [3], koľko to potrvá? | Ako dlho pre [3] maliarov |
| Distance [100] km → Petrol [6.5] l | [100] km take [6.5] l |

Symbols after a field: only units that never change (%, km, l, ml, €, J).
Sentences the page generates *from* the numbers ("3 maliari potrebujú 8 dní")
use proper plural forms via Fluent selectors. Rows wrap on narrow screens
(`horizontal_wrapped`), never a fixed-width grid wider than a phone.

**Guess box** (`kit::inset`). Guess first, then check or reveal. Verdicts: exact
(teal), close (note), too little / too much (coral). Until the answer is
revealed, every drawing shows "?" in its place.

**Canvas.** Filled `deep`, corner 3, content inset by 12 pt, full width of
the section. Height fits the content (no big empty areas).

**Fold-outs.** 🎓 *What school calls this / Ako to volajú v škole*: the school
words, introduced only after the visitor has played. 🔍 *For the grown-ups*:
formulas, assumptions, sources. Both closed by default.

**Notes.** 🐄 under each drawing, small and weak, where the picture simplifies.

**Takeaway.** Note-coloured frame with one or two sentences; one per page.

**Insets** (`kit::inset`): boxes inside a section (guess box, small charts),
`bg` on the section's `card`. Never a card inside a card of the same colour.

Buttons for actions (Check, Show the answer, Run, Start over) stay plain egui
buttons with a glyph where it helps: 👁 show, ▶ run, ⏸ pause, 🔄 start over.

## 5. Plots and pictures

**Colour roles** as in section 1: known amber, asked teal, warning coral, the
reference step or point in `strong` white, more series in `series` order.

**Line styles.** Solid 2.5 px = reality, measurement, the thing itself.
Dashed 2 px (6 on, 4 off) = a prediction, a model, "what the rule says".
Guide lines from a point to the axes: dashed 1.3 px in the point's colour.

**Points.** Filled circles, radius 5–6, in the role colour.

**Grid and axes.** Grid lines 1 px in `lerp(deep, line, 0.7)`; tick labels
12 pt weak, with units on the labels ("10 km", "25 %"). Axis ends rounded up
to 1, 2, 2.5 or 5 × 10ⁿ. Numbers through `I18n::num` (decimal comma in SK).

**Legend.** Inside the plot, top left: an 18 px line sample (solid or dashed)
and the label in the same colour. No legend boxes.

**Labels.** On or next to the thing they name, in its colour, on a `deep` tag
when they cross lines. Arrows → mean "goes with / corresponds to".

**Picture types used so far** (reuse before inventing):
- *pieces bar*: a whole cut into equal pieces, known pieces amber, one piece
  outlined white, brackets for known and asked parts;
- *double number line* ("two rulers"): two scales, partner values linked,
  the same ÷ and × arrows above and below;
- *line through zero* with a step triangle;
- *unit grid*: 10 × 10 squares for percent;
- *area grid*: rectangles of squares for "the product stays the same";
- *line chart* with a dashed model and a solid reality;
- *before/after bars*: bars for each step, the start level dashed across,
  the change written between the bars;
- *split bar*: one whole cut into two named parts (price without VAT + VAT),
  a wrong cut marked with a dashed coral line;
- *population grid*: 100 × 100 tiny squares, one per person, the cases as
  rings so they can be found (risk);
- *log bar chart* (energy ladder, coming back);
- 3D scenes in wgpu (powers of ten, later), same colour roles.

## 6. Wording

- Plain and concrete, for any curious mind (D-020). Slovak: tykanie.
- Start from the situation, name the school term last (D-035).
- No teleology: things don't "want" or "try" (BRIEF).
- Form-style labels next to inputs (section 4); generated sentences may use plurals.
- Questions as link text (D-011). Model alerts say what is simplified.
- Both languages written together; a page is not done without both (D-013).

## 7. Layout and screens

- Text column at most 1100 pt wide; the menu is 200 pt on the right.
- Every page works at 400 pt (phone): rows wrap, side-by-side insets stack
  below 560 pt, drawings use the full width and scale. A drawing with words
  beside it puts the words below it under 520 pt. Field labels never break
  inside (`Label::extend`); the whole field moves to the next row instead.
- Check every changed page by screenshot (D-037): 1280 wide and 400 wide,
  English and Slovak.

## 8. Checklist for a new page

1. Sections, choice bars, inputs, insets from `kit`; colours from `Palette`.
2. Known amber, asked teal, warnings coral; dashed = model, solid = reality.
3. Form-style inputs; no plural words next to fields.
4. Guess before answer where there is an answer; "?" in drawings until then.
5. 🎓 school words, 🔍 grown-ups, 🐄 notes, one takeaway, question links.
6. All glyphs in the font test; all texts in both catalogs.
7. Screenshots at 1280 and 400, EN and SK, looked at.
