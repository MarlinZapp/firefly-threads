## Building

- Use Rustup to install the Rust nightly toolchain.
- Install wasm-pack, for example with `cargo install wasm-pack`.
- Run `wasm-pack build --target web` to build the rust code into a js package.
- Use a web server without CORS to serve the files.
  - For example execute `sfz --port 8000 --coi -r` in the root directory.
