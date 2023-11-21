import { meta } from "./gurls-v0.0.1.js";
import { PROGRAM_ID, RPC_NODE } from "./config.js";
import YAML from "yaml";

import {
  web3Accounts,
  web3Enable,
  web3FromAddress,
} from "@polkadot/extension-dapp";
import { decodeAddress, GearApi, GearKeyring } from "@gear-js/api";
import { customAlphabet } from "nanoid";

window.programId = PROGRAM_ID;

let program = document.getElementById("program");
program.innerHTML = PROGRAM_ID;
program.setAttribute(
  "href",
  `https://idea.gear-tech.io/programs/${PROGRAM_ID}?node=${RPC_NODE}`
);

const Nanoid = customAlphabet("abcdefghijklmnopqrstuvwxyz0123456789");

function randomCode(n) {
  return Nanoid(n);
}

let apiPromise = GearApi.create({
  providerAddress: RPC_NODE,
});

let shorten = document.getElementById("shorten");

let outputDiv = document.getElementById("outputDiv");
let outputSpan = document.getElementById("outputSpan");

initApi();

async function checkWeb3() {
  shorten.innerHTML = "Checking wallet ...";
  let extensions = await web3Enable("GURLS");
  return extensions.length > 0;
}

async function initApi() {
  shorten.innerHTML = "Initializing api ...";
  let api = await apiPromise;

  window.meta = meta;
  let alice = await GearKeyring.fromSuri("//Alice");
  window.alice = alice;
  window.aliceRaw = decodeAddress(alice.address);
  window.api = api;

  if (!(await checkWeb3())) {
    alert("Please install PolkadotJS extension");
    return;
  }

  shorten.innerHTML = "Loading account ...";

  let accounts = await web3Accounts();
  let found = false;
  for (let account of accounts) {
    let freeBalance = await api.query.system.account(account.address);
    if (!freeBalance.data.free.isZero()) {
      console.log(account.address, freeBalance.data.free.toHuman());
      window.account = account;
      let injector = await web3FromAddress(account.address);
      window.injector = injector;
      api.setSigner(injector.signer);
      found = true;
      break;
    }
  }

  if (!found) {
    alert("Please add account with funds to PolkadotJS extension");
    return;
  }

  shorten.innerHTML = "Shorten";
  shorten.disabled = false;
  // prevent default
  shorten.onclick = async (e) => {
    hideOutput();
    let code = randomCode(4);
    // e.preventDefault();
    let url = document.getElementById("url").value;
    if (url.trim().length === 0) {
      return;
    }
    // await api.tx.system.remarkWithEvent(url).signAndSend(window.account.address);
    console.log({ code, url });
    await addUrl(code, url);
    outputDiv.style.display = "block";
    // strip trailing slash from window.location.href
    outputSpan.innerHTML = `${window.location.href.replace(
      /\/$/,
      ""
    )}/#/${code}`;
  };
}

async function addUrl(code, url) {
  shorten.disabled = true;
  let payload = { addUrl: { code, url } };
  console.log("payload:", payload);
  let gas = await api.program.calculateGas.handle(
    decodeAddress(window.account.address),
    window.programId,
    payload,
    0,
    true,
    window.meta
  );
  console.log("decoded address:", decodeAddress(window.account.address));
  console.log(`gas: ${gas}`);
  await new Promise((resolve, reject) => {
    api.message
      .send(
        {
          destination: window.programId,
          payload,
          gasLimit: gas.min_limit,
          value: 0,
        },
        window.meta
      )
      .signAndSend(window.account.address, ({ events, status }) => {
        console.log(`STATUS: ${status.toString()}`);
        if (status.isInBlock) {
          resolve(status.asInBlock);
        }
        if (status.isFinalized) {
          resolve(status.asFinalized);
        }
        events.forEach(({ event }) => {
          if (event.method === "ExtrinsicFailed") {
            console.log("ExtrinsicFailed");
            let error = api.getExtrinsicFailedError(event).docs.join("\n");
            console.log(error);
            alert(error);
          }
        });
      });
  });
  shorten.disabled = false;
}
