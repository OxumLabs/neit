@echo off
:: build.bat - Build menu for multiple Rust targets
:: This script installs targets, builds the project, and copies the binary to a bin directory.

:menu
cls
echo.
echo ====================================================================
echo         Select the platform you want to build for:
echo ====================================================================
echo  1.  Linux x86_64 (GNU)
echo  2.  Linux x86_64 (Musl)
echo  3.  Linux ARMv7
echo  4.  Linux ARM64
echo  5.  Linux x86 (32-bit)
echo  6.  Linux x86_64 (Android)
echo  7.  Linux ARM64 (Android)
echo  8.  Linux ARMv7 (Android)
echo  9.  Windows x86_64 (GNU)
echo 10.  Windows i686 (GNU)
echo 11.  Windows x86_64 (MSVC)
echo 12.  Windows i686 (MSVC)
echo 13.  macOS x86_64
echo 14.  macOS ARM64
echo 15.  FreeBSD x86_64
echo 16.  FreeBSD i686
echo 17.  FreeBSD ARM64
echo 18.  NetBSD x86_64
echo 19.  NetBSD i686
echo 20.  OpenBSD x86_64
echo 21.  WebAssembly
echo 22.  WebAssembly (WASI)
echo 23.  RISC-V
echo 24.  Install dependencies
echo 25.  Exit
echo ====================================================================
set /p choice="Please select an option (1-25): "

if "%choice%"=="1" goto build1
if "%choice%"=="2" goto build2
if "%choice%"=="3" goto build3
if "%choice%"=="4" goto build4
if "%choice%"=="5" goto build5
if "%choice%"=="6" goto build6
if "%choice%"=="7" goto build7
if "%choice%"=="8" goto build8
if "%choice%"=="9" goto build9
if "%choice%"=="10" goto build10
if "%choice%"=="11" goto build11
if "%choice%"=="12" goto build12
if "%choice%"=="13" goto build13
if "%choice%"=="14" goto build14
if "%choice%"=="15" goto build15
if "%choice%"=="16" goto build16
if "%choice%"=="17" goto build17
if "%choice%"=="18" goto build18
if "%choice%"=="19" goto build19
if "%choice%"=="20" goto build20
if "%choice%"=="21" goto build21
if "%choice%"=="22" goto build22
if "%choice%"=="23" goto build23
if "%choice%"=="24" goto install_dependencies
if "%choice%"=="25" goto exit

echo Invalid option. Press any key to return to the menu...
pause >nul
goto menu

:build1
echo Installing target: x86_64-unknown-linux-gnu...
rustup target add x86_64-unknown-linux-gnu
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-unknown-linux-gnu...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-unknown-linux-gnu
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-unknown-linux-gnu\release\neit" "bin\neit-linux-gnu"
echo Build completed successfully!
pause
goto menu

:build2
echo Installing target: x86_64-unknown-linux-musl...
rustup target add x86_64-unknown-linux-musl
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-unknown-linux-musl...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-unknown-linux-musl
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-unknown-linux-musl\release\neit" "bin\neit-linux-musl"
echo Build completed successfully!
pause
goto menu

:build3
echo Installing target: armv7-unknown-linux-gnueabihf...
rustup target add armv7-unknown-linux-gnueabihf
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for armv7-unknown-linux-gnueabihf...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target armv7-unknown-linux-gnueabihf
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\armv7-unknown-linux-gnueabihf\release\neit" "bin\neit-linux-armv7"
echo Build completed successfully!
pause
goto menu

:build4
echo Installing target: aarch64-unknown-linux-gnu...
rustup target add aarch64-unknown-linux-gnu
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for aarch64-unknown-linux-gnu...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target aarch64-unknown-linux-gnu
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\aarch64-unknown-linux-gnu\release\neit" "bin\neit-linux-arm64"
echo Build completed successfully!
pause
goto menu

:build5
echo Installing target: i686-unknown-linux-gnu...
rustup target add i686-unknown-linux-gnu
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for i686-unknown-linux-gnu...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target i686-unknown-linux-gnu
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\i686-unknown-linux-gnu\release\neit" "bin\neit-linux-x86-32"
echo Build completed successfully!
pause
goto menu

:build6
echo Installing target: x86_64-unknown-linux-android...
rustup target add x86_64-unknown-linux-android
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-unknown-linux-android...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-unknown-linux-android
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-unknown-linux-android\release\neit" "bin\neit-linux-android-x86"
echo Build completed successfully!
pause
goto menu

:build7
echo Installing target: aarch64-unknown-linux-android...
rustup target add aarch64-unknown-linux-android
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for aarch64-unknown-linux-android...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target aarch64-unknown-linux-android
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\aarch64-unknown-linux-android\release\neit" "bin\neit-linux-android-arm64"
echo Build completed successfully!
pause
goto menu

:build8
echo Installing target: armv7-unknown-linux-androideabi...
rustup target add armv7-unknown-linux-androideabi
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for armv7-unknown-linux-androideabi...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target armv7-unknown-linux-androideabi
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\armv7-unknown-linux-androideabi\release\neit" "bin\neit-linux-android-armv7"
echo Build completed successfully!
pause
goto menu

:build9
echo Installing target: x86_64-pc-windows-gnu...
rustup target add x86_64-pc-windows-gnu
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-pc-windows-gnu...
cargo build --release --target x86_64-pc-windows-gnu
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-pc-windows-gnu\release\neit.exe" "bin\neit-windows-gnu-x86_64.exe"
echo Build completed successfully!
pause
goto menu

:build10
echo Installing target: i686-pc-windows-gnu...
rustup target add i686-pc-windows-gnu
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for i686-pc-windows-gnu...
cargo build --release --target i686-pc-windows-gnu
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\i686-pc-windows-gnu\release\neit.exe" "bin\neit-windows-gnu-i686.exe"
echo Build completed successfully!
pause
goto menu

:build11
echo Installing target: x86_64-pc-windows-msvc...
rustup target add x86_64-pc-windows-msvc
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-pc-windows-msvc...
cargo build --release --target x86_64-pc-windows-msvc
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-pc-windows-msvc\release\neit.exe" "bin\neit-windows-msvc-x86_64.exe"
echo Build completed successfully!
pause
goto menu

:build12
echo Installing target: i686-pc-windows-msvc...
rustup target add i686-pc-windows-msvc
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for i686-pc-windows-msvc...
cargo build --release --target i686-pc-windows-msvc
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\i686-pc-windows-msvc\release\neit.exe" "bin\neit-windows-msvc-i686.exe"
echo Build completed successfully!
pause
goto menu

:build13
echo Installing target: x86_64-apple-darwin...
rustup target add x86_64-apple-darwin
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-apple-darwin...
cargo build --release --target x86_64-apple-darwin
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-apple-darwin\release\neit" "bin\neit-macos-x86_64"
echo Build completed successfully!
pause
goto menu

:build14
echo Installing target: aarch64-apple-darwin...
rustup target add aarch64-apple-darwin
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for aarch64-apple-darwin...
cargo build --release --target aarch64-apple-darwin
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\aarch64-apple-darwin\release\neit" "bin\neit-macos-arm64"
echo Build completed successfully!
pause
goto menu

:build15
echo Installing target: x86_64-unknown-freebsd...
rustup target add x86_64-unknown-freebsd
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-unknown-freebsd...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-unknown-freebsd
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-unknown-freebsd\release\neit" "bin\neit-freebsd-x86_64"
echo Build completed successfully!
pause
goto menu

:build16
echo Installing target: i686-unknown-freebsd...
rustup target add i686-unknown-freebsd
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for i686-unknown-freebsd...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target i686-unknown-freebsd
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\i686-unknown-freebsd\release\neit" "bin\neit-freebsd-i686"
echo Build completed successfully!
pause
goto menu

:build17
echo Installing target: aarch64-unknown-freebsd...
rustup target add aarch64-unknown-freebsd
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for aarch64-unknown-freebsd...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target aarch64-unknown-freebsd
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\aarch64-unknown-freebsd\release\neit" "bin\neit-freebsd-arm64"
echo Build completed successfully!
pause
goto menu

:build18
echo Installing target: x86_64-unknown-netbsd...
rustup target add x86_64-unknown-netbsd
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-unknown-netbsd...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-unknown-netbsd
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-unknown-netbsd\release\neit" "bin\neit-netbsd-x86_64"
echo Build completed successfully!
pause
goto menu

:build19
echo Installing target: i686-unknown-netbsd...
rustup target add i686-unknown-netbsd
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for i686-unknown-netbsd...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target i686-unknown-netbsd
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\i686-unknown-netbsd\release\neit" "bin\neit-netbsd-i686"
echo Build completed successfully!
pause
goto menu

:build20
echo Installing target: x86_64-unknown-openbsd...
rustup target add x86_64-unknown-openbsd
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for x86_64-unknown-openbsd...
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release --target x86_64-unknown-openbsd
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\x86_64-unknown-openbsd\release\neit" "bin\neit-openbsd-x86_64"
echo Build completed successfully!
pause
goto menu

:build21
echo Installing target: wasm32-unknown-unknown...
rustup target add wasm32-unknown-unknown
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for wasm32-unknown-unknown...
cargo build --release --target wasm32-unknown-unknown
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\wasm32-unknown-unknown\release\neit" "bin\neit-wasm"
echo Build completed successfully!
pause
goto menu

:build22
echo Installing target: wasm32-wasi...
rustup target add wasm32-wasi
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for wasm32-wasi...
cargo build --release --target wasm32-wasi
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\wasm32-wasi\release\neit" "bin\neit-wasm-wasi"
echo Build completed successfully!
pause
goto menu

:build23
echo Installing target: riscv64gc-unknown-none-elf...
rustup target add riscv64gc-unknown-none-elf
if errorlevel 1 (
    echo Failed to install target.
    pause
    goto menu
)
echo Building for riscv64gc-unknown-none-elf...
cargo build --release --target riscv64gc-unknown-none-elf
if errorlevel 1 (
    echo Build failed!
    pause
    goto menu
)
if not exist bin mkdir bin
copy "target\riscv64gc-unknown-none-elf\release\neit" "bin\neit-riscv"
echo Build completed successfully!
pause
goto menu

:install_dependencies
echo ====================================================================
echo Attempting to install dependencies...
echo ====================================================================
echo.
echo Please install the following dependencies manually:
echo - Rust (https://www.rust-lang.org/tools/install)
echo - Visual Studio Build Tools (for MSVC targets)
echo - GNU toolchain (for GNU targets)
echo ====================================================================
pause
goto menu

:exit
echo Exiting...
exit /B 0
