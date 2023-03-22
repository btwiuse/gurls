#!/usr/bin/env -S deno run -A

const RPC_NODE = "http://127.0.0.1:9933";

async function request(method: string, params: any) {
  const response = await fetch(RPC_NODE, {
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

console.log(JSON.stringify(await request("author_rotateKeys", [])));
