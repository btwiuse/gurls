#!/usr/bin/env -S deno run -A
import {
  decodeAddress,
  GearApi,
  GearKeyring,
  getWasmMetadata,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";
import { config } from "https://deno.land/x/dotenv/mod.ts";
import deploy from "../dist/deploy.json" assert { type: "json" };
import { metaWasm } from "../dist/mod.ts";

let { RPC_NODE } = config();

async function initGearApi() {
  return await GearApi.create({
    providerAddress: RPC_NODE,
  });
}

async function main() {
  console.log("api is initializing. Please hold on...");

  const api = await initGearApi();

  let unsub = api.gearEvents.subscribeToGearEvent(
    "UserMessageSent",
    ({
      data: {
        message: { id, source, destination, payload, value, reply },
      },
    }) => {
      console.log(
        new Date(),
        "UserMessageSent",
        `
  messageId: ${id.toHex()}
  source: ${source.toHex()}
  payload: ${payload.toHuman()}
  `,
      );
    },
  );
  // Unsubscribe
  // unsub();

  /*
  unsub = api.gearEvents.subscribeToGearEvent(
    "MessageEnqueued",
    ({ data: { id, source, destination, entry } }) => {
      console.log(
        new Date(),
        "MessageEnqueued",
        {
          messageId: id.toHex(),
          programId: destination.toHex(),
          userId: source.toHex(),
          entry: entry.isInit
            ? entry.asInit
            : entry.isHandle
            ? entry.asHandle
            : entry.asReply,
        },
      );
    },
  );
  */

  unsub = api.gearEvents.subscribeToGearEvent(
    "MessagesDispatched",
    async ({ index, data }) => {
      console.log(
        new Date(),
        "MessagesDispatched",
      );

      if (data.stateChanges.toJSON().includes(deploy.programId)) {
        let query = { "Status": null };

        const result = await api.programState.read(
          deploy.programId,
          metaWasm,
          query,
        );

        console.log("program state changed:", result.toHuman());
      }
    },
  );
}

await main();
