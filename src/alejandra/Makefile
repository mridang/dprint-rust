.PHONY: build clean test lint

PLUGIN := alejandra
OUT_DIR ?= $(abspath ../../build)
ROOT := $(abspath ../..)
TARGET := wasm32-unknown-unknown

build:
	mkdir -p $(OUT_DIR)
	cargo build -p dprint-plugin-alejandra --target $(TARGET) --release
	cp $(ROOT)/target/$(TARGET)/release/dprint_plugin_alejandra.wasm $(OUT_DIR)/$(PLUGIN).wasm

test:
	cargo test -p dprint-plugin-alejandra -- --nocapture

lint:
	cargo clippy -p dprint-plugin-alejandra --all-targets -- -D warnings

clean:
	rm -f $(OUT_DIR)/$(PLUGIN).wasm
