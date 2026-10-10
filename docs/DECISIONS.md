# Decisions

Format: **D-nnn — title** (date, status). Why, and what it rules out.
Status: *accepted*, *tentative* (revisit before launch), *superseded by D-x*.

**D-001 — Rust + egui/eframe, compiled to wasm** (2026-10-11, accepted)
One language for sims and UI; same code runs natively for fast debugging.
Cost: canvas UI is not indexable by search engines and not a normal web page
(no text selection, ctrl-F, browser translate). Mitigated a little by the
`<meta description>` and `<noscript>` text.

**D-002 — Workspace split: `sims` (pure) / `app` (egui) / `server` (axum)** (2026-10-11, accepted)
Physics and maths live in `sims` with tests and no UI dependency, so they can
be checked natively and reused (e.g. by a future wgpu renderer or a CLI).

**D-003 — eframe with the wgpu renderer** (2026-10-11, accepted)
Needed for the planned 3D powers-of-ten cubes (custom wgpu paint callback,
same pattern as spatlers). egui-wgpu prefers WebGPU and falls back to WebGL2.
v0 topics draw with the egui painter only.

**D-004 — Topics as a trait + static metadata + registry** (2026-10-11, accepted)
`Topic { meta(), ui(ui, nav), save(), load() }`; `TopicMeta` is a static with
id, title, icon, group, blurb, maturity, related links. The id is permanent:
it is the URL route and the storage key. Tests enforce unique, URL-safe ids
and that every related link targets a registered topic.

**D-005 — Hash routing (`#/topic-id`)** (2026-10-11, accepted)
Shareable links and working back button without server routing; any static
host works. The app polls the hash each frame to follow back/forward.

**D-006 — Local saving only with consent; the answer itself is stored** (2026-10-11, accepted)
Nothing is written to localStorage until the visitor answers the banner.
"No" is stored too (as strictly necessary), so we don't ask every visit.
Withdrawing consent or "Forget everything" clears the origin's localStorage.
egui's own memory (window/scroll state) follows the same consent.
No CMP (CookieYes etc.) needed while there are no ads/analytics — see Q-004.

**D-007 — Ad slot lives in the HTML, not the canvas** (2026-10-11, tentative)
Ad networks insert DOM elements/iframes; they cannot draw into our canvas.
`index.html` has a hidden `<aside id="ad-top">` above the canvas. Turning ads
on will also require a consent platform (TCF) and an updated privacy page.

**D-008 — Server is a placeholder** (2026-10-11, accepted)
axum serving `dist/` with compression and basic headers. Accounts, sessions
(AAA), server-side saves are later; when they come, state already serialises
via serde per topic, so a server store can sit behind the same `save/load`.

**D-009 — No OS randomness; seeded `SmallRng`** (2026-10-11, accepted)
Avoids getrandom's wasm configuration and makes runs reproducible
("show me that again"). Seeds change on "Start over".

**D-010 — Code AGPL-3.0-or-later, content CC BY-SA 4.0** (2026-10-11, tentative)
Mirrors the Flysch choice. Open: Q-003.

**D-011 — Topic graph with question-phrased links** (2026-10-11, accepted)
`TopicMeta.related` renders a "Where next?" strip; `Nav::link` lets a topic
link inline from its text. Curriculum lives in CURRICULUM.md.
