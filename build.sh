#!/bin/bash
clear
# Color formatting functions for better visibility
color_reset="\033[0m"
color_green="\033[32m"
color_red="\033[31m"
color_yellow="\033[33m"
color_cyan="\033[36m"
color_bold="\033[1m"
color_underline="\033[4m"

install_dependencies() {
    echo -e "${color_bold}${color_cyan}Attempting to install dependencies...${color_reset}"
    echo "------------------------------------------------------------"
    sudo apt-get update
    sudo apt-get install -y build-essential libssl-dev pkg-config
    if [ $? -eq 0 ]; then
        echo -e "${color_bold}${color_green}Dependencies installed successfully!${color_reset}"
    else
        echo -e "${color_bold}${color_red}Failed to install dependencies.${color_reset}"
        echo -e "Please install them manually by running the following commands:"
        echo -e "${color_yellow}  - build-essential${color_reset}"
        echo -e "${color_yellow}  - libssl-dev${color_reset}"
        echo -e "${color_yellow}  - pkg-config${color_reset}"
    fi
    echo "------------------------------------------------------------"
    read -p "Press Enter to return to the menu..."
}

install_target() {
    target=$1
    echo -e "${color_bold}${color_cyan}Installing target: $target...${color_reset}"
    echo "------------------------------------------------------------"
    rustup target add "$target"
    if [ $? -eq 0 ]; then
        echo -e "${color_bold}${color_green}Target $target installed successfully!${color_reset}"
    else
        echo -e "${color_bold}${color_red}Failed to install target $target.${color_reset}"
    fi
    echo "------------------------------------------------------------"
    read -p "Press Enter to continue..."
}

build_for_target() {
    target=$1
    target_name=$2
    echo -e "${color_bold}${color_cyan}Building for $target...${color_reset}"
    echo "------------------------------------------------------------"
    RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target "$target"
    if [ $? -eq 0 ]; then
        echo -e "${color_bold}${color_green}Build for $target completed successfully!${color_reset}"

        # Copy the built binary to the bin directory with the correct name
        echo -e "${color_bold}${color_cyan}Copying to bin/...${color_reset}"
        mkdir -p bin

        # For Windows, append .exe to the binary name
        if [[ "$target" == *"windows"* ]]; then
            cp "target/$target/release/neit.exe" "bin/neit-$target_name.exe"
        else
            cp "target/$target/release/neit" "bin/neit-$target_name"
        fi
        if [[ "$target" != *"windows"* ]]; then
        chmod +x "bin/neit-$target_name"
        echo -e "${color_bold}${color_green}Copied to bin/neit-$target_name${color_reset}"
        fi
    else
        echo -e "${color_bold}${color_red}Failed to build for $target!${color_reset}"
    fi
    echo "------------------------------------------------------------"
    read -p "Press Enter to continue..."
}

show_menu() {
    clear
    echo -e "${color_bold}${color_yellow}Select the platform(s) you want to build for (comma separated, e.g., 1,2,3):${color_reset}"
    echo "------------------------------------------------------------"
    echo "1. Linux x86_64 (GNU)"
    echo "2. Linux x86_64 (Musl)"
    echo "3. Linux ARMv7"
    echo "4. Linux ARM64"
    echo "5. Linux x86 (32-bit)"
    echo "6. Linux x86_64 (Android)"
    echo "7. Linux ARM64 (Android)"
    echo "8. Linux ARMv7 (Android)"
    echo "9. Windows x86_64 (GNU)"
    echo "10. Windows i686 (GNU)"
    echo "11. Windows x86_64 (MSVC)"
    echo "12. Windows i686 (MSVC)"
    echo "13. macOS x86_64"
    echo "14. macOS ARM64"
    echo "15. FreeBSD x86_64"
    echo "16. FreeBSD i686"
    echo "17. FreeBSD ARM64"
    echo "18. NetBSD x86_64"
    echo "19. NetBSD i686"
    echo "20. OpenBSD x86_64"
    echo "21. WebAssembly"
    echo "22. WebAssembly (WASI)"
    echo "23. RISC-V"
    echo "24. Install dependencies"
    echo "25. Exit"
    echo "------------------------------------------------------------"
    read -p "Please select an option (1-25): " choice

    # Split comma-separated input into an array
    IFS=',' read -r -a choices <<< "$choice"

    for selected in "${choices[@]}"; do
        case $selected in
            1)
                install_target "x86_64-unknown-linux-gnu"
                build_for_target "x86_64-unknown-linux-gnu" "linux-gnu"
                ;;
            2)
                install_target "x86_64-unknown-linux-musl"
                build_for_target "x86_64-unknown-linux-musl" "linux-musl"
                ;;
            3)
                install_target "armv7-unknown-linux-gnueabihf"
                build_for_target "armv7-unknown-linux-gnueabihf" "linux-armv7"
                ;;
            4)
                install_target "aarch64-unknown-linux-gnu"
                build_for_target "aarch64-unknown-linux-gnu" "linux-arm64"
                ;;
            5)
                install_target "i686-unknown-linux-gnu"
                build_for_target "i686-unknown-linux-gnu" "linux-x86-32"
                ;;
            6)
                install_target "x86_64-unknown-linux-android"
                build_for_target "x86_64-unknown-linux-android" "linux-android-x86"
                ;;
            7)
                install_target "aarch64-unknown-linux-android"
                build_for_target "aarch64-unknown-linux-android" "linux-android-arm64"
                ;;
            8)
                install_target "armv7-unknown-linux-androideabi"
                build_for_target "armv7-unknown-linux-androideabi" "linux-android-armv7"
                ;;
            9)
                install_target "x86_64-pc-windows-gnu"
                build_for_target "x86_64-pc-windows-gnu" "windows-gnu-x86_64"
                ;;
            10)
                install_target "i686-pc-windows-gnu"
                build_for_target "i686-pc-windows-gnu" "windows-gnu-i686"
                ;;
            11)
                install_target "x86_64-pc-windows-msvc"
                build_for_target "x86_64-pc-windows-msvc" "windows-msvc-x86_64"
                ;;
            12)
                install_target "i686-pc-windows-msvc"
                build_for_target "i686-pc-windows-msvc" "windows-msvc-i686"
                ;;
            13)
                install_target "x86_64-apple-darwin"
                build_for_target "x86_64-apple-darwin" "macos-x86_64"
                ;;
            14)
                install_target "aarch64-apple-darwin"
                build_for_target "aarch64-apple-darwin" "macos-arm64"
                ;;
            15)
                install_target "x86_64-unknown-freebsd"
                build_for_target "x86_64-unknown-freebsd" "freebsd-x86_64"
                ;;
            16)
                install_target "i686-unknown-freebsd"
                build_for_target "i686-unknown-freebsd" "freebsd-i686"
                ;;
            17)
                install_target "aarch64-unknown-freebsd"
                build_for_target "aarch64-unknown-freebsd" "freebsd-arm64"
                ;;
            18)
                install_target "x86_64-unknown-netbsd"
                build_for_target "x86_64-unknown-netbsd" "netbsd-x86_64"
                ;;
            19)
                install_target "i686-unknown-netbsd"
                build_for_target "i686-unknown-netbsd" "netbsd-i686"
                ;;
            20)
                install_target "x86_64-unknown-openbsd"
                build_for_target "x86_64-unknown-openbsd" "openbsd-x86_64"
                ;;
            21)
                install_target "wasm32-unknown-unknown"
                build_for_target "wasm32-unknown-unknown" "wasm"
                ;;
            22)
                install_target "wasm32-wasi"
                build_for_target "wasm32-wasi" "wasm-wasi"
                ;;
            23)
                install_target "riscv64gc-unknown-none-elf"
                build_for_target "riscv64gc-unknown-none-elf" "riscv"
                ;;
            24)
                install_dependencies
                ;;
            25)
                echo -e "${color_bold}${color_green}Exiting...${color_reset}"
                exit 0
                ;;
            *)
                echo -e "${color_bold}${color_red}Invalid option $selected, skipping...${color_reset}"
                ;;
        esac
    done

    read -p "Press Enter to return to the menu..."
}

# Main loop
while true; do
    show_menu
done