.PHONY: output install check

output:
	cargo build --release

install: output
	cp ./target/release/pencil ~/.local/bin/pencil

check:
	cargo check --release
