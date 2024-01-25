# FGJ-2024

## Setup
> Initial setup will take like 20 minutes thanks to ungodly amount of dependencies for both `bevy` and `wasm-server-runner`

### Setup native developing environment
1. Run `cargo build` and wait for all of the 334 dependencies to compile
2. `cargo run`

### Setup web-assembly developing environment
1. Download wasm-runner `cargo install wasm-server-runner`
2. Use the wasm-runner `export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-server-runner`
3. Compile to wasm: `cargo build --target wasm32-unknown-unknown`
4. Setup localhost wasm-server: `cargo run --target wasm32-unknown-unknown` 
5. Or if you are (for some reason) on Windows: `wasm-server-runner \target\wasm32-unknown-unknown\debug\GGJ-2024.wasm`
6. Wasm is now hosted on `http://127.0.0.1:1334`

## TODO
- Use `trunk` as wasm-runner instead

## CI/CD pipeline
~~Copy and pasted~~ inspired by: https://plippe.github.io/blog/2021/07/12/rust-wasm-github.html

Documentation for actions:
- https://github.com/actions-rs/toolchain
- https://github.com/jetli/wasm-bindgen-action
- https://github.com/jetli/trunk-action
- https://github.com/peaceiris/actions-gh-pages (this one for some reason really just loves to just break)