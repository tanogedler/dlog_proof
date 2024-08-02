
# ZK DLOG Proof

This is a simple library that implements the Non-interactive Schnorr ZK DLOG Proof scheme with a Fiat-Shamir transformation. 

We use the library `curv` to handle the elliptic curve operations. The library can be found [here](https://github.com/ZenGo-X/curv).

Also, we use `sha2` to hash the challenge in the Fiat-Shamir transformation. 


## Rust

This implementation uses the latest stable version of Rust (rustc 1.80.0).

To install Rust, follow the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Usage

To build the program, use the following command:

```bash
cargo build
```

## Tests

To run the tests, use the following command:

```bash
cargo test
```

## Run 

You can also run a simple execution of the code under the `main.rs` file using 

```bash
cargo run
```