import {
  GearApi,
  GearKeyring,
  getWasmMetadata,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/src/index.ts";
import { decodeAddress } from "https://github.com/btwiuse/gear-js/raw/deno/api/src/utils/index.ts";
import { waitForInit } from "./waitForInit.ts";
import { postMetadata } from "./postMetadata.ts";

async function initGearApi() {
  return await GearApi.create({
    providerAddress: "wss://rpc-node.gear-tech.io",
  });
}

console.log("api is initializing. Please hold on...");

let api = await initGearApi();

// deploy to gear
let codeIds = await api.code.all();
console.log("number of all code", codeIds.length);

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
console.log({
  meta,
  metaWasm,
  code,
});

let gas = await api.program.calculateGas.initUpload(
  aliceHex,
  code,
  "0x00",
  0,
  true,
  meta,
);
console.log(`GasLimit: ${gas}\n`);

let initPayload = "0x00";

let codeId =
  "0xcbabf7da317393244549cf01e3a03fe250e079e9bc84a2f21fad42adf17435fe";

let { programId, salt, extrinsic } = api.program.create({
  codeId,
  initPayload,
  gasLimit: gas.min_limit,
}, meta);

/*
const { programId, codeId, salt, extrinsic } = api.program.upload({
  code,
  initPayload,
  gasLimit: gas.min_limit,
}, meta);
*/

console.log(`ProgramID: ${programId}\n`);
console.log(`CodeId: ${codeId}\n`);
console.log(`salt: ${salt}\n`);
// console.log(`extrinsic: ${extrinsic}\n`);

// await extrinsic.signAndSend(alice);

// const { codeHash } = await api.code.upload(code);

// console.log(`codeHash: ${codeHash}\n`);

await new Promise((resolve, reject) => {
  api.program.signAndSend(alice, ({ events, status }) => {
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

// await waitForInit(api, programId);

console.log("Program is initialized");

// const id = '0x1bf2eac9232d461e17194b1f334b10419c319557f8ebd6cf9fee43da0ca3cb94';
const id = programId;

for (let i = 1; i < 3; i++) {
  console.log(await api.program.exists(id));
  await new Promise((resolve) => setTimeout(resolve, 1000));
}

/*
let metaWasm = Deno.readFileSync(
  "./target/wasm32-unknown-unknown/release/gurls.meta.wasm",
);

let programId = Deno.env.get("PROG") ??
  "0xbdaa64b7aabf93243abc4eed0c9058c3e8cc03826cf936756538de9920c4298c";

let alice = await GearKeyring.fromSuri("//Alice");

let api = await GearApi.create({
  providerAddress: "wss://rpc-node.gear-tech.io",
});
*/

await postMetadata(api, alice, metaWasm, programId);

let CODE = new Date().getTime().toString();

console.log("CODE:", CODE);

let payload = {
  AddUrl: {
    code: CODE,
    url: "https://google.com",
  },
};

console.log("payload:", CODE);

gas = await api.program.calculateGas.handle(
  aliceHex,
  programId,
  payload,
  0,
  true,
  meta,
);
console.log(`GasLimit: ${gas}\n`);

let tx = api.message.send({
  destination: programId,
  payload,
  gasLimit: gas.min_limit,
  value: 0,
}, meta);

await new Promise((resolve, reject) => {
  tx.signAndSend(alice, ({ events, status }) => {
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

const state = await api.programState.read(
  id,
  metaWasm,
  {
    "Code": CODE,
  },
);

console.log("state:", state.toHuman());

gas = await api.program.calculateGas.handle(
  aliceHex,
  programId,
  payload,
  0,
  true,
  meta,
);
console.log(`GasLimit: ${gas}\n`);
await new Promise((resolve, reject) => {
  tx.signAndSend(alice, ({ events, status }) => {
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
