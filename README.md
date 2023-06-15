<p align="center">
  <a href="https://gitpod.io/#https://github.com/btwiuse/gurls" target="_blank">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="Gitpod">
  </a>
</p>

# Gear URL Shortener (GURLS)

A URL shortener built on top of Deno Deploy and Gear, inspired by
[goo.gl](https://goo.gl), [git.io](https://git.io),
[yaus](https://github.com/denoland/deploy_examples/tree/main/yaus)

- Backend API / SSR server:
  [https://btwiuse/deploy_examples/blob/btwiuse/urls/mod.tsx](https://github.com/btwiuse/deploy_examples/blob/btwiuse/gurls/mod.tsx)
- Frontend script / browser logic: [./gurls.ts](./gurls.ts)
- Contract: [./lib.rs](./lib.rs)

## Live Demo

- DEV_KEY:
  `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice`
- RPC_NODE: `wss://rpc-node.gear-tech.io`
- PROGRAM_ID:
  [`0x69d6bf23daf0aae8561a106245c80a5545232b5277788ab00cbc408171985857`](https://idea.gear-tech.io/programs/0x69d6bf23daf0aae8561a106245c80a5545232b5277788ab00cbc408171985857?node=wss://rpc-node.gear-tech.io)

Visit one of the links for a live version

- https://gurls.deno.dev
- https://gurls.up.railway.app
- https://gurls.vercel.app (WIP)
  - created by `npx create-gear-app`
  - TODO: port to Next.js
- http://127.0.0.1:3000
  - run
    `PORT=3000 deno run --allow-net --allow-env https://github.com/btwiuse/deploy_examples/raw/btwiuse/gurls/mod.tsx`

## Deploy

Before you proceed, make sure you have `git`, `yarn`, `rustup`, `deno`, `node`,
`jq` installed

```
$ git clone https://github.com/btwiuse/gurls && cd gurls
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
  name: "github.com/btwiuse/gurls",
  meta: '{"types":"0x180010146775726c7314636f64656318616374696f6e18416374696f6e0001041841646455726c080110636f...',
  title: "github.com/btwiuse/gurls",
  metaWasm: "AGFzbQEAAAABTw1gAX8AYAF/AX9gAAF/YAN/f38Bf2AAAGADf39/AGACf38Bf2ACf38AYAR/f39/AGAEf39/fwF/YAV/f39/fwBg...",
  signature: "0x2e778426cd1ff638eba215ee6f210828bb7703cd320a45dd80b04f7bb3c7681a6a7d99f45b90d2cce58f887a93551b9439...",
  programId: "0x648df8e155670708c61327f0d9e0aefdea21c257acd5638d6a9a3d31e127ef3d",
  genesis: "0xd144f24baf0b991be22ea8dc7dd4540d9d1e971e6bf17b1770b9fc9c88272484"
}
Program deloyed: https://idea.gear-tech.io/programs/0x648df8e155670708c61327f0d9e0aefdea21c257acd5638d6a9a3d31e127ef3d?node=wss://rpc-node.gear-tech.io
```

## Directory Layout

```
.
├── Makefile               // Make targets: build, deploy, publish, clean, ...
├── script/                // Utility scripts
├── dist/                  // Artifacts to be published at https://unpkg.com/browse/gurls/dist/

├── lib.rs                 // Gear contract
├── build.rs               // Custom build script
├── Cargo.toml             // Cargo crate config
├── rust-toolchain         // Rust toolchain config
├── target/                // Cargo build cache

├── gurls.ts               // Browser script
├── esbuild.config.mjs     // Esbuild bundler config
├── package.json           // Npm package config
├── node_modules/          // Node module cache

└── README.md              // This file
```

## Objectives

In this workshop, you will learn how to

- Implement basic contract logic
- Deploy your contract using [Gear Idea](https://idea.gear-tech.io)
- Use [`@gear-js/api`](https://www.npmjs.com/package/@gear-js/api)
  - to interact with published contract
    - read ( get program state )
    - write ( sending message )
  - from both frontend and backend
    - browser
    - nodejs
    - deno

## References

- https://wiki.gear-tech.io/docs/getting-started-in-5-minutes/
- https://github.com/gear-tech/gear/tree/master/examples
- https://en.wikipedia.org/wiki/Static_(keyword)
- https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics
- https://www.pwnthebox.net/rust/2020/11/01/deciphering-no-mangle.html
- https://docs.rust-embedded.org/book/interoperability/rust-with-c.html#building-a-c-api
- https://github.com/gear-tech/gear-js/blob/main/api/README.md
  - https://github.com/gear-tech/gear-js/blob/main/api/README.md#read-state-of-program
  - https://github.com/gear-tech/gear-js/blob/main/api/README.md#send-message
