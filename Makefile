all: build expand fmt

deploy:
	deno run -A --unsafely-ignore-certificate-errors deploy.ts

build:
	cargo build --release
	yarn && node esbuild.config.mjs
	mkdir -p dist
	cp target/wasm32-unknown-unknown/release/gurls.*.wasm dist/
	cat dist/gurls.meta.wasm | base64 -w0 | jq -R . > dist/gurls.meta.wasm.base64.json

expand:
	cargo expand > lib.expanded.rs

fmt:
	deno fmt --ignore=node_modules,target,dist
	cargo fmt

publish: build
	yarn publish --access public

clean:
	rm -rf dist target

start:
	deno run -rA index.ts
