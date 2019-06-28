#!/bin/sh
BIN=./target/debug/hxdmp
DATA_DIR=./data
HEX=hex.tmp
HX=hx.tmp

# build the binary
cargo build

# iterate over test files, comparing hexdumps
for file in "$DATA_DIR"/*; do
    printf "testing file %s\n" "$file"
    hexdump -C "$file" > "$HEX"
    "$BIN" "$file" > "$HX"
    diff -s "$HEX" "$HX"
done

# clean up
rm "$HEX" "$HX"
