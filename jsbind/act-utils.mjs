import { getCreds } from './act-utils/creds-client.mjs';
import { makeHttpRequest } from './act-utils/http-client.mjs';
import { printHost } from './act-utils/print-client.mjs';
import { getSysTimeUnixMillis } from './act-utils/time-client.mjs';

const base64Compile = str => WebAssembly.compile(typeof Buffer !== 'undefined' ? Buffer.from(str, 'base64') : Uint8Array.from(atob(str), b => b.charCodeAt(0)));

class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;
let _fs;
async function fetchCompile (url) {
  if (isNode) {
    _fs = _fs || await import('fs/promises');
    return WebAssembly.compile(await _fs.readFile(url));
  }
  return fetch(url).then(WebAssembly.compileStreaming);
}

const instantiateCore = WebAssembly.instantiate;

const toUint64 = val => BigInt.asUintN(64, val);

function toUint16(val) {
  val >>>= 0;
  val %= 2 ** 16;
  return val;
}

const utf8Decoder = new TextDecoder();

const utf8Encoder = new TextEncoder();

let utf8EncodedLen = 0;
function utf8Encode(s, realloc, memory) {
  if (typeof s !== 'string') throw new TypeError('expected a string');
  if (s.length === 0) {
    utf8EncodedLen = 0;
    return 1;
  }
  let allocLen = 0;
  let ptr = 0;
  let writtenTotal = 0;
  while (s.length > 0) {
    ptr = realloc(ptr, allocLen, 1, allocLen + s.length);
    allocLen += s.length;
    const { read, written } = utf8Encoder.encodeInto(
    s,
    new Uint8Array(memory.buffer, ptr + writtenTotal, allocLen - writtenTotal),
    );
    writtenTotal += written;
    s = s.slice(read);
  }
  if (allocLen > writtenTotal)
  ptr = realloc(ptr, allocLen, 1, writtenTotal);
  utf8EncodedLen = writtenTotal;
  return ptr;
}

let exports0;

function lowering0() {
  const ret = getSysTimeUnixMillis();
  return toUint64(ret);
}
let exports1;
let memory0;
let realloc0;

function lowering1(arg0) {
  const ret = getCreds();
  const {accessKeyId: v0_0, secretAccessKey: v0_1, sessionToken: v0_2 } = ret;
  const ptr1 = utf8Encode(v0_0, realloc0, memory0);
  const len1 = utf8EncodedLen;
  dataView(memory0).setInt32(arg0 + 4, len1, true);
  dataView(memory0).setInt32(arg0 + 0, ptr1, true);
  const ptr2 = utf8Encode(v0_1, realloc0, memory0);
  const len2 = utf8EncodedLen;
  dataView(memory0).setInt32(arg0 + 12, len2, true);
  dataView(memory0).setInt32(arg0 + 8, ptr2, true);
  const ptr3 = utf8Encode(v0_2, realloc0, memory0);
  const len3 = utf8EncodedLen;
  dataView(memory0).setInt32(arg0 + 20, len3, true);
  dataView(memory0).setInt32(arg0 + 16, ptr3, true);
}

function lowering2(arg0, arg1) {
  const ptr0 = arg0;
  const len0 = arg1;
  const result0 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  printHost(result0);
}

function lowering3(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
  let enum0;
  switch (arg0) {
    case 0: {
      enum0 = 'GET';
      break;
    }
    case 1: {
      enum0 = 'POST';
      break;
    }
    case 2: {
      enum0 = 'PUT';
      break;
    }
    case 3: {
      enum0 = 'DELETE';
      break;
    }
    case 4: {
      enum0 = 'OPTIONS';
      break;
    }
    case 5: {
      enum0 = 'HEAD';
      break;
    }
    default: {
      throw new TypeError('invalid discriminant specified for Methods');
    }
  }
  const ptr1 = arg1;
  const len1 = arg2;
  const result1 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr1, len1));
  const len4 = arg4;
  const base4 = arg3;
  const result4 = [];
  for (let i = 0; i < len4; i++) {
    const base = base4 + i * 16;
    const ptr2 = dataView(memory0).getInt32(base + 0, true);
    const len2 = dataView(memory0).getInt32(base + 4, true);
    const result2 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr2, len2));
    const ptr3 = dataView(memory0).getInt32(base + 8, true);
    const len3 = dataView(memory0).getInt32(base + 12, true);
    const result3 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr3, len3));
    result4.push([result2, result3]);
  }
  const ptr5 = arg5;
  const len5 = arg6;
  const result5 = new Uint8Array(memory0.buffer.slice(ptr5, ptr5 + len5 * 1));
  const ret = makeHttpRequest({
    method: enum0,
    uri: result1,
    headers: result4,
    body: result5,
  });
  const {status: v6_0, body: v6_1 } = ret;
  dataView(memory0).setInt16(arg7 + 0, toUint16(v6_0), true);
  const val7 = v6_1;
  const len7 = val7.byteLength;
  const ptr7 = realloc0(0, 0, 1, len7 * 1);
  const src7 = new Uint8Array(val7.buffer || val7, val7.byteOffset, len7 * 1);
  (new Uint8Array(memory0.buffer, ptr7, len7 * 1)).set(src7);
  dataView(memory0).setInt32(arg7 + 8, len7, true);
  dataView(memory0).setInt32(arg7 + 4, ptr7, true);
}
let exports2;
let postReturn0;

function listTables() {
  const ret = exports1['list-tables']();
  let variant2;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      const ptr0 = dataView(memory0).getInt32(ret + 4, true);
      const len0 = dataView(memory0).getInt32(ret + 8, true);
      const result0 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr0, len0));
      variant2= {
        tag: 'ok',
        val: result0
      };
      break;
    }
    case 1: {
      const ptr1 = dataView(memory0).getInt32(ret + 4, true);
      const len1 = dataView(memory0).getInt32(ret + 8, true);
      const result1 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr1, len1));
      variant2= {
        tag: 'err',
        val: result1
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  postReturn0(ret);
  if (variant2.tag === 'err') {
    throw new ComponentError(variant2.val);
  }
  return variant2.val;
}

const $init = (async() => {
  const module0 = fetchCompile(new URL('./act-utils.core.wasm', import.meta.url));
  const module1 = base64Compile('AGFzbQEAAAABFQNgAX8AYAJ/fwBgCH9/f39/f39/AAMEAwABAgQFAXABAwMHGAQBMAAAATEAAQEyAAIIJGltcG9ydHMBAAovAwkAIABBABEAAAsLACAAIAFBAREBAAsXACAAIAEgAiADIAQgBSAGIAdBAhECAAsALglwcm9kdWNlcnMBDHByb2Nlc3NlZC1ieQENd2l0LWNvbXBvbmVudAYwLjEzLjEApwEEbmFtZQATEndpdC1jb21wb25lbnQ6c2hpbQGKAQMAKWluZGlyZWN0LWFjdDp1dGlscy9jcmVkcy1jbGllbnQtZ2V0LWNyZWRzASppbmRpcmVjdC1hY3Q6dXRpbHMvcHJpbnQtY2xpZW50LXByaW50LWhvc3QCMGluZGlyZWN0LWFjdDp1dGlscy9odHRwLWNsaWVudC1tYWtlLWh0dHAtcmVxdWVzdA');
  const module2 = base64Compile('AGFzbQEAAAABFQNgAX8AYAJ/fwBgCH9/f39/f39/AAIfBAABMAAAAAExAAEAATIAAgAIJGltcG9ydHMBcAEDAwkJAQBBAAsDAAECAC4JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQGMC4xMy4xABwEbmFtZQAVFHdpdC1jb21wb25lbnQ6Zml4dXBz');
  Promise.all([module0, module1, module2]).catch(() => {});
  ({ exports: exports0 } = await instantiateCore(await module1));
  ({ exports: exports1 } = await instantiateCore(await module0, {
    'act:utils/creds-client': {
      'get-creds': exports0['0'],
    },
    'act:utils/http-client': {
      'make-http-request': exports0['2'],
    },
    'act:utils/print-client': {
      'print-host': exports0['1'],
    },
    'act:utils/time-client': {
      'get-sys-time-unix-millis': lowering0,
    },
  }));
  memory0 = exports1.memory;
  realloc0 = exports1.cabi_realloc;
  ({ exports: exports2 } = await instantiateCore(await module2, {
    '': {
      $imports: exports0.$imports,
      '0': lowering1,
      '1': lowering2,
      '2': lowering3,
    },
  }));
  postReturn0 = exports1['cabi_post_list-tables'];
})();

await $init;

export { listTables }