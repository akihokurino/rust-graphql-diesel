FROM rust:1.53.0 as local-dev

RUN cargo install sccache

ENV SCCACHE_CACHE_SIZE="5G"
ENV SCCACHE_DIR=/app/.cache/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"