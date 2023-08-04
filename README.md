To run this example you need [`cargo`](https://www.rust-lang.org/tools/install),
the [`wasm32-unknown-unknown` toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html),
[`node`](https://nodejs.org/en/download),
[`python`](https://www.python.org/downloads/),
[`wasm-tools`](https://github.com/bytecodealliance/wasm-tools#installation), and
[`jco`](https://github.com/bytecodealliance/jco#installation) installed.

To successfully run the example you will need the `AWS_ACCESS_KEY_ID`,
`AWS_SECRET_ACCESS_KEY`, and the `AWS_SESSION_TOKEN` environment variables set.

To build the project, from the root of the project, run the following command:

```
make build
```

To execute the Node demo run:

```
node index.mjs
```

To execute the Python demo run:

```
python demo.py
```
