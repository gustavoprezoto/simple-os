# Simple OS
This project is intended build a bare-metal application while im learning Rust.

You can have a reference for this project here: https://os.phil-opp.com/

## Considerations

- This project uses nightly Rust compiler, so it mighty break sometimes
- The project target CPU architecture is x86-64
- The project builds the Rust core lib for our custom target (you can check the configs at `.cargo/config.toml`)

## How to run it

1. [Install nightly Rust compiler](https://os.phil-opp.com/minimal-rust-kernel/#installing-rust-nightly)
2. Build the project for the custom target/architecture with `cargo build --target x86_64-simple-os.json`
3. TBD...