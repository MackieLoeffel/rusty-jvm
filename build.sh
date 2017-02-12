#!/bin/sh

javac -target 1.2 -source 1.2 assets/*.java

rustup run nightly cargo clippy && \
RUST_TEST_THREADS=1 cargo test -- --nocapture && cargo run
# cargo run
