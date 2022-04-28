#!/usr/bin/env bash
# building
cbindgen src/lib.rs -l c > rustywallet.h
cargo lipo --release

# moving files to the ios project
proj=ios
inc=../${proj}/include
libs=../${proj}/libs

rm -rf ${inc} ${libs}

mkdir ${inc}
mkdir ${libs}

cp rustywallet.h ${inc}
cp target/universal/release/librustywallet.a ${libs}
