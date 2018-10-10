# Project euler in Rust

## Run example

    cargo run --release --bin p005

## Bugs

1. Programs don't terminate when (and maxes out cpu):

        $ env RUSTFLAGS="-C target-cpu=native" cargo run --release --bin p005

    and

        [profile.release]
        lto = true

    are both enabled. Any other combination works.