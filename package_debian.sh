#!/bin/bash
Version=$1

echo "Packaging .DEB Linux-x86_64 Version: $Version"
rm -rf build_output/$Version/deb_package
mkdir -p build_output/$Version/deb_package/jlv-$Version
cp build_output/$Version/linux-x86_64/jlv build_output/$Version/deb_package/jlv-$Version
cd build_output/$Version/deb_package/jlv-$Version
dh_make --createorig --packageclass=s
if [ $? -ne 0 ]; then
    echo "Build failed"
fi
touch debian/install
echo "jlv usr/bin" > debian/install
echo "Building package: $Version"
debuild -us -uc
if [ $? -ne 0 ]; then
    echo "Build failed"
fi
echo "Debian package created: $Version"