# Game Lib Example

The policy for the game is implemented as a library in `src`. To run the game in a
browser using WebAssembly, run the `build.sh` script in `web-frontend`, and run
a webserver from `web-frontend/web`. To run the game in a terminal, build and
run `ansi-frontend`.

## WebAssembly dependencies

```
$ rustup target add wasm32-unknown-unknown
$ cargo install --git https://github.com/alexcrichton/wasm-gc
```
