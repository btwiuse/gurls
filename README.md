# Gear URL Shortener (GURLS)

A URL shortener built on top of Deno Deploy and Gear.

## Directory Layout
```
.
├── Makefile               // Make targets: build, deploy, publish, clean, ...
├── deploy.ts              // Contract deploy utility

├── lib.rs                 // Gear contract
├── build.rs               // Custom build script
├── Cargo.toml             // Cargo crate config
├── rust-toolchain         // Rust toolchain config

├── gurls.ts               // Browser script
├── esbuild.config.mjs     // Esbuild bundler config
├── package.json           // Npm package config

└── README.md              // This file
```

