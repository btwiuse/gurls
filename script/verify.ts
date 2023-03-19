import { HexString } from "https://deno.land/x/polkadot@0.2.31/util/types.ts";
import {
  hexToU8a,
  u8aToHex,
} from "https://deno.land/x/polkadot@0.2.31/util/index.ts";
import {
  blake2AsHex,
  blake2AsU8a,
} from "https://deno.land/x/polkadot@0.2.31/util-crypto/index.ts";
import { metaHashU8a, metaTxtU8a } from "./meta.ts";

export function metaTxtHash(): HexString {
  return blake2AsHex(metaTxtU8a(), 256);
}

export function metaHash(): HexString {
  return u8aToHex(metaHashU8a());
}

export function metaVerify(): boolean {
  if (metaTxtHash() !== metaHash()) {
    throw Error(
      `blake2(meta.txt) does not match .metahash: (${metaTxtHash()} != ${metaHash()})`,
    );
  }
  return true;
}
