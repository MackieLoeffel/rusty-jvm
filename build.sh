#!/bin/sh

make
rustup run nightly cargo clippy && \
RUST_TEST_THREADS=1 cargo test -- --nocapture && make test && cargo run -- Jump
# cargo run
