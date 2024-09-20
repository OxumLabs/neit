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
_WRT("Hello world")

# This will be replaced by std lib echoln

fn hi() {
    _WRT("hi")
}

hi()

# Call the function

fn add(i: int, q: float) {}
add(1, 2)

# Function with arguments

may name = "neit"
may b = 9
may z = 9.9
may x = 90 + 80

# Only constant expressions for now
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

# MIT License

---

**Copyright 2024 Jay Tirth Kundan, Oxum Labs, Bilal Khalil**

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

**THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.**

---
