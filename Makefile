.PHONY: clean build run format lint install remove

clean:
	cargo clean

build:
	cargo build

run:
	cargo run

format:
	cargo fmt

lint:
	cargo clippy --all-features -- -Dwarnings
	
install:
	cargo install --path .

uninstall:
	cargo remove --path .

precommit: format build lint
