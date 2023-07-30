To run this example you need both `wasm-pack` and `node` installed.
Note that you will have to hardcode your credentials in `lib.rs` (although I've
never gotten it to the point where that actually matters).
Then, from the root of the project, run the following commands:

```
wasm-pack build --debug --target nodejs
node --experimental-wasm-modules index.mjs
```
