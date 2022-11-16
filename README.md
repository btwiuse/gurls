# Gear URL Shortener (GURLS)

A URL shortener built on top of Deno Deploy and Gear, inspired by [goo.gl](https://goo.gl), [git.io](https://git.io), [yaus](https://github.com/denoland/deploy_examples/tree/main/yaus)

- Backend API / SSR server: [https://btwiuse/deploy_examples/blob/btwiuse/urls/mod.tsx](https://github.com/btwiuse/deploy_examples/blob/btwiuse/gurls/mod.tsx)
- Frontend script / browser logic: [./gear.ts](./lib.rs)
- Contract: [./lib.rs](./lib.rs)

## Live Demo

Visit one of the links for a live version

- https://gurls.deno.dev
- https://gurls.up.railway.app
- http://127.0.0.1:3000
  - `PORT=3000 deno run --allow-net --allow-env https://github.com/btwiuse/deploy_examples/raw/btwiuse/gurls/mod.tsx`

## Objectives

In this workshop, you will learn how to

- Implement basic contract logic
- Compile and deploy your contract
- Use `@gear-js/api`
  - to interact with published contract
    - read ( get program state )
    - write ( sending message )
  - from both frontend and backend
    - browser
    - nodejs
    - deno

## Directory Layout
```
.
├── Makefile               // Make targets: build, deploy, publish, clean, ...
├── deploy.ts              // Contract deploy utility
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

