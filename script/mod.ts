import {
  getProgramMetadata,
  Metadata,
} from "https://raw.githubusercontent.com/btwiuse/gear-js/deno/api/index.ts";
import codeB64 from "./opt.wasm.base64.json" assert { type: "json" };
import metaB64 from "./meta.wasm.base64.json" assert { type: "json" };
import metaHex from "./meta.txt.json" assert { type: "json" };

let code = Uint8Array.from(atob(codeB64), (c) => c.charCodeAt(0));
let metaWasm = Uint8Array.from(atob(metaB64), (c) => c.charCodeAt(0));
let meta = getProgramMetadata(metaHex);

export { code, meta, metaHex, metaWasm };
