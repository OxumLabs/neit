#!/usr/bin/env bash
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

# Include the NASM folder for Windows
echo "Copying NASM files..."
mkdir -p bin/windows/nasm
cp -r ./nasm/* bin/windows/

# Include Visual C++ Redistributable if required
cp ./vc_redist.x64.exe ./bin/windows

# Create the docs folder in bin/windows with README and LICENSE
echo "Creating docs folder with README and LICENSE..."
mkdir -p bin/windows/docs

# Generate README and LICENSE files dynamically
echo "This program includes NASM, the Netwide Assembler, for assembling code." > bin/windows/docs/README.txt
echo "Refer to the LICENSE file for more details." >> bin/windows/docs/README.txt

echo "NASM is licensed under the 2-clause BSD license." > bin/windows/docs/LICENSE.txt
echo "Redistribution and use in source and binary forms, with or without modification, are permitted under the following conditions:" >> bin/windows/docs/LICENSE.txt
echo "" >> bin/windows/docs/LICENSE.txt
echo "1. Redistributions of source code must retain the above copyright notice, this list of conditions, and the following disclaimer." >> bin/windows/docs/LICENSE.txt
echo "2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions, and the following disclaimer in the documentation and/or other materials provided with the distribution." >> bin/windows/docs/LICENSE.txt
echo "" >> bin/windows/docs/LICENSE.txt
echo "THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED." >> bin/windows/docs/LICENSE.txt
echo "IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE." >> bin/windows/docs/LICENSE.txt

# Final output message
echo "Binaries and NASM successfully copied to ./bin"
