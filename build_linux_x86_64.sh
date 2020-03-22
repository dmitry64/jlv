#!/bin/bash
Version=$1

echo "Building Linux-x86_64 Version: $Version"
mkdir -p build_output/$Version/linux-x86_64

rm -rf target
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/jlv build_output/$Version/linux-x86_64