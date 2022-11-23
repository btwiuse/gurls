PKG=gboard

all: build fmt

init:
	cargo install cargo-expand
	yarn

expand:
	cargo expand > dist/lib.expanded.rs

deploy: build
	deno run -A --unsafely-ignore-certificate-errors script/deploy.ts

build:
	mkdir -p dist
	cargo build --release
	cp -v target/wasm32-unknown-unknown/release/$(PKG).opt.wasm dist/opt.wasm
	cp -v target/wasm32-unknown-unknown/release/$(PKG).meta.wasm dist/meta.wasm
	cat dist/meta.wasm | base64 -w0 | jq -R . > dist/meta.wasm.base64.json
	cat dist/opt.wasm | base64 -w0 | jq -R . > dist/opt.wasm.base64.json
	deno run -A script/meta.ts > dist/meta.json
	cp script/mod.ts dist/mod.ts
	cp script/index.ts dist/index.ts

repl:
	deno repl --eval-file=script/repl.ts

fmt:
	deno fmt --ignore=node_modules,target,dist
	cargo fmt

publish: build
	yarn publish --access public

clean:
	rm -rf dist target

start:
	deno run -A https://github.com/btwiuse/deploy_examples/raw/btwiuse/gurls/mod.tsx
