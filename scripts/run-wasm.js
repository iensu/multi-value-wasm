const fs = require("fs");

const wasmBuffer = fs.readFileSync("target/wasm32-unknown-unknown/release/wasm_lib.wasm");

WebAssembly.instantiate(wasmBuffer).then(wasmModule => {
  const { add, div } = wasmModule.instance.exports;
  const sum = add(5, 6);
  console.log('WASM says:', sum);
  console.log('WASM says 42 / 2 =', div(42, 2));
  console.log('WASM says 42 / 0 =', div(42, 0));
});
