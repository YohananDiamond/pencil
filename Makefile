.PHONY: output install check

output:
	cargo build --release

install: output
	cp ./target/release/penrose-main ~/.local/bin/penrose

check:
	cargo check --release
