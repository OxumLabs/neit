#!/bin/bash
echo "Making for Linux GNU"
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-gnu
echo "Making for Windows"
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu
echo "Making for Linux Musl"
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-unknown-linux-musl

echo "removing old bin folder"
rm -rf bin
echo "creating new bin folder"
mkdir bin

echo "copying linux gnu version to bin"
cp target/x86_64-unknown-linux-gnu/release/neit bin/neit-lingnu
echo "copying windows version to bin"
cp target/x86_64-pc-windows-gnu/release/neit.exe bin/neit-win.exe
echo "copying linux musl version to bin"
cp target/x86_64-unknown-linux-musl/release/neit bin/neit-musl

echo "Making them executable lol"
chmod +x bin/neit-lingnu
chmod +x bin/neit-musl

echo "built successfully"