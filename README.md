# Gear Pixel Board

A Pixel Board on top of Gear, inspired by [matrix67.com](http://matrix67.com/)

- UI: [../src/pages/home/Home.tsx](https://github.com/btwiuse/gboard/blob/main/src/pages/home/Home.tsx)
- Contract: [./contract/lib.rs](./lib.rs)

## Live Demo

- DEV_KEY:
  `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice`
- RPC_NODE: `wss://rpc-node.gear-tech.io`
- PROGRAM_ID:
  [`0xe5046f27ada0b95657215894f64d6c5028f1c5cad59b77c82852169df83d1744`](https://idea.gear-tech.io/programs/0xe5046f27ada0b95657215894f64d6c5028f1c5cad59b77c82852169df83d1744?node=wss://n.up.railway.app/rpc/ws)

Visit one of the links for a live version

- https://gboard.deno.dev
- https://gboard.vercel.app
- http://127.0.0.1:3000
  - run `yarn && yarn start`

## Deploy Contract

Before you proceed, make sure you have `git`, `yarn`, `rustup`, `deno`, `node`,
`jq` installed

```
$ git clone https://github.com/btwiuse/gboard && cd gboard/contract
$ make init
...
$ make deploy
...
Deploying program...
CodeID already exists, skipping upload...
{
  codeId: "0x83239c38c37f77a1943c7ac4d44ab5f09fae588c50ca6c602b1c7a93763b9fc8",
  programId: "0x648df8e155670708c61327f0d9e0aefdea21c257acd5638d6a9a3d31e127ef3d"
}
Posting metadata...
{
  name: "github.com/btwiuse/gboard",
  meta: '{"types":"0x180010146775726c7314636f64656318616374696f6e18416374696f6e0001041841646455726c080110636f...',
  title: "github.com/btwiuse/gboard",
  metaWasm: "AGFzbQEAAAABTw1gAX8AYAF/AX9gAAF/YAN/f38Bf2AAAGADf39/AGACf38Bf2ACf38AYAR/f39/AGAEf39/fwF/YAV/f39/fwBg...",
  signature: "0x2e778426cd1ff638eba215ee6f210828bb7703cd320a45dd80b04f7bb3c7681a6a7d99f45b90d2cce58f887a93551b9439...",
  programId: "0x648df8e155670708c61327f0d9e0aefdea21c257acd5638d6a9a3d31e127ef3d",
  genesis: "0xd144f24baf0b991be22ea8dc7dd4540d9d1e971e6bf17b1770b9fc9c88272484"
}
Program deloyed: https://idea.gear-tech.io/programs/0x648df8e155670708c61327f0d9e0aefdea21c257acd5638d6a9a3d31e127ef3d?node=wss://rpc-node.gear-tech.io
```

## Objectives

In this workshop, you will learn how to

- Implement basic contract logic
- Deploy your contract using [Gear Idea](https://idea.gear-tech.io)
- Use [`@gear-js/api`](https://www.npmjs.com/package/@gear-js/api)
  - to interact with published contract
    - read ( get program state )
    - write ( send messages )
    - watch ( subscribe to state changes)

## References

- http://matrix67.com
- https://github.com/gear-tech/gear-js/blob/main/api/README.md
  - https://github.com/gear-tech/gear-js/blob/main/api/README.md#read-state-of-program
  - https://github.com/gear-tech/gear-js/blob/main/api/README.md#send-message
  - https://github.com/gear-tech/gear-js/blob/main/api/README.md#subscribe-to-specific-gear-events
- https://polkadot.js.org/docs/api/cookbook/tx/#how-can-i-batch-transactions
