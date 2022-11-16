# Gear URL Shortener (GURLS)

A URL shortener built on top of Deno Deploy and Gear, inspired by [goo.gl](https://goo.gl), [git.io](https://git.io), [yaus](https://github.com/denoland/deploy_examples/tree/main/yaus)

- Backend API / SSR server: [https://btwiuse/deploy_examples/blob/btwiuse/urls/mod.tsx](https://github.com/btwiuse/deploy_examples/blob/btwiuse/gurls/mod.tsx)
- Frontend script / browser logic: [./gear.ts](./gear.ts)
- Contract: [./lib.rs](./lib.rs)

## Live Demo

- DEV_KEY: `bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice`
- RPC_NODE: `wss://rpc-node.gear-tech.io`
- PROGRAM_ID: [`0x024d4e3cf6afae2f53f3d0e0bdd33a24e903463a51bbd7ca7d2be5cbf66be750`](https://idea.gear-tech.io/programs/0x024d4e3cf6afae2f53f3d0e0bdd33a24e903463a51bbd7ca7d2be5cbf66be750)

Visit one of the links for a live version

- https://gurls.deno.dev
- https://gurls.up.railway.app
- https://gurls.vercel.app (WIP)
  - created by `npx create-gear-app`
  - TODO: port to Next.js
- http://127.0.0.1:3000
  - run `PORT=3000 deno run --allow-net --allow-env https://github.com/btwiuse/deploy_examples/raw/btwiuse/gurls/mod.tsx`


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

- https://github.com/gear-tech/gear/tree/master/examples
- https://doc.rust-lang.org/reference/items/static-items.html#mutable-statics
- https://www.pwnthebox.net/rust/2020/11/01/deciphering-no-mangle.html
- https://docs.rust-embedded.org/book/interoperability/rust-with-c.html#building-a-c-api
- https://github.com/gear-tech/gear-js/blob/main/api/README.md
  - https://github.com/gear-tech/gear-js/blob/main/api/README.md#read-state-of-program
  - https://github.com/gear-tech/gear-js/blob/main/api/README.md#send-message
