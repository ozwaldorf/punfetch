.PHONY: clean build run format lint install remove init-docs save-docs patch-docs dev

MAKEFLAGS += --no-print-directory
IMAGE = ~/.config/term.png
PATCH = $(shell readlink -f docs/patch.diff)

init-docs:
	git submodule update --init
	cd docs/vercel && npm ci

patch-docs:
	cd docs/vercel && git apply $(PATCH)

save-docs:
	cd docs/vercel && git diff > $(PATCH)

dev:
	cd docs/vercel && npm run dev

clean:
	cargo clean

build:
	cargo build

install:
	cargo install --path .

uninstall:
	cargo remove --path .

run:
	cargo run

format:
	cargo fmt

lint:
	cargo fmt -- --check
	cargo clippy --all-targets --all-features -- -Dwarnings -Dclippy::all

test:
	cargo test --all --all-features

bench:
	cargo bench

ci: format lint test

# Requires kitty with remote control enabled
demo:
	kitty @ launch --cwd=current --copy-env --location=vsplit sh -c "make demo-1; sleep 30"
	clear && make demo-0 && sleep 30

# Run punfetch demo
demo-0:
	punfetch -i $(IMAGE)
	@printf '%.s─' $$(seq 1 $$(tput cols));
	punfetch
	@printf '%.s─' $$(seq 1 $$(tput cols));
	punfetch --show-logo never

# Run onefetch comparison demo
demo-1:
	onefetch -i $(IMAGE)
	@printf '%.s─' $$(seq 1 $$(tput cols)); echo
	onefetch
	@printf '%.s─' $$(seq 1 $$(tput cols)); echo
	onefetch --show-logo never
