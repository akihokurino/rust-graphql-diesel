# rust-graphql-sample

## 構成
- rust 1.53.0
- sccache
- diesel
- juniper
https://github.com/graphql-rust/juniper

## diesel
```
cargo install diesel_cli --no-default-features --features mysql
diesel setup
diesel migration generate create_users
diesel migration run
```
