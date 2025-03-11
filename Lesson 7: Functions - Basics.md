# **Lesson 7: Functions - Basics**

Welcome to Lesson 7! Today, we’ll dive into **functions** in Rust. Functions are reusable blocks of code that perform a specific task. We’ll cover how to **declare and call functions**, as well as how to use **function parameters** and **return types**. Let’s get started!

---

## **1. Declaring and Calling Functions**

In Rust, you declare a function using the `fn` keyword. Functions can be called by using their name followed by parentheses `()`.

### **Syntax**
```rust
fn function_name() {
    // Code to execute
}
```

### **Example**
```rust
fn greet() {
    println!("Hello, world!");
}

fn main() {
    greet(); // Call the function
}
```

#### **Output**
```
Hello, world!
```

---

## **2. Function Parameters**

Functions can take **parameters** (inputs) to make them more flexible. Parameters are specified in the function signature.

### **Syntax**
```rust
fn function_name(param1: Type1, param2: Type2) {
    // Code to execute
}
```

### **Example**
```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet("Alice"); // Call the function with an argument
    greet("Bob");
}
```

#### **Output**
```
Hello, Alice!
Hello, Bob!
```

---

## **3. Function Return Types**

Functions can return values using the `->` syntax. The return type is specified after the function signature.

### **Syntax**
```rust
fn function_name() -> ReturnType {
    // Code to execute
    value // Return a value
}
```

### **Example**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // Return the sum of a and b
}

fn main() {
    let result = add(5, 3); // Call the function and store the result
    println!("The sum is: {}", result);
}
```

#### **Output**
```
The sum is: 8
```

---

## **4. Putting It All Together**

Let’s write a program that uses functions with parameters and return types:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    greet("Alice"); // Call the greet function
    greet("Bob");

    let result = add(5, 3); // Call the add function
    println!("The sum is: {}", result);
}
```

#### **Output**
```
Hello, Alice!
Hello, Bob!
The sum is: 8
```

---

## **5. Key Points**
- **Declaring Functions**: Use the `fn` keyword.
- **Calling Functions**: Use the function name followed by `()`.
- **Parameters**: Specify inputs in the function signature.
- **Return Types**: Use `->` to specify the return type.

---

## **6. Practice Exercises**

1. **Basic Function**:
   - Write a function called `say_hello` that prints "Hello, world!".

2. **Function with Parameters**:
   - Write a function called `multiply` that takes two integers as parameters and prints their product.

3. **Function with Return Type**:
   - Write a function called `subtract` that takes two integers as parameters and returns their difference.

---

## **7. Solutions to Practice Exercises**

### **Exercise 1: Basic Function**
```rust
fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    say_hello();
}
```

#### **Output**
```
Hello, world!
```

---

### **Exercise 2: Function with Parameters**
```rust
fn multiply(a: i32, b: i32) {
    println!("The product is: {}", a * b);
}

fn main() {
    multiply(5, 3);
}
```

#### **Output**
```
The product is: 15
```

---

### **Exercise 3: Function with Return Type**
```rust
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let result = subtract(10, 4);
    println!("The difference is: {}", result);
}
```

#### **Output**
```
The difference is: 6
```

---
