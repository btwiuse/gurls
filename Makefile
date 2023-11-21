PKG=gurls

all: build fmt

init:
	cargo install cargo-expand
	yarn

expand:
	cargo expand > dist/lib.expanded.rs

deploy:
	deno run -A https://gear.deno.dev/deploy.ts

build:
	cargo build --release

repl:
	deno repl -A --eval-file=script/repl.ts

fmt:
	yarn fmt
	cargo fmt
	deno fmt --ignore=node_modules,target,dist

publish: build
	yarn publish --access public

clean:
	rm -rf dist target

start:
	deno run -A https://github.com/btwiuse/deploy_examples/raw/btwiuse/gurls/mod.tsx
