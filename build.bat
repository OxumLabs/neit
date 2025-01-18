@echo off

echo Making for Linux GNU
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-unknown-linux-gnu

echo Making for Windows
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-pc-windows-gnu

echo Making for Linux Musl
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-unknown-linux-musl

echo removing old bin folder
if exist bin rmdir /s /q bin
echo creating new bin folder
mkdir bin

echo copying linux gnu version to bin
copy target\x86_64-unknown-linux-gnu\release\neit bin\neit-lingnu
echo copying windows version to bin
copy target\x86_64-pc-windows-gnu\release\neit.exe bin\neit-win.exe
echo copying linux musl version to bin
copy target\x86_64-unknown-linux-musl\release\neit bin\neit-musl

echo built successfully