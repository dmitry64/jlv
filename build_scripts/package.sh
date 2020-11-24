#!/bin/bash

echo "Packaging jlv..."

echo "Reading version from TOML file..."
Version=$(cat ../Cargo.toml | grep "version" | grep -Po 'version = *\K"[^"]*"' | sed 's/"//g')
echo "Version detected: $Version"

echo "Cleaning build folder"
rm -rf build_output
mkdir -p build_output/"$Version"

echo "Running build scripts..."

./build_linux_x86_64.sh "$Version"
if [ $? -ne 0 ]; then
    echo "Linux build failed!"
    exit 1
fi
./package_debian.sh "$Version"
if [ $? -ne 0 ]; then
    echo "Failed make debian package!"
    exit 1
fi
./build_windows_x86_64.sh "$Version"
if [ $? -ne 0 ]; then
    echo "Windows build failed!"
    exit 1
fi

echo "Packaging finished successfully!"