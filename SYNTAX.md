
# Neit Programming Language Guide

## **Neit Requirements**
Neit requires `clang` to be installed on your system for building code. However, `clang` is not required to run the code using the interpreter.

---

## **Neit Command Line Arguments**

Neit provides a set of easy-to-use command-line arguments for efficient development.

### **Main Commands**

- **`run`**  
  - Lexes, parses, (future) optimizes, and runs Neit code in the interpreter.
  - *Example*:  
    ```bash
    neit run joy.nsc
    ```

- **`build`**  
  - Lexes, parses, generates C code, and builds it with `clang`.
  - *Example*:  
    ```bash
    neit build joy.nsc
    ```

### **Options**

- **`-h` or `--help`**  
  - Displays help details about commands and options.  
  - *Example*:  
    ```bash
    neit -h
    ```

- **`-grammar=<file>` or `-g=<file>`**
  - Specifies a grammar file for parsing. File extension does not matter.  
  - *Example*:  
    ```bash
    neit run joy.nsc -grammar=grammar.txt
    ```

- **`-opt=<level>`** or `--optimisation=<level>`
  - Specifies the optimization level for code generation.  
    - **`1`** – Least optimizations for minor speed improvements.  
    - **`2`** – Moderate optimizations for a balance of speed and size.  
    - **`3`** – Full optimizations for faster and smaller output.  
    - **`4`** – Aggressive optimizations with longer compile times.  
  - *Example*:  
    ```bash
    neit build joy.nsc -opt=2
    ```

- **`-static`**  
  - Generates a statically linked binary.  
  - *Example*:  
    ```bash
    neit build joy.nsc -static
    ```

- **`-o=<name>`** or `--out=<name>`
  - Specifies the output file name (default: `output`).  
  - *Example*:  
    ```bash
    neit build joy.nsc -o=mybinary
    ```

- **`-cls`**  
  - Clears the screen before running code (applies to the `run` command).  
  - *Example*:  
    ```bash
    neit run joy.nsc -cls
    ```

- **`--retain-c`** or `-rc`
  - Retains the generated C file after building.  
  - *Example*:  
    ```bash
    neit build joy.nsc -retain-c
    ```

---

## **Neit Syntax**

### **Variable Declaration**

- Declare a string variable:  
  ```neit
  may name = "joy"
  ```

- Declare an integer variable:  
  ```neit
  may age = 16
  ```

- Declare a float variable:  
  ```neit
  may height = 16.2
  ```
- Refrence a variable to another variable:  
  ```neit
  may name2 = name
  ```
### **Re-Assign values to variables**
- you can assign new values to variables in the following way:
   ```may name = "joy" # a variable called name
  name = "yoj" # reassign it
  ```

### **Print Statements**

- Print normal text:  
  ```neit
  print Hello world
  ```

- Print the value of a variable:  
  ```neit
  print {name}
  ```

### **Comments**

- **Single-line comments** use the `#` symbol:  
  ```neit
  # This is a single-line comment
  print Hello
  ```

- **Multi-line comments** use `##` to begin and end:  
  ```neit
  ## Multi-line comment example:
  This spans multiple lines ##
  ```

### **Input**

- Take input from the user:  
  ```neit
  may name = takein()
  print {name}
  ```

### **Clear Screen**

- Clear the screen using the `cls` command:  
  ```neit
  cls
  ```

### **Functions (Commands)**

- Define a command (function):  
  ```neit
  cmd hi {
    (name:str) # Define parameters (currently ignored but required)
    println hi
  }
  ```
  > please note that the arguments are optinal and they can be empty , just put `()`

- Call a command:  
  ```neit
  call hi {"joy"}
  ```

- If no arguments are required:  
  ```neit
  call hi
  ```
- Wait for <numerical_value><ms|hr|s|m>
  we can wait for a certain amount of time thanks to the easy to use `wait` command which works as follows:
  ```neit
  wait 1s
  ```
  this makes the program wait for 1 second
  the possible units are:
   - s :~ second
   - ms :~ millisecond
   - m :~ minute
   - hr :~ hour

### **while loop**
while loops works like `if` statements but they run as long as the condition meets!
```neit
while (cond){
  //code
}
```
### **Conditional Statements (Only ``if`` for now)**
neit also supports conditional statements , but for now only supports ``if`` and can be used in the following way
```neit
if (cond){
  #code
}
```
for example say we wanna match if ``1 == 2`` so:
```neit
if (1 == 2){
  println Nuh uh
}
```
we can also match strings , neit ships with its own implementation of ``strcmp`` which is put directly into the generated code but we are writting a ``stdlib`` in C for neit called ``nelibc`` , anyways you can compare strings like usual:
```neit
may name = takein()
if (name == "joy"){
  println Hello joyyyyy!!!!
}
```
---

### **Exit Command**

- Exit the program with the specified status code
  ```neit
  exit {status_code}
  ```
  available status codes are:
  - for success use : ``ok`` , ``success`` , ``0``
  - for failure use : ``fail`` , ``failure`` , ``1``
  - for invalid argument use : ``invalid arg`` , ``inv arg`` , ``128``
  - for not found use : ``not found`` , ``nf`` , ``127``
  - for permission error use : ``permission err`` , ``perm err`` , ``permission denied`` , ``126``
  - for killed use : ``killed`` , ``kill`` , ``137``
  - for interrupt use : ``interrupt`` , ``int`` , ``signal int`` , ``130``
  - for segmentation fault use : ``segfault`` , ``seg`` , ``segmentation fault`` , ``11``
  - for out of range use : ``out of range`` , ``range error`` , ``255``
  
  for example:
  ```neit
  exit fail
  ```

---

## **Semigen Grammar**

You can specify custom grammar rules to change commands. The format is:

```neit
<original_command> ~ <replacement>
```

### Example:
```neit
print ~ say
```

This replaces `print` with `say`. You can now use `say` instead of `print` to output text, while still being able to use `print`.

> **Note:** Only commands can be changed, not other syntax elements.

---
