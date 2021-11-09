# rust-graphql-diesel-sample

## 構成
- rust v1.53.0
- sccache
- diesel
- juniper （ https://github.com/graphql-rust/juniper )
- juniper-from-schema（ https://github.com/davidpdrsn/juniper-from-schema ）

## diesel
```
cargo install diesel_cli --no-default-features --features mysql
diesel setup
diesel migration generate create_users
diesel migration run
```
