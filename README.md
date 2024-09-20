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

# Apache License

**Copyright 2024 Jay Tirth Kundan, Oxum Labs, bilal kanjelkheir**

Apache License  
Version 2.0, January 2004  
http://www.apache.org/licenses/

## TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

### 1. Definitions.

"License" shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.

### 2. Grant of License.

Subject to the terms and conditions of this License, each Contributor hereby grants You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable license to use, reproduce, modify, prepare Derivative Works of, publicly display, publicly perform, sublicense, and distribute the Work and such Derivative Works in Source or Object form.

### 3. Conditions of Attribution.

You must retain, in the Source form of any Derivative Works that You distribute, all copyright, patent, trademark, and attribution notices from the Source form of the Work, excluding those notices that do not pertain to any part of the Derivative Works.

**Additionally, if you modify or distribute this software, you must provide clear and prominent attribution to [Your Name/Company]** in all copies or substantial portions of the software, including modified versions. Attribution must be included both in any associated documentation and user-visible notices (if such exist).

### 4. Disclaimer of Warranty.

Unless required by applicable law or agreed to in writing, Licensor provides the Work (and each Contributor provides its Contributions) on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied, including, without limitation, any warranties or conditions of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A PARTICULAR PURPOSE. You are solely responsible for determining the appropriateness of using or redistributing the Work and assume any risks associated with Your exercise of permissions under this License.

### 5. Limitation of Liability.

In no event and under no legal theory, whether in tort (including negligence), contract, or otherwise, unless required by applicable law (such as deliberate and grossly negligent acts) or agreed to in writing, shall any Contributor be liable to You for damages, including any direct, indirect, special, incidental, or consequential damages of any character arising as a result of this License or out of the use or inability to use the Work (including but not limited to damages for loss of goodwill, work stoppage, computer failure or malfunction, or any and all other commercial damages or losses), even if such Contributor has been advised of the possibility of such damages.
