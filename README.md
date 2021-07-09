# rust-graphql-sample
Rustの練習用プロジェクト

## Cargo

```
cargo install sccache
```

## diesel

```
cargo install diesel_cli --no-default-features --features mysql
diesel setup
diesel migration generate create_users
diesel migration run
```