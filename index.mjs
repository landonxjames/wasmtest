import * as wasmMod from "./pkg/wasmtest.js";

Error.stackTraceLimit = Infinity;

const out = await wasmMod.list_tables();

console.log(out);
