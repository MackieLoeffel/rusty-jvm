#!/bin/sh

for i in assets/*.java; do
    javac -target 1.2 -source 1.2 "$i"
done

cargo test -- --nocapture
