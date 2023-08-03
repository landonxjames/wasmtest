build:
	cargo build --target wasm32-unknown-unknown
	wasm-tools component new ./target/wasm32-unknown-unknown/debug/wasmtest.wasm -o my-component-nowasi.wasm
	jco transpile my-component-nowasi.wasm -o jsbind --map "act:utils/*=./act-utils/*.mjs"
	mv jsbind/my-component-nowasi.js jsbind/my-component-nowasi.mjs