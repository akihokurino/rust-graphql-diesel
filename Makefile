MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

run-local:
	docker-compose up

build_linux:
	sccache --start-server
	mkdir -p .cache/sccache
	SCCACHE_CACHE_SIZE=5G \
	SCCACHE_DIR=.cache/sccache \
	RUSTC_WRAPPER=`which sccache` \
	CC_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-gcc \
	CXX_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-g++ \
	AR_x86_64_unknown_linux_gnu=x86_64-unknown-linux-gnu-ar \
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc \
	cargo build --target x86_64-unknown-linux-musl
	cp -r ~/Library/Caches/Mozilla.sccache/* .cache/sccache
	sccache --stop-server

# MacでSCCACHE_DIRが機能していない
# rustc 1.53.0
# sccache 0.2.15
build_mac:
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