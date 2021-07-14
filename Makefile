MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

run-local:
	docker-compose up

build_linux:
	cargo build --target x86_64-unknown-linux-musl

# MacでSCCACHE_DIRが機能していない
# rustc 1.53.0
# sccache 0.2.15
build:
	sccache --start-server
	mkdir -p .cache/sccache
	SCCACHE_CACHE_SIZE=5G \
    SCCACHE_DIR=.cache/sccache \
    RUSTC_WRAPPER=`which sccache` \
    cargo build
	cp -r ~/Library/Caches/Mozilla.sccache/* .cache/sccache
	sccache --stop-server

stop-sccache-server:
	sccache --stop-server

clean:
	cargo clean
	rm -rf .cache
	docker-compose down --rmi all