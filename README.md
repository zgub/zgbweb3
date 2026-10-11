# zgbweb

Interactive science & maths explainers for curious kids — Rust, egui, WebAssembly.

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
cargo run -p zgbweb-server -- dist 127.0.0.1:8090
```

Tests:

```sh
cargo test -p sims       # quick: pure logic
cargo test               # everything (builds wgpu, slow the first time)
```

## Layout

```
crates/sims     pure maths/physics, no UI, unit tests
crates/app      egui app: shell, pages, topics (one module per topic), theme, i18n
crates/app/locales   en.ftl, sk.ftl — every visible text (Project Fluent)
crates/app/assets    Nunito subset font (+ OFL licence)
crates/server   axum placeholder serving dist/
docs/           BRIEF · STATUS · CURRICULUM · DECISIONS · QUESTIONS · BUGS
index.html      trunk entry; canvas + reserved ad slot
tools/          build-font.sh (regenerates the font subset)
```

Adding a topic: see the doc comment at the top of `crates/app/src/topics/mod.rs`.

Dev shortcuts (native): `cargo run -- powers-of-ten/9` opens a topic (and state),
`ZGB_LANG=sk` picks the language, `ZGB_SIZE=400x860` the window size.
