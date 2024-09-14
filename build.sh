# Clean previous builds
rm -rf ./bin
mkdir -p bin/linux bin/windows

# Build static Linux binary with MUSL
cargo build --release --target x86_64-unknown-linux-musl
if [ $? -eq 0 ]; then
    echo "Linux binary built successfully."
else
    echo "Linux binary build failed."
    exit 1
fi

# Copy Linux binary to bin/linux
cp ./target/x86_64-unknown-linux-musl/release/neit bin/linux/neit

# Build static Windows binary with MinGW
cargo build --release --target x86_64-pc-windows-gnu
if [ $? -eq 0 ]; then
    echo "Windows binary built successfully."
else
    echo "Windows binary build failed."
    exit 1
fi

# Copy Windows binary to bin/windows
cp ./target/x86_64-pc-windows-gnu/release/neit.exe bin/windows/neit.exe
cp ./vc_redist.x64.exe ./bin/windows
echo "Binaries successfully copied to ./bin"


#cargo build --release --target x86_64-pc-windows-gnu -C target-feature=+crt-static foo.rs
