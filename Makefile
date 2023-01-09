.PHONY: clean build run format lint install remove

MAKEFLAGS += --no-print-directory
IMAGE = ~/.config/term.png

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

demo:
	kitty @ launch --cwd=current --copy-env --location=vsplit sh -c "make demo-1; sleep 30"
	clear && make demo-0 && sleep 30

demo-0:
	punfetch -i $(IMAGE)
	@printf "%.s\n" $$(seq 1 5); printf '%.s─' $$(seq 1 $$(tput cols));
	punfetch
	@printf "%.s\n" $$(seq 1 5); printf '%.s─' $$(seq 1 $$(tput cols));
	punfetch --show-logo never
	@printf "%.s\n" $$(seq 1 10)

demo-1:
	onefetch -i $(IMAGE)
	@printf '%.s─' $$(seq 1 $$(tput cols)); echo
	onefetch
	@printf '%.s─' $$(seq 1 $$(tput cols)); echo
	onefetch --show-logo never
