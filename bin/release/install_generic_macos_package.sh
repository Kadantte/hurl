#!/bin/bash
set -Eeuo pipefail

echo "----- install generic macos package -----"

# install
generic_macos_package=$(ls target/upload/hurl-*-x86_64-macos.tar.gz)

install_dir="/tmp/hurl-generic-macos"
mkdir -p "${install_dir}"
tar xvf "${generic_macos_package}" -C "${install_dir}" --strip-components=1

# Return PATH var to parent shell
echo "Run this if you want to use fresh builded hurl package:"
echo "  export PATH=${install_dir}:$PATH"

