# rust-graphql-sample
Rustの練習用プロジェクト

## sccache

```
cargo install sccache
```

## linux build

- https://github.com/messense/homebrew-macos-cross-toolchains
- https://github.com/messense/homebrew-macos-cross-toolchains/releases
- https://xn--kst.jp/blog/2021/05/16/m1mac-cross-compile/
- https://tech.bitbank.cc/lambda-custmon-runtime/
- https://github.com/chinedufn/cross-compile-rust-from-mac-to-linux

```
brew tap messense/macos-cross-toolchains
brew install x86_64-unknown-linux-gnu
brew install aarch64-unknown-linux-gnu

rustup target add x86_64-unknown-linux-musl
```

## diesel

```
cargo install diesel_cli --no-default-features --features mysql
diesel setup
diesel migration generate create_users
diesel migration run
```