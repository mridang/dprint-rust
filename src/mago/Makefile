.PHONY: build clean test lint

PLUGIN := mago
OUT_DIR ?= $(abspath ../../build)
ROOT := $(abspath ../..)
TARGET := wasm32-unknown-unknown

build:
	mkdir -p $(OUT_DIR)
	cargo build -p dprint-plugin-mago --target $(TARGET) --release
	cp $(ROOT)/target/$(TARGET)/release/dprint_plugin_mago.wasm $(OUT_DIR)/$(PLUGIN).wasm

test:
	cargo test -p dprint-plugin-mago -- --nocapture

lint:
	cargo clippy -p dprint-plugin-mago --all-targets -- -D warnings

clean:
	rm -f $(OUT_DIR)/$(PLUGIN).wasm
