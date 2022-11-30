#!/usr/bin/env -S deno run -A
import {
  CreateType,
  decodeAddress,
  GearApi,
  GearKeyring,
  getWasmMetadata,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";
import { config } from "https://deno.land/x/dotenv/mod.ts";
import deploy from "../dist/deploy.json" assert { type: "json" };
import { meta, metaWasm } from "../dist/mod.ts";

let { RPC_NODE } = config();

async function initGearApi() {
  return await GearApi.create({
    providerAddress: RPC_NODE,
  });
}

async function main() {
  let result = CreateType.create(
    "Event",
    "0x0024646464646464646464d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
    meta,
  );
  console.log(JSON.stringify(result.toJSON()));
}

await main();
Deno.exit(0);
