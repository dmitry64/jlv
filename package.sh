#!/bin/bash

echo "Packaging..."

Version=$(cat Cargo.toml | grep "version" | grep -Po 'version = *\K"[^"]*"' | sed 's/"//g')
echo "Version: $Version"

rm -rf build_output
mkdir -p build_output/$Version

./build_linux_x86_64.sh $Version
./package_debian.sh $Version
./build_windows_x86_64.sh $Version
