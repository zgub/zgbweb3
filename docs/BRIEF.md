# zgbweb3 — project brief

*Third attempt, hence the 3 (see D-016). Public name still open: Q-001.*

## What
A website of small interactive science and maths explainers — "sims and
visualisations" — written in Rust with egui, running in the browser as
WebAssembly. Each topic is something you poke at: sliders, dice, dragging
arrows, watching a gas settle.

## For whom
Curious kids roughly 9–14, plus the parent or teacher next to them. Started
as an energy-scale ladder for adults; retargeted to kids once it was clear
that orders of magnitude are the real barrier ("people don't understand
exponential scales").

## Principles
- **Interact, don't lecture.** A topic is a toy with a question attached.
- **Linked, not linear.** Topics link by the next question a kid would ask
  (see CURRICULUM.md).
- **Honest numbers.** Every number has its assumptions one click away.
- **Careful language.** No teleology ("dice don't want anything").
- **Private by default.** No tracking, no cookies. Local saving only after
  the visitor says yes.
- **Small and fast.** One wasm bundle, static hosting possible.

## Shape of the page
- Top bar: home, breadcrumb, settings (theme, size, motion, local saving), topic-menu toggle.
- Right side: topic menu grouped by strand, with maturity badges.
- Centre: the topic.
- Footer: privacy, license, about, version.
- Dark mode default; light / follow-system selectable.
- Reserved DOM slot above the canvas for a possible ad unit (off).

## Tech
- `crates/sims` — pure logic, no UI, unit-tested.
- `crates/app` — eframe/egui app; native window for dev, wasm for web (trunk).
  wgpu renderer: WebGPU where available, WebGL2 fallback.
- `crates/server` — axum; today serves `dist/`, later accounts/sessions.

## Not now (but designed not to block)
Accounts/login, server-side saves, ads, analytics, translations.
