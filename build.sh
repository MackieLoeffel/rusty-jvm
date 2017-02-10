#!/bin/sh

javac -target 1.2 -source 1.2 assets/*.java

cargo test -- --nocapture # && cargo run
# cargo run
