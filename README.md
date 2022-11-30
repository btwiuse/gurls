# Gear Chat

A minimalistic IRC style terminal group chat built on top of Gear messages

## Live Demo

- DEV_KEY:
  `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice`
- PROGRAM_ID:
  [`0x7b742fc73552a132d3f97272658be0f030d8aade708aff32ad981d300aaf33ca`](https://idea.gear-tech.io/programs/0x7b742fc73552a132d3f97272658be0f030d8aade708aff32ad981d300aaf33ca?node=wss://rpc-node.gear-tech.io)

Install `deno` (https://deno.land/manual@v1.28.2/getting_started/installation), clone this repository, then join the chat by running

```
$ make chat
```

## Deploy

Before you proceed, make sure you have `git`, `yarn`, `rustup`, `deno`, `node`,
`jq` installed

```
$ git clone https://github.com/btwiuse/gearchat && cd gearchat
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
  name: "groupchat",
  meta: '{"types":"0x180010146775726c7314636f64656318616374696f6e18416374696f6e0001041841646455726c080110636f...',
  title: "groupchat",
  metaWasm: "AGFzbQEAAAABTw1gAX8AYAF/AX9gAAF/YAN/f38Bf2AAAGADf39/AGACf38Bf2ACf38AYAR/f39/AGAEf39/fwF/YAV/f39/fwBg...",
  signature: "0x2e778426cd1ff638eba215ee6f210828bb7703cd320a45dd80b04f7bb3c7681a6a7d99f45b90d2cce58f887a93551b9439...",
  programId: "0x648df8e155670708c61327f0d9e0aefdea21c257acd5638d6a9a3d31e127ef3d",
  genesis: "0xd144f24baf0b991be22ea8dc7dd4540d9d1e971e6bf17b1770b9fc9c88272484"
}
Program deloyed: https://idea.gear-tech.io/programs/0x648df8e155670708c61327f0d9e0aefdea21c257acd5638d6a9a3d31e127ef3d?node=wss://rpc-node.gear-tech.io
```

## Objectives

In this workshop, you will learn how to

- Retrieve historical messages using Gear Idea's API
- Decode message payload using metadata
- Get familiar with various kinds of ids: ActorId, MessageId, ProgramId, CodeId, etc.
- Learn the basic concepts of Actor Model

## References

- https://zh.m.wikipedia.org/zh-hans/%E6%BC%94%E5%91%98%E6%A8%A1%E5%9E%8B
- https://github.com/gear-tech/gear-js/tree/main/utils/meta-cli
- https://github.com/gear-tech/gear-js/blob/main/api/README.md
  - https://github.com/gear-tech/gear-js/tree/main/api#encode--decode-payloads
