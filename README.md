# zgbweb3

zgub.net — a personal site: hobby projects, games, things I am learning and
things I can explain. Rust, egui, WebAssembly. English and Slovak.

## Run

Native (fastest edit loop):

```sh
cargo run --release
```

Web (needs the wasm target and trunk once):

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
trunk serve            # http://127.0.0.1:8080
```

Production build + local server:

```sh
trunk build --release
cargo run -p zgbweb3-server -- dist 127.0.0.1:8090
```

Tests:

```sh
cargo test -p sims -p content   # quick: pure logic and the site map
cargo test                      # everything (builds wgpu, slow the first time)
```

## Layout

```
crates/content         site map: 5 sections, every page (id, glyph, status, cross-links)
crates/content/locales en.ftl, sk.ftl — every visible text (Project Fluent), compiled in
crates/sims            pure maths/physics, no UI, unit tests
crates/app             egui app: shell (top bar, menu, footer), pages, topics, theme, i18n
crates/server          thin axum server: dist/ + crawler-refusing headers; /api reserved
docs/                  BRIEF · STATUS · CHANGELOG · DECISIONS · QUESTIONS · BUGS · CURRICULUM · ENERGY-LADDER · WHAT-CHANGES-WHAT
index.html             trunk entry; canvas + reserved ad slot
```

Adding a page: see the doc comment at the top of `crates/content/src/lib.rs`;
a page with its own code also needs a module in `crates/app/src/topics/`.

Screenshots without a window or GPU:
`ZGB_SHOT=shot.png cargo run --example shot -- rule-of-three/percent/rulers`.

Dev shortcuts (native): `cargo run -- rule-of-three/fuel` opens a page (and state),
`cargo run -- explore` a section, `ZGB_LANG=sk` picks the language,
`ZGB_SIZE=400x860` the window size.
