build:
	npm install
	pip install -r requirements.txt
	cargo build --target wasm32-unknown-unknown
	wasm-tools component new ./target/wasm32-unknown-unknown/debug/wasmtest.wasm -o act-utils.wasm
	jco transpile act-utils.wasm -o jsbind --map "act:utils/*=./act-utils/*.mjs"
	mv jsbind/act-utils.js jsbind/act-utils.mjs
	python -m wasmtime.bindgen act-utils.wasm --out-dir pybind/


build-release:
	npm install
	pip install -r requirements.txt
	cargo build --target wasm32-unknown-unknown --release
	wasm-tools component new ./target/wasm32-unknown-unknown/release/wasmtest.wasm -o act-utils.wasm
	jco transpile act-utils.wasm -o jsbind --map "act:utils/*=./act-utils/*.mjs"
	mv jsbind/act-utils.js jsbind/act-utils.mjs
	python -m wasmtime.bindgen act-utils.wasm --out-dir pybind/