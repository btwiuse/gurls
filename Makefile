all: build fmt

deploy:
	deno run -A --unsafely-ignore-certificate-errors script/deploy.ts

build:
	mkdir -p dist
	cargo expand > dist/lib.expanded.rs
	cargo build --release
	cp target/wasm32-unknown-unknown/release/gurls.*.wasm dist/
	cat dist/gurls.meta.wasm | base64 -w0 | jq -R . > dist/gurls.meta.wasm.base64.json
	yarn && node esbuild.config.mjs

fmt:
	deno fmt --ignore=node_modules,target,dist
	cargo fmt

publish: build
	yarn publish --access public

clean:
	rm -rf dist target

start:
	deno run -A https://github.com/btwiuse/deploy_examples/raw/btwiuse/gurls/mod.tsx
