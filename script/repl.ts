import { GearApi, GearKeyring } from "https://github.com/btwiuse/gear-js/raw/deno/api/src/index.ts";

async function initGearApi() {
  return await GearApi.create({ providerAddress: "wss://rpc-node.gear-tech.io" });
}

console.log("api is initializing. Please hold on...");

const api = await initGearApi();
