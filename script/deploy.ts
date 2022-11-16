import {
  GearApi,
  GearKeyring,
  getWasmMetadata,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/src/index.ts";
import { decodeAddress } from "https://github.com/btwiuse/gear-js/raw/deno/api/src/utils/index.ts";
import { waitForInit } from "./waitForInit.ts";
import { postMetadata } from "./postMetadata.ts";

let RPC_NODE = "wss://rpc-node.gear-tech.io";

async function initGearApi() {
  return await GearApi.create({
    providerAddress: RPC_NODE,
  });
}

console.log("api is initializing. Please hold on...");

let api = await initGearApi();

// alice
let alice = await GearKeyring.fromSuri("//Alice");
let aliceHex = decodeAddress(alice.address);

// get free balance
async function showFreeBalance(api: GearApi, address: string) {
  let { data: { free } } = await api.query.system.account(address);
  console.log(`${address}'s free balance:`, free.toHuman());
}

await showFreeBalance(api, alice.address);

console.log("decodedAddress:", aliceHex);

// read file from ./target/wasm32-unknown-unknown/release/gurls.meta.wasm
let code = Deno.readFileSync(
  "./target/wasm32-unknown-unknown/release/gurls.opt.wasm",
);
let metaWasm = Deno.readFileSync(
  "./target/wasm32-unknown-unknown/release/gurls.meta.wasm",
);

let meta = await getWasmMetadata(metaWasm);

console.log("Deploying program...");

let program = {
  code,
  gasLimit: 1000000000,
  value: 0,
  initPayload: "0x00",
};

let { codeId } = await api.program.upload(
  program,
  meta,
);

if (!await api.code.exists(codeId)) {
  console.log("CodeID not found, uploading...");
  await new Promise((resolve, reject) => {
    api.code.signAndSend(alice, ({ events, status }) => {
      // console.log(`STATUS: ${status.toString()}`);
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
} else {
  console.log("CodeID already exists, skipping upload...");
}

let gas = await api.program.calculateGas.initCreate(
  aliceHex,
  codeId,
  "0x00",
  0,
  true,
  meta,
);
// console.log(`GasLimit: ${gas}\n`);

let { programId, extrinsic } = api.program.create({
  codeId,
  initPayload: "0x00",
  gasLimit: gas.min_limit,
}, meta);

console.log({ codeId, programId });

await new Promise((resolve, reject) => {
  api.program.signAndSend(alice, ({ events, status }) => {
    // console.log(`STATUS: ${status.toString()}`);
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

// await waitForInit(api, programId);

console.log("Posting metadata...");

await postMetadata(api, alice, metaWasm, programId);

// assert program exists
if (!await api.program.exists(programId)) {
  throw new Error("Program not found");
}

console.log(
  "Program deloyed:",
  `https://idea.gear-tech.io/programs/${programId}?node=${RPC_NODE}`,
);

Deno.exit(0);
