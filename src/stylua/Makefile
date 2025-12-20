.PHONY: build clean test lint

PLUGIN := stylua
OUT_DIR ?= $(abspath ../../build)
ROOT := $(abspath ../..)
TARGET := wasm32-unknown-unknown

build:
	mkdir -p $(OUT_DIR)
	cargo build -p dprint-plugin-stylua --target $(TARGET) --release
	cp $(ROOT)/target/$(TARGET)/release/dprint_plugin_stylua.wasm $(OUT_DIR)/$(PLUGIN).wasm

test:
	cargo test -p dprint-plugin-stylua -- --nocapture

lint:
	cargo clippy -p dprint-plugin-stylua --all-targets -- -D warnings

clean:
	rm -f $(OUT_DIR)/$(PLUGIN).wasm
