import * as wasmMod from "./jsbind/act-utils.mjs";

Error.stackTraceLimit = Infinity;

const out = wasmMod.listTables();

console.log("Output in Node: \n", out);
