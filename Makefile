.PHONY: default build build-mago build-stylua build-rumdl build-alejandra lint test clean

ROOT_BUILD_DIR := $(abspath build)

default: build

build: build-mago build-stylua build-rumdl build-alejandra

build-mago:
	$(MAKE) -C src/mago build OUT_DIR=$(ROOT_BUILD_DIR)

build-stylua:
	$(MAKE) -C src/stylua build OUT_DIR=$(ROOT_BUILD_DIR)

build-rumdl:
	$(MAKE) -C src/rumdl build OUT_DIR=$(ROOT_BUILD_DIR)

build-alejandra:
	$(MAKE) -C src/alejandra build OUT_DIR=$(ROOT_BUILD_DIR)

lint:
	cargo fmt --all
	$(MAKE) -C src/mago lint
	$(MAKE) -C src/stylua lint
	$(MAKE) -C src/rumdl lint
	$(MAKE) -C src/alejandra lint

test:
	$(MAKE) -C src/mago test
	$(MAKE) -C src/stylua test
	$(MAKE) -C src/rumdl test
	$(MAKE) -C src/alejandra test

clean:
	cargo clean
	rm -rf $(ROOT_BUILD_DIR)
