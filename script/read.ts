#!/usr/bin/env -S deno run -A
import {
  GearApi,
  getProgramMetadata,
} from "https://gear-js.deno.dev/api/index.ts";
import { config } from "https://deno.land/x/dotenv/mod.ts";
// import deploy from "../dist/deploy.json" assert { type: "json" };
import { metaHex } from "../dist/mod.ts";

const deploy = {
  programId: '0x78d61422cebfdc62cdad97e338f4660b0368100ed2412db19dc42fc00964f826',
};

const meta = getProgramMetadata(metaHex);

let { RPC_NODE } = config();

async function initGearApi() {
  return await GearApi.create({
    providerAddress: RPC_NODE,
  });
}

async function main() {
  console.log("api is initializing. Please hold on...");

  const api = await initGearApi();

  let query = { "Code": "wtf" };

  console.log({ programId: deploy.programId, meta, query });

  const result = await api.programState.read(
    { programId: deploy.programId },
    meta,
  );

  console.log("result:", result.toHuman());
}

await main();
Deno.exit(0);
