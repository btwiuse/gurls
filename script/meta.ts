import { getProgramMetadata } from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";
import {
  hexToU8a,
  u8aToHex,
} from "https://deno.land/x/polkadot@0.2.31/util/index.ts";
import {
  blake2AsHex,
  blake2AsU8a,
} from "https://deno.land/x/polkadot@0.2.31/util-crypto/index.ts";

export function metaTxtU8a(): Uint8Array {
  return hexToU8a(Deno.readTextFileSync("meta.txt"));
}

export function metaHashU8a(): Uint8Array {
  return new Uint8Array(JSON.parse(Deno.readTextFileSync(".metahash")));
}

export const metaHex = u8aToHex(metaTxtU8a());
export const meta = getProgramMetadata(metaHex);

// console.log(JSON.stringify(meta, "", "  "));
