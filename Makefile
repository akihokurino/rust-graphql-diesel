MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

run-local:
	docker-compose up

build:
	export SCCACHE_DIR=.cache/sccache
	export RUSTC_WRAPPER=${HOME}/.cargo/bin/sccache
	cargo build