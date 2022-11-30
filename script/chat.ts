#!/usr/bin/env -S deno run -A --unsafely-ignore-certificate-errors

import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

import {
  CreateType,
  decodeAddress,
  GearApi,
  GearKeyring,
  getWasmMetadata,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";
import { meta } from "https://unpkg.com/gearchat/dist/mod.ts";
import { config } from "https://deno.land/x/dotenv/mod.ts";
import deploy from "https://unpkg.com/gearchat/dist/deploy.json" assert {
  type: "json",
};

let { RPC_NODE } = config();

async function initGearApi() {
  return await GearApi.create({
    providerAddress: RPC_NODE,
  });
}

const api = await initGearApi();
const alice = await GearKeyring.fromSuri("//Alice");
const aliceHex = decodeAddress(alice.address);
const genesis = api.genesisHash.toHex();
const groupchat = deploy.programId;

function decodeEvent(payload: string) {
  try {
    return CreateType.create("Event", payload, meta);
  } catch (e) {
    console.log(payload, e);
  }
}

async function request(method: string, params: any) {
  const response = await fetch("https://idea.gear-tech.io/api", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      jsonrpc: "2.0",
      id: Math.floor(Math.random() * 100),
      method,
      params,
    }),
  });
  const responseData: any = await response.json();
  return responseData;
}

async function getMessages() {
  let resp = await request("message.all", {
    genesis,
    source: groupchat,
    limit: 100,
  });
  return resp.result.messages.filter((x) => x.exitCode === 0).reverse();
}

async function getDecodedMessages() {
  let msgs = await getMessages();
  // console.log(msgs);
  let decoded = msgs.map(
    ({ timestamp, id, payload, destination }) => {
      let { msg, by } = decodeEvent(payload).toHuman().AddedMsg;
      let localtime = new Date(timestamp).toLocaleString();
      return {
        localtime,
        by,
        msg,
      };
    },
  );
  return decoded;
}

function shortenAddr(addr: string) {
  return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
}

async function showDecodedMessages() {
  let decoded = await getDecodedMessages();
  console.clear();
  decoded.forEach(({ localtime, by, msg }) => {
    console.log(`[${localtime}] ${shortenAddr(by)}: ${msg}`);
  });
}

function subscribeTo(event: string, callback: () => void) {
  api.gearEvents.subscribeToGearEvent(event, callback);
}

const refresh = async () => {
  await new Promise((resolve) => setTimeout(resolve, 6000));
  await showDecodedMessages();
  console.log();
  console.log("PRESS ENTER TO SEND:");
};

async function addMsg(m: string) {
  const payload = { addMsg: m };
  // console.error(payload);

  const gas = await api.program.calculateGas.handle(
    aliceHex,
    groupchat,
    payload,
    0,
    true,
    meta,
  );
  // console.error(`GasLimit: ${gas}\n`);

  const msg = {
    destination: groupchat,
    payload,
    gasLimit: gas.min_limit,
    value: 0,
  };

  // console.log(msg);

  const tx = api.message.send(msg, meta);

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
}

async function main() {
  subscribeTo("MessageEnqueued", refresh);
  refresh();
  const stdin = await Deno.open("/dev/stdin");
  for await (const line of readline(stdin)) {
    if (line.length === 0) {
      continue;
    }
    let msg = new TextDecoder().decode(line);
    console.info(`SENDING...`);
    await addMsg(msg);
  }
  stdin.close();
}

await main();

// gear-meta decode "0x0024646464646464646464d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d" -m /home/aaron/gurls/dist/meta.wasm -t handle_output
