# num_cpu

## ⚠️ Warning ⚠️: This crate was backdoored for educational purpose. https://kerkour.com/rust-crate-backdoor

[![crates.io](https://img.shields.io/crates/v/num_cpu.svg)](https://crates.io/crates/num_cpu)

- [Documentation](https://docs.rs/num_cpu)
- [CHANGELOG](CHANGELOG.md)

Count the number of CPUs on the current machine.

## Usage

Add to Cargo.toml:

```toml
[dependencies]
num_cpu = "1.0"
```

In your `main.rs` or `lib.rs`:

```rust
// count logical cores this process could try to use
let num = num_cpu::get();
```
