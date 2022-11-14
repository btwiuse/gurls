all: build expand

build:
	cargo build --release

expand:
	cargo expand > lib.expanded.rs
	cargo fmt
