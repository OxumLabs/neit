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
echo Building Linux binary...
%WSL_PATH% %CARGO_PATH% build --release --target x86_64-unknown-linux-gnu
if %ERRORLEVEL% neq 0 (
    echo Linux binary build failed.
    exit /b 1
) else (
    echo Linux binary built successfully.
)

rem Copy Linux binary to bin\linux
echo Copying Linux binary...
%WSL_PATH% cp /mnt/f/rust/neit/target/x86_64-unknown-linux-gnu/release/neit /mnt/f/rust/neit/bin/linux/neit
if %ERRORLEVEL% neq 0 (
    echo Failed to copy Linux binary.
    exit /b 1
)

rem Build static Windows binary with MSVC
echo Building Windows binary...
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\Community\VC\vcvarsall.bat" x64
cargo build --release --target x86_64-pc-windows-msvc
if %ERRORLEVEL% neq 0 (
    echo Windows binary build failed.
    exit /b 1
) else (
    echo Windows binary built successfully.
)

rem Copy Windows binary to bin\windows
echo Copying Windows binary...
copy .\target\x86_64-pc-windows-msvc\release\neit.exe bin\windows\neit.exe
if %ERRORLEVEL% neq 0 (
    echo Failed to copy Windows binary.
    exit /b 1
)

rem Include Visual C++ Redistributable if required
echo Copying Visual C++ Redistributable...
copy vc_redist.x64.exe bin\windows\
if %ERRORLEVEL% neq 0 (
    echo Failed to copy Visual C++ Redistributable.
    exit /b 1
)


rem Final output message
echo Binaries files successfully copied to .\bin

endlocal
pause
