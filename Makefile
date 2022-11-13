USER_INSTALL_DIR := ~/.local/bin
DEST_DIR := $(USER_INSTALL_DIR)

.PHONY: output install check

output: test
	cargo build --release

test:
	cargo test --release

install: output
	cp ./target/release/pencil $(DEST_DIR)/pencil
	strip $(DEST_DIR)/pencil

check:
	cargo check --release
