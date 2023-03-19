import {
  ProgramDoesNotExistError,
  ProgramExitedError,
  ProgramTerminatedError,
  SubmitProgramError,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/src/errors/index.ts";
import { ProgramMap } from "https://github.com/btwiuse/gear-js/raw/deno/api/src/types/interfaces/index.ts";
import { Option } from "https://deno.land/x/polkadot@0.2.32/types/index.ts";
import { HexString } from "https://deno.land/x/polkadot@0.2.32/util/types.ts";
import { parse } from "https://deno.land/std/encoding/toml.ts";
import {
  decodeAddress,
  GearApi,
  GearKeyring,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";
// import { waitForInit } from "./waitForInit.ts";
import { postMetadata } from "./postMetadata.ts";
import { meta, metaHex } from "./meta.ts";
import { config } from "https://deno.land/x/dotenv/mod.ts";
import { metaVerify } from "./verify.ts";
import { code } from "./code.ts";

async function cargoBuild() {
  let cmd = ["cargo", "build", "--release"];
  let p = Deno.run({ cmd });
  let { code } = await p.status();
  if (code !== 0) {
    throw Error(`Command '${cmd.join(" ")}' exited with code ${code}`);
  }
}

function packageName(): string {
  let cargoToml = Deno.readTextFileSync("Cargo.toml");
  const parsedToml = parse(cargoToml) as { [key: string]: any };
  const packageName = parsedToml["package"]["name"];
  return packageName;
}

async function initGearApi(RPC_NODE: string) {
  return await GearApi.create({
    providerAddress: RPC_NODE,
  });
}

async function uploadProgram(): string {
  let program = {
    code: await code(),
    gasLimit: 1000000000,
    value: 0,
    // initPayload: "0x00",
  };

  let { codeId } = await api.program.upload(
    program,
    // meta,
  );

  if (!await api.code.exists(codeId)) {
    // console.log("CodeID not found, uploading...");
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
  } else {
    // console.log("CodeID already exists, skipping upload...");
  }
  return codeId;
}

async function deployProgram(codeId: string) {
  let aliceHex = decodeAddress(alice.address);
  // console.log("decodedAddress:", aliceHex);

  let gas = await api.program.calculateGas.initCreate(
    aliceHex,
    codeId,
    INIT_PAYLOAD,
    0,
    true,
    await meta(),
  );
  // console.log(`GasLimit: ${gas}\n`);

  let { programId, extrinsic } = api.program.create({
    codeId,
    initPayload: INIT_PAYLOAD,
    gasLimit: gas.min_limit,
  }, await meta());

  // console.log({ codeId, programId });

  let out = await new Promise((resolve, reject) => {
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

  await checkProgram(programId);
  // console.log(out);
  return programId;
}

async function makePayload(programId: string) {
  for (let i = 0; i < 10; i++) {
    await new Promise((resolve) => setTimeout(resolve, 3000));
    // assert program exists
    if (await api.program.exists(programId)) {
      break;
    }
  }

  let genesis = api.genesisHash.toHex();
  let params = {
    genesis,
    metaHex: await metaHex(),
    programId,
    name: PROGRAM_NAME,
  };
  return params;
}

async function codeHash(id: HexString): Promise<HexString> {
  const programOption =
    (await api.query.gearProgram.programStorage(id)) as Option<ProgramMap>;

  if (programOption.isNone) throw new ProgramDoesNotExistError();

  const program = programOption.unwrap()[0];

  if (program.isTerminated) throw new ProgramTerminatedError(id);

  if (program.isExited) throw new ProgramExitedError(program.asExited.toHex());

  return program.asActive.codeHash.toHex();
}

async function init() {
  let dotenv = config();

  let PROGRAM_NAME = Deno.env.get("PROGRAM_NAME") || dotenv.PROGRAM_NAME ||
    packageName() || "unknown";
  let RPC_NODE = Deno.env.get("RPC_NODE") || dotenv.RPC_NODE ||
    `wss://rpc-node.gear-tech.io`;
  let DEV_KEY = Deno.env.get("DEV_KEY") || dotenv.DEV_KEY || "//Alice";
  let INIT_PAYLOAD = parseJSON(
    Deno.env.get("INIT_PAYLOAD") || dotenv.INIT_PAYLOAD || "null",
  );

  let metadata = await meta();
  if (INIT_PAYLOAD === null && metadata.types.init.input !== null) {
    throw Error(
      "Your contract requires an init input, but INIT_PAYLOAD is null. Deployment might fail!",
    );
  }

  console.log("Package Name:", PROGRAM_NAME);

  console.log(`api (${RPC_NODE}) is initializing. Please hold on...`);
  let api = await initGearApi(RPC_NODE);

  let alice = await GearKeyring.fromSuri(DEV_KEY);
  let { data: { free } } = await api.query.system.account(alice.address);
  console.log(`Dev key: ${alice.address}; free balance:`, free.toHuman());

  return {
    PROGRAM_NAME,
    RPC_NODE,
    INIT_PAYLOAD,
    api,
    alice,
  };
}

let { PROGRAM_NAME, RPC_NODE, INIT_PAYLOAD, alice, api } = await init();

async function checkProgram(programId: HexString) {
  await codeHash(programId);
}

async function main() {
  await cargoBuild();

  console.info("Verifying metadata...");
  metaVerify();

  console.info("Uploading program...");
  let codeId = await uploadProgram();
  console.info({ codeId });

  console.info("Deploying program...");
  let programId = await deployProgram(codeId);
  console.info({ programId });

  console.info("Making payload...");
  let params = await makePayload(programId);
  console.info(params);

  console.info("Posting metadata...");
  let resp = await postMetadata(params);
  console.info(resp);

  console.info(
    "Program deloyed:",
    `https://idea.gear-tech.io/programs/${programId}?node=${RPC_NODE}`,
  );
  // Deno.writeTextFileSync( "./dist/deploy.json", JSON.stringify( { codeId, programId, RPC_NODE, }, null, "  ",),);
}

main().then(() => Deno.exit(0));

function parseJSON(str: string): any | null {
  if (str.length === 0) {
    return null;
  }
  try {
    return JSON.parse(str);
  } catch (error) {
    console.error("Failed to parse JSON:", error);
    return null;
  }
}
