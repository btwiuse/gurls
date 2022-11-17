import {
  getWasmMetadata,
  Metadata,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";
// import metaJson from "./meta.json" assert { type: "json" };
import codeB64 from "./opt.wasm.base64.json" assert { type: "json" };
import metaB64 from "./meta.wasm.base64.json" assert { type: "json" };

let code = Uint8Array.from(atob(codeB64), (c) => c.charCodeAt(0));
let metaWasm = Uint8Array.from(atob(metaB64), (c) => c.charCodeAt(0));
// let meta = meta as Metadata;
let meta = await getWasmMetadata(metaWasm);

export { code, meta, metaWasm };
