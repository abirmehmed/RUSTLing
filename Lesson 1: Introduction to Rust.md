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


# **Practice Exercises for Lesson 1: Introduction to Rust**

Here are some **practice exercises** to help you get comfortable with the basics of Rust, including setting up the environment, writing your first program, and understanding compiled vs. interpreted languages.

---

## **Exercise 1: Modify the "Hello, World!" Program**

### **Task**
Modify the "Hello, World!" program to print your name instead of "World".

### **Steps**
1. Open the `src/main.rs` file in your `hello_world` project.
2. Replace `"Hello, World!"` with `"Hello, [Your Name]!"`.
3. Run the program using `cargo run`.

### **Example Solution**
```rust
fn main() {
    println!("Hello, Alice!");
}
```

#### **Output**
```
Hello, Alice!
```

---

## **Exercise 2: Create a Personalized Greeting Program**

### **Task**
Use Cargo to create a new project called `greeting` and write a program that asks for the user’s name and prints a personalized greeting.

### **Steps**
1. Create a new project:
   ```bash
   cargo new greeting
   cd greeting
   ```
2. Open the `src/main.rs` file and write the following code:
   ```rust
   use std::io;

   fn main() {
       println!("What is your name?");

       let mut name = String::new();
       io::stdin()
           .read_line(&mut name)
           .expect("Failed to read input");

       println!("Hello, {}! Welcome to Rust.", name.trim());
   }
   ```
3. Run the program using `cargo run`.

### **Example Output**
```
What is your name?
Alice
Hello, Alice! Welcome to Rust.
```

---

## **Exercise 3: Understand Compiled vs. Interpreted Languages**

### **Task**
Write a short explanation (in comments or a separate text file) of the differences between compiled and interpreted languages. Use Rust and Python as examples.

### **Example Explanation**
```rust
// Compiled vs. Interpreted Languages
//
// Compiled Languages (e.g., Rust):
// - The source code is translated into machine code by a compiler before execution.
// - The resulting binary file can be run directly by the computer.
// - Advantages: Faster execution, better performance for system-level tasks.
// - Disadvantages: Longer development cycle, platform-specific binaries.
//
// Interpreted Languages (e.g., Python):
// - The source code is executed line-by-line by an interpreter at runtime.
// - Advantages: Easier to debug and test, platform-independent.
// - Disadvantages: Slower execution, less control over low-level system resources.
```

---

## **Exercise 4: Explore Cargo Commands**

### **Task**
Experiment with the following Cargo commands and write down what each one does:
1. `cargo build`
2. `cargo run`
3. `cargo check`
4. `cargo clean`

### **Example Notes**
```rust
// Cargo Commands:
//
// 1. `cargo build`: Compiles the project and generates an executable binary.
// 2. `cargo run`: Compiles and runs the project in one step.
// 3. `cargo check`: Checks the code for errors without generating an executable.
// 4. `cargo clean`: Removes the `target` directory, cleaning up build artifacts.
```

---

## **Exercise 5: Create a New Project**

### **Task**
Create a new Rust project called `calculator` using Cargo. Write a simple program that adds two numbers and prints the result.

### **Steps**
1. Create a new project:
   ```bash
   cargo new calculator
   cd calculator
   ```
2. Open the `src/main.rs` file and write the following code:
   ```rust
   fn main() {
       let a = 5;
       let b = 3;
       let sum = a + b;
       println!("The sum of {} and {} is: {}", a, b, sum);
   }
   ```
3. Run the program using `cargo run`.

### **Example Output**
```
The sum of 5 and 3 is: 8
```

---

## **Exercise 6: Experiment with `println!`**

### **Task**
Experiment with the `println!` macro to print different types of data, such as:
- Integers
- Floating-point numbers
- Strings
- Booleans

### **Example Code**
```rust
fn main() {
    let integer = 42;
    let float = 3.14;
    let string = "Hello, Rust!";
    let boolean = true;

    println!("Integer: {}", integer);
    println!("Float: {}", float);
    println!("String: {}", string);
    println!("Boolean: {}", boolean);
}
```

#### **Output**
```
Integer: 42
Float: 3.14
String: Hello, Rust!
Boolean: true
```

---

## **Exercise 7: Write a Program to Calculate the Area of a Rectangle**

### **Task**
Write a program that calculates the area of a rectangle. The program should:
1. Define variables for the length and width of the rectangle.
2. Calculate the area using the formula: `area = length * width`.
3. Print the result.

### **Example Code**
```rust
fn main() {
    let length = 10;
    let width = 5;
    let area = length * width;
    println!("The area of the rectangle is: {}", area);
}
```

#### **Output**
```
The area of the rectangle is: 50
```

---

