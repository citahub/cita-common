# CITA-Web3

Rust implementation of JSON-RPC multi-transport client for [CITA].

[CITA]: https://github.com/citahub/cita

## Usage

First, add the dependencies to `Cargo.toml`:

```toml
[dependencies]
cita-web3 = { git = "https://github.com/citahub/cita-common" }
cita-web3 = { git = "https://github.com/citahub/cita-common", branch="develop"}
```

Then, add this crate to the source codes:

```rust
extern crate cita_web3;
```

## Examples

- [Query the Block Height](examples/query_height.rs)

  ```sh
  cargo run --example query_height http://IP:PORT
  ```
