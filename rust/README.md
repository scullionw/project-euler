# Project euler in Rust

## Run example

    cargo run --release --bin p005

## Benchmarks

    cpunative cargobench --bin p001

where:
    
    alias cpunative="RUSTFLAGS='-C target-cpu=native'"
    alias cargobench="cargo +nightly bench --features benchmode"



## Optimizations for speed:

1. Enable processor specific intrinsics

        $ env RUSTFLAGS="-C target-cpu=native" cargo run --release --bin p005

2. Enable link-time optimizations

        [profile.release]
        lto = true # or thin / fat

3. Possible gains (no significant differences measured)

        [profile.release]
        lto = true
        panic = "abort"
        codegen-units=1

## Optimizations for size

    $ strip ./target/release/p005
    $ upx --brute ./target/release/p005 (will use more memory during use)