#!/bin/bash
Version=$1

echo "Building Windows-x86_64 Version: $Version"
mkdir -p build_output/$Version/windows-x86_64

rm -rf target
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/jlv.exe build_output/$Version/windows-x86_64