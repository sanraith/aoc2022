# aoc2022

Solutions for Advent of Code 2022 in Rust with a complementary web runner.

## Development environment setup

- Install rust-up from <https://rustup.rs/>
- Install wasm-pack from <https://rustwasm.github.io/wasm-pack/installer/>
- `npm install`

## WASM build

- Build site: `npm run build`
- Build rust source only: `wasm-pack build --target web --out-dir ../web/pkg aoc-ui`
- Local web-server: `npm run serve`

## Attributions

- Favicon: <https://iconarchive.com/show/simple-christmas-icons-by-gpritiranjan/christmas-tree-icon.html>
