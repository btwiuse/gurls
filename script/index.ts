import metaJson from "./meta.json";
import codeB64 from "./opt.wasm.base64.json";
import metaB64 from "./meta.wasm.base64.json";

let code = Uint8Array.from(atob(codeB64), (c) => c.charCodeAt(0));
let metaWasm = Uint8Array.from(atob(metaB64), (c) => c.charCodeAt(0));
let meta = metaJson;

export { code, meta, metaWasm };
