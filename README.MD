# Simple OS
This project is intended to build a bare-metal application while I'm learning Rust.

You can have a reference for this project here: https://os.phil-opp.com/

## Considerations

- Uses the Rust nightly compiler, which means it might occasionally break due to nightly changes.
- Targets the x86-64 CPU architecture.
- Builds the Rust core library for a custom target (see configuration in `.cargo/config.toml`).

## How to run it

1. [Install nightly Rust compiler](https://os.phil-opp.com/minimal-rust-kernel/#installing-rust-nightly)
2. Build the project for the custom target/architecture with `cargo build --target x86_64-simple-os.json`
3. TBD...