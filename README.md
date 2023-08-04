This repo contains an example of running the AWS Rust SDK compiled to WASM in two
language runtimes, Node and Python. It achieves this by defining some utility
functions was [WebAssembly components](./wit/act-utils.wit), which are implemented in the host language
and used to provide system integration for the WASM sandbox. Currently the required
utils are an `http-client` to perform the network calls, a `time-client` to provide
the system time, a `print-client` to send output to `sdtdout`, and a `creds-client`
for providing the environment's AWS credentials. These utils are currently implemented in
[JavaScript](./jsbind/act-utils/) and in [Python](./demo.py) and demos are provided for each.
The demo calls the DynamoDB `listTables` API, so you will need at least one Dynamo table
in your account to see any useful output.

To run this example you need the following dependencies installed:

- [`cargo`](https://www.rust-lang.org/tools/install)
- [`wasm32-unknown-unknown` toolchain](https://rust-lang.github.io/rustup/concepts/toolchains.html)
- [`node`](https://nodejs.org/en/download)
- [`python`](https://www.python.org/downloads/)
- [`wasm-tools`](https://github.com/bytecodealliance/wasm-tools#installation)
- [`jco`](https://github.com/bytecodealliance/jco#installation)

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
