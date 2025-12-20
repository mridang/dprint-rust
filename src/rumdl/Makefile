.PHONY: build clean test lint

PLUGIN := rumdl
OUT_DIR ?= $(abspath ../../build)
ROOT := $(abspath ../..)
TARGET := wasm32-unknown-unknown

build:
	mkdir -p $(OUT_DIR)
	cargo build -p dprint-plugin-rumdl --target $(TARGET) --release
	cp $(ROOT)/target/$(TARGET)/release/dprint_plugin_rumdl.wasm $(OUT_DIR)/$(PLUGIN).wasm

test:
	cargo test -p dprint-plugin-rumdl -- --nocapture

lint:
	cargo clippy -p dprint-plugin-rumdl --all-targets -- -D warnings

clean:
	rm -f $(OUT_DIR)/$(PLUGIN).wasm
