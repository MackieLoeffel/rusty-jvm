#!/bin/sh

javac -target 1.2 -source 1.2 assets/*.java

RUST_TEST_THREADS=1 cargo test -- --nocapture && cargo run
# cargo run
