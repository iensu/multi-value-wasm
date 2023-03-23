# Basic WASM lib built in Rust

Showcases multi-value return values in WebAssembly and how to use them for rudimentary error handling.

The multi-value feature is only supported by the `wasm32-unknown-unknown` target for now with the compiler flag specified in [.cargo/config](.cargo/config).

The [scripts folder](scripts) contains examples of how to call the library from node.js.

## How to build

Provided you have [Rustup](https://rustup.rs/) installed, the project should build with:

``` shell
$ cargo build --target=wasm32-unknown-unknown --release
```

The WASM module file can be found in the [target/wasm32-unknown-unknown/release](target/wasm32-unknown-unknown/release) folder.

Once you have built the WASM module you can try running the scripts in the [scripts folder](scripts).
