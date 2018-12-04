# CITA-Web3

Rust implementation of JSON-RPC multi-transport client for [CITA].

[CITA]: https://github.com/cryptape/cita

## Usage

First, add the dependencies to `Cargo.toml`:

```toml
[dependencies]
cita-web3 = { git = "https://github.com/cryptape/cita-common" }

[features]
default = ["secp256k1", "sha3hash"]
secp256k1 = ["cita-web3/secp256k1"]
ed25519 = ["cita-web3/ed25519"]
sm2 = ["cita-web3/sm2"]
sha3hash = ["cita-web3/sha3hash"]
blake2bhash = ["cita-web3/blake2bhash"]
sm3hash = ["cita-web3/sm3hash"]
```

Then, add this crate to the source codes:

```rust
extern crate cita_web3;
```

## Examples

- [Query the Block Height](examples/query_height.rs)

  ```sh
  cargo run --example query_height --features "secp256k1 sha3hash" http://IP:PORT
  ```
