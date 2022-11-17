import {
  decodeAddress,
  GearApi,
  GearKeyring,
  getWasmMetadata,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/src/index.ts";
import metaWasmBase64 from "../dist/meta.wasm.base64.json" assert {
  type: "json",
};

let metaWasm = Uint8Array.from(atob(metaWasmBase64), (c) => c.charCodeAt(0));

const codeId =
  "0xcbabf7da317393244549cf01e3a03fe250e079e9bc84a2f21fad42adf17435fe";
const programId =
  "0x1bf2eac9232d461e17194b1f334b10419c319557f8ebd6cf9fee43da0ca3cb94";

let meta = await getWasmMetadata(metaWasm);

console.log(JSON.stringify(meta, "", "  "));
