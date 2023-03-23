// Run with --experimental-wasm-modules flag
import { add, div } from '../target/wasm32-wasi/release/wasm_lib.wasm'

console.log('WASM says  5 + 6 =', add(5, 6));
console.log('WASM says 42 / 2 =', div(42, 2));
console.log('WASM says 42 / 0 =', div(42, 0));
