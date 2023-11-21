import { u8aToHex } from "@polkadot/util";
import { ProgramMetadata } from "@gear-js/api";

let metaResp = await fetch(
  "https://no-cors.deno.dev/https://github.com/btwiuse/gurls/releases/download/v0.0.1/gurls.meta.txt"
);

let metaText = await metaResp.text();

// console.log(`0x${metaText}`);

export const meta = ProgramMetadata.from(`0x${metaText}`);

/*
let wasmResp = await fetch(
  "https://no-cors.deno.dev/https://github.com/btwiuse/gurls/releases/download/v0.0.1/gurls.opt.wasm",
);

let wasmBuff = await wasmResp.arrayBuffer();

export const wasm = new Uint8Array(wasmBuff);
*/

// console.log(u8aToHex(wasm));

// export { meta, wasm };
