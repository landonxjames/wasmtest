To run this example you need [`cargo`](https://www.rust-lang.org/tools/install),
the [`wasm32-unknown-unknown` toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html),
[`node`](https://nodejs.org/en/download),
[`wasm-tools`](https://github.com/bytecodealliance/wasm-tools#installation), and
[`jco`](https://github.com/bytecodealliance/jco#installation) installed.

To successfully run the example you will need the `AWS_ACCESS_KEY_ID`,
`AWS_SECRET_ACCESS_KEY`, and the `AWS_SESSION_TOKEN` environment variables set.

Then, from the root of the project, run the following commands:

```
make build
node index.mjs
```
