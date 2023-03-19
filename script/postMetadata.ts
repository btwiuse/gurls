import {
  decodeAddress,
  GearApi,
  GearKeyring,
} from "https://github.com/btwiuse/gear-js/raw/deno/api/index.ts";

export async function postMetadata(
  params: {
    genesis: string;
    metaHex: string;
    programId: string;
    name: string;
  },
) {
  let {
    genesis,
    metaHex,
    programId,
    name,
  } = params;

  let body = {
    "id": Math.floor(Math.random() * 100),
    "jsonrpc": "2.0",
    "method": "program.meta.add",
    "params": params,
  };

  // https requires passing `--unsafely-ignore-certificate-errors` to deno
  // so revert to http as a workaround
  let resp = await fetch("http://idea.gear-tech.io/api", {
    "headers": {
      "Accept": "application/json",
      "content-type": "application/json;charset=utf-8",
    },
    body: JSON.stringify(body),
    "method": "POST",
  });

  return await resp.json();
}
