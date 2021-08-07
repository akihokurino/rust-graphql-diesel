MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

run-local:
	docker-compose up

build:
	SCCACHE_CACHE_SIZE=5G \
    RUSTC_WRAPPER=`which sccache` \
    cargo build

clean:
	cargo clean
	docker-compose down --rmi all