@echo off
setlocal

rem Clean previous builds
rmdir /S /Q bin
mkdir bin\linux
mkdir bin\windows

rem Define WSL and Cargo path
set WSL_PATH=C:\Windows\System32\wsl.exe
set CARGO_PATH=~/.cargo/bin/cargo

rem Build static Linux binary with MUSL using WSL
%WSL_PATH% %CARGO_PATH% build --release --target x86_64-unknown-linux-gnu
if %ERRORLEVEL% neq 0 (
    echo Linux binary build failed.
    exit /b 1
) else (
    echo Linux binary built successfully.
)

rem Copy Linux binary to bin\linux
%WSL_PATH% cp ./target/x86_64-unknown-linux-gnu/release/neit bin/linux/neit
if %ERRORLEVEL% neq 0 (
    echo Failed to copy Linux binary.
    exit /b 1
)

rem Build static Windows binary with MSVC
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\Community\VC\vcvarsall.bat" x64
cargo build --release --target x86_64-pc-windows-msvc
if %ERRORLEVEL% neq 0 (
    echo Windows binary build failed.
    exit /b 1
) else (
    echo Windows binary built successfully.
)

rem Copy Windows binary to bin\windows
copy ./target/x86_64-pc-windows-msvc/release/neit.exe bin\windows\neit.exe
if %ERRORLEVEL% neq 0 (
    echo Failed to copy Windows binary.
    exit /b 1
)

rem Include the NASM folder for Windows
echo Copying NASM files...
mkdir bin\windows\nasm
xcopy /E /I /Y nasm\* bin\windows\nasm\
if %ERRORLEVEL% neq 0 (
    echo Failed to copy NASM files.
    exit /b 1
)

rem Include Visual C++ Redistributable if required
copy vc_redist.x64.exe bin\windows\
if %ERRORLEVEL% neq 0 (
    echo Failed to copy Visual C++ Redistributable.
    exit /b 1
)

rem Create the docs folder in bin\windows with README and LICENSE
echo Creating docs folder with README and LICENSE...
mkdir bin\windows\docs

rem Generate README and LICENSE files dynamically
(
    echo This program includes NASM, the Netwide Assembler, for assembling code.
    echo Refer to the LICENSE file for more details.
) > bin\windows\docs\README.txt

(
    echo NASM is licensed under the 2-clause BSD license.
    echo Redistribution and use in source and binary forms, with or without modification, are permitted under the following conditions:
    echo.
    echo 1. Redistributions of source code must retain the above copyright notice, this list of conditions, and the following disclaimer.
    echo 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions, and the following disclaimer in the documentation and/or other materials provided with the distribution.
    echo.
    echo THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS 'AS IS' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
    echo IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
) > bin\windows\docs\LICENSE.txt

rem Final output message
echo Binaries and NASM successfully copied to .\bin

endlocal
pause
