MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

run-local:
	cargo run

build:
	cargo build