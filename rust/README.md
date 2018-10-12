# Project euler in Rust

## Run example

    cargo run --release --bin p005

## To run faster with:

1. Enable processor specific intrinsics

        $ env RUSTFLAGS="-C target-cpu=native" cargo run --release --bin p005

2. Enable link-time optimizations

        [profile.release]
        lto = true

3. Possible gains (no significant differences measured)

        [profile.release]
        lto = true
        panic = "abort"
        codegen-units=1