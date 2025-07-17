# Rust_Analysis_Code

This repository contains all code examples from the bachelor thesis *"Analysis of the Rust Programming Language"*.

## Structure

The folder structure mirrors the organization of the thesis. Each directory corresponds to a specific chapter or section discussed in the thesis, containing the relevant Rust source files and any associated C code used for FFI examples.

## Running the Code

All binaries can be executed using `cargo run --bin <binary-name>`.  
The `<binary-name>` corresponds to the entry listed under the `[ [bin] ]` section in the `Cargo.toml` file.

The procedural macros example must be executed using `cargo test --bin <binary-name>`. 


For example:

```bash
cargo run --bin extern.rs
