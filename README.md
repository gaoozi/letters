# Letters

A blog system build with rust.

## Tech Stack

- rust
    - [axum](https://github.com/tokio-rs/axum): A ergonomic and modular web framework
    - [tracing](https://github.com/tokio-rs/tracing): Application level tracing for Rust
    - [sqlx](https://github.com/launchbadge/sqlx): an async, pure Rust† SQL crate featuring compile-time checked queries without a DSL.
    - [serde](https://github.com/serde-rs/serde): Serialization framework for Rust
    - [thiserror](https://github.com/dtolnay/thiserror): derive(Error) for struct and enum error types
    - [dotenvy](https://github.com/allan2/dotenvy): A well-maintained fork of the Rust dotenv crate
    - [argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2): Pure Rust implementation of the Argon2 password hashing function
    - [base64](https://github.com/marshallpierce/rust-base64): base64, in rust
    - [jsonwebtoken](https://github.com/Keats/jsonwebtoken): JWT lib in rust
    - [secrecy](https://github.com/iqlusioninc/crates/tree/main/secrecy): A simple secret-keeping library for Rust.

## Usage

### Migrations

1. Install toolchain

```
$ cargo install sqlx-cli
```

2. create .env file, like:

```shell
# .env

DATABASE_URL=mysql://mysql@localhost/db_name
```

3. create/drop database

```
sqlx database create    # create database
sqlx database drop      # drop database
```

3. create and run migrations

create migrate, will create a new file in `migrations/<timestamp>-<name>.(up/down).sql`:
```
sqlx migrate add -r <name>
```
then add your database scheme to this file

---
run migrations
```
sqlx migrate run
```

more information to read [sqlx-cli document](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
