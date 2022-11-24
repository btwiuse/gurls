#!/usr/bin/env -S deno run -A
import {
  decodeAddress,
  GearApi,
  GearKeyring,
  getWasmMetadata,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";
import { config } from "https://deno.land/x/dotenv/mod.ts";
import deploy from "../dist/deploy.json" assert { type: "json" };
import { meta } from "../dist/mod.ts";

let { RPC_NODE } = config();

async function initGearApi() {
  return await GearApi.create({
    providerAddress: RPC_NODE,
  });
}

console.log("api is initializing. Please hold on...");

const alice = await GearKeyring.fromSuri("//Alice");
const aliceHex = decodeAddress(alice.address);

const api = await initGearApi();

async function main(px: number) {
  // const payload = { Set: true };
  const payload = { SetPixel: [px, px % 2 === 1] };
  // const payload = { TogglePixel: [px, px%2===1] };
  console.log(payload);

  const gas = await api.program.calculateGas.handle(
    aliceHex,
    deploy.programId,
    payload,
    0,
    true,
    meta,
  );
  console.log(`GasLimit: ${gas}\n`);

  const msg = {
    destination: deploy.programId,
    payload,
    gasLimit: gas.min_limit,
    value: 0,
  };

  console.log(msg);

  const tx = api.message.send(msg, meta);

  return tx;
}

// let tx = await main(0);
let txs = api.tx.utility.batch([
  await main(1),
  await main(2),
  await main(3),
  await main(4),
  await main(5),
  await main(6),
  await main(7),
  await main(8),
  await main(9),
  await main(10),
  await main(11),
  await main(12),
  await main(13),
  await main(14),
]);

await new Promise((resolve, reject) => {
  txs.signAndSend(alice, ({ events, status }) => {
    console.log(`STATUS: ${status.toString()}`);
    if (status.isFinalized) {
      resolve(status.asFinalized);
    }
    events.forEach(({ event }) => {
      if (event.method === "ExtrinsicFailed") {
        reject(api.getExtrinsicFailedError(event).docs.join("\n"));
      }
    });
  });
});

Deno.exit(0);
