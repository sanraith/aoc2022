# aoc2022

Solutions for Advent of Code 2022 in Rust with a complementary web runner.  
Web runner will be available at (WIP): <https://sanraith.github.io/aoc2022/>  
All scaffold related requests are cached locally.

## Development environment setup

- Install rust-up from <https://rustup.rs/>
- Install npm from <https://nodejs.org/en/download/>
- Install wasm-pack from <https://rustwasm.github.io/wasm-pack/installer/>
- `npm install`
- Download your own inputs
  - manually from <https://adventofcode.com> to _aoc_lib/input/yearXXXX/dayXX.txt_,
  - or use `cargo run -- scaffold --inputs` to download them automatically.

## Running the solutions

Run the selected solutions and optionally copy the result to the clipboard. See config options in the generated `aoc_config.ini`.

- `cargo run`: Solve the last available day.
- `cargo run -- solve`: Solve all days in the current year.
- `cargo run -- ui`: Display a pretty UI to solve all days in the current year.
- `cargo run -- --help`: Display the available options.

## Testing

- `cargo test -p aoc-lib year2022`: Run tests for year 2022.
- `cargo test -p aoc-lib year2022::day04`: Run tests for year 2022 day 4.

## Scaffolding

Automatically download input and prepare test and solution files for the given day.

- `cargo run -- scaffold`: Scaffolds the latest available day.
- `cargo run -- scaffold --year 2021 1,2,5`: Scaffolds the specified days.
- `cargo run -- scaffold --help`: Display all available options.

## WASM build

The repo includes a Javascript + WASM based web runner to run solutions in the browser. It is available at: <https://sanraith.github.io/aoc2022/>

- Build site: `npm run build`
- Build rust source only: `wasm-pack build --target web --out-dir ../web/pkg aoc-ui`
- Local web-server: `npm run serve`

## Attributions

- Advent of Code: <https://adventofcode.com>
- Favicon: <https://iconarchive.com/show/simple-christmas-icons-by-gpritiranjan/christmas-tree-icon.html>
