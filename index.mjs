import * as wasmMod from "./jsbind/my-component-nowasi.mjs";

Error.stackTraceLimit = Infinity;

const out = wasmMod.listTables();

console.log(out);
