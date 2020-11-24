#!/bin/bash

if [ -z "$1" ]
then
      echo "Version not found!"
      echo "Usage: $0 VERSION"
      exit 1
fi

Version=$1
echo "Building Linux-x86_64 Version: $Version"
cd ..
mkdir -p build_output/"$Version"/linux-x86_64

rm -rf target
cargo build --release --target x86_64-unknown-linux-gnu
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

cp target/x86_64-unknown-linux-gnu/release/jlv build_output/"$Version"/linux-x86_64
