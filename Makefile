build:
	cargo build --release

install: build
	cp ./target/release/penrose-main ~/.local/bin/penrose

check:
	cargo check
