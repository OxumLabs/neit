<div style="text-align:center">
<img src="logo.png" width="80px">
<h2 style="font-size: 45px; color: #4CAF50; text-align: center; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.3);">
    Neit Programming Language
</h2>

<p style="font-size: 30px; color: #f 0f0f0; background-color: #1a1a1a; padding: 20px; border-radius: 10px; line-height: 1.6; text-align: center;">
    The Neit Programming Language is a modern general-use programming language designed to be very easy by design and fast for production. It builds executables that are libc dependency-free, making the final build extremely lightweight and fast. We utilize the LLVM compiler (work in progress for now; ASM is our go-to). Some simple code in Neit is shown below.
</p>

## </div>

## Code Example

---

```neit

#define functions with fn keyword
fn hi() { #a local function

}

pub fn hey(){ #public function

}

print("Hello world") # print hello world without new line

println("Hello") # prints hello world with new line

hey() #call fucntions

may x = 0 # declarae variables (immutable for now) using may keyword

may z = x-1 # supports mathematical operand like + , - , / , * , ** , // , %

fn hello(){} # btw empty functions

pub fn dem(){} # and empty public functions

println("{z}") # print variables by putting them in {}

println("{100**29}") # print supports maths aswell :3

fn yo(x : int , y : string){} # functions can take arguments but they dont do anything for now :(

```

---

---

## Installation Instructions

---

### Windows Installation

Download and install the Windows MSVC compiler to get the linker. Follow the instructions from the official Microsoft site.

### Linux Installation

For Linux:
Make sure you have NASM and the LD linker installed on your system. You can install them using your package manager:

```bash
# for debian and ubuntu based
sudo apt install nasm binutils

# for fedora and its deriavatives / RHEL
sudo dnf install nasm binutils


# for Arch and its deriavatives
sudo pacman -S nasm binutils

# opensuse
sudo zypper install nasm binutils


# alpine linux
sudo apk add nasm binutils

```

---

# This Project is under the **Apache License 2.0 with additional requirements so anyone who is going to fork this work is obliged to read the license thouroghly (its short dont worry)** here : [license](LICENSE)
