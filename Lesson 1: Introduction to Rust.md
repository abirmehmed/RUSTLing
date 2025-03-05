# **Lesson 1: Introduction to Rust**

Welcome to your first lesson on Rust! Today, we’ll cover the basics of Rust, set up your development environment, write your first Rust program, and understand the difference between compiled and interpreted languages. Let’s get started!

---

## **1. Overview of Rust**

### **What is Rust?**
Rust is a modern systems programming language designed for:
- **Safety**: Rust ensures memory safety at compile time, preventing common bugs like null pointer dereferencing and buffer overflows.
- **Performance**: Rust is as fast as C/C++ and is often used in performance-critical applications.
- **Concurrency**: Rust makes it easier to write concurrent programs by preventing data races at compile time.

### **Why Learn Rust?**
- Rust is widely used in systems programming, web assembly, game development, and more.
- It has a growing ecosystem and is loved by developers for its modern tooling and community support.

---

## **2. Setting Up the Environment**

To start programming in Rust, you’ll need to set up your development environment. Rust provides an easy-to-use toolchain installer called **Rustup**.

### **Installing Rustup**
1. **Visit the Rustup Website**: Go to [https://rustup.rs/](https://rustup.rs/).
2. **Install Rustup**:
   - On **Linux/macOS**, run the following command in your terminal:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - On **Windows**, download and run the `rustup-init.exe` installer from the website.
3. **Follow the Prompts**: The installer will guide you through the setup process. Choose the default options unless you have specific preferences.

### **Verifying the Installation**
Once Rustup is installed, verify the installation by running:
```bash
rustc --version
```
This should display the installed Rust version (e.g., `rustc 1.75.0`).

### **What is Cargo?**
Cargo is Rust’s build system and package manager. It handles:
- Compiling your code.
- Downloading and managing dependencies.
- Running tests.
- Building documentation.

Check if Cargo is installed:
```bash
cargo --version
```

---

## **3. Writing Your First "Hello, World!" Program**

Let’s write your first Rust program: the classic "Hello, World!".

### **Steps**:
1. **Create a New Project**:
   Use Cargo to create a new project:
   ```bash
   cargo new hello_world
   cd hello_world
   ```
   This creates a directory called `hello_world` with the following structure:
   ```
   hello_world/
   ├── Cargo.toml
   └── src/
       └── main.rs
   ```

2. **Write the Code**:
   Open the `src/main.rs` file and replace its content with:
   ```rust
   fn main() {
       println!("Hello, World!");
   }
   ```
   - `fn main()`: Defines the main function, the entry point of the program.
   - `println!`: A macro that prints text to the console.

3. **Run the Program**:
   Use Cargo to compile and run the program:
   ```bash
   cargo run
   ```
   You should see the output:
   ```
   Hello, World!
   ```

---

## **4. Understanding Compiled vs. Interpreted Languages**

### **Compiled Languages (e.g., Rust, C, C++)**
- **How They Work**: The source code is translated into machine code by a compiler before execution. The resulting binary file can be run directly by the computer.
- **Advantages**:
  - Faster execution since the code is already translated into machine language.
  - Better performance for system-level tasks.
- **Disadvantages**:
  - Longer development cycle due to the need for compilation.
  - Platform-specific binaries (you need to compile for each target platform).

### **Interpreted Languages (e.g., Python, JavaScript)**
- **How They Work**: The source code is executed line-by-line by an interpreter at runtime.
- **Advantages**:
  - Easier to debug and test since there’s no separate compilation step.
  - Platform-independent (the same code can run on any platform with the interpreter).
- **Disadvantages**:
  - Slower execution compared to compiled languages.
  - Less control over low-level system resources.

### **Rust as a Compiled Language**
Rust is a compiled language, meaning your code is translated into machine code before execution. This gives Rust its performance advantages but requires a compilation step during development.

---

## **5. Key Takeaways**
- **Rust Overview**: A modern systems programming language focused on safety, performance, and concurrency.
- **Environment Setup**: Installed Rust using Rustup and learned about Cargo.
- **First Program**: Wrote and ran a "Hello, World!" program in Rust.
- **Compiled vs. Interpreted Languages**: Explored the differences and understood why Rust is a compiled language.

---

## **6. Practice Exercise**
1. Modify the "Hello, World!" program to print your name instead of "World".
2. Use Cargo to create a new project called `greeting` and write a program that prints a personalized greeting.

---
