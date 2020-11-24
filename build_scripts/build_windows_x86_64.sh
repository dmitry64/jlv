#!/bin/bash
if [ -z "$1" ]
then
      echo "Version not found!"
      echo "Usage: $0 VERSION"
      exit 1
fi

Version=$1
echo "Building Windows-x86_64 Version: $Version"
cd ..
mkdir -p build_output/"$Version"/windows-x86_64

rm -rf target
cargo build --release --target x86_64-pc-windows-gnu
if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

cp target/x86_64-pc-windows-gnu/release/jlv.exe build_output/"$Version"/windows-x86_64