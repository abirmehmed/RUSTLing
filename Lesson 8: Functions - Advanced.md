# **Lesson 8: Functions - Advanced**

Welcome to Lesson 8! Today, we’ll explore some **advanced topics** related to functions in Rust. While Rust doesn’t support **function overloading** directly, we’ll look at alternatives. We’ll also introduce **closures**, which are anonymous functions that can capture their environment. Let’s dive in!

---

## **1. Function Overloading (Alternatives)**

Function overloading is the ability to define multiple functions with the same name but different parameters. Rust doesn’t support this directly, but we can achieve similar behavior using **generics** or **different function names**.

---

### **Alternative 1: Using Generics**

Generics allow you to write functions that work with multiple types.

#### **Example**
```rust
fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

fn main() {
    print_value(42); // Works with integers
    print_value("Hello"); // Works with strings
}
```

#### **Output**
```
Value: 42
Value: Hello
```

---

### **Alternative 2: Using Different Function Names**

You can define functions with different names to handle different parameter types.

#### **Example**
```rust
fn print_int(value: i32) {
    println!("Integer: {}", value);
}

fn print_str(value: &str) {
    println!("String: {}", value);
}

fn main() {
    print_int(42);
    print_str("Hello");
}
```

#### **Output**
```
Integer: 42
String: Hello
```

---

## **2. Closures (Basic Introduction)**

Closures are **anonymous functions** that can capture variables from their surrounding environment. They are similar to lambda functions in other languages.

---

### **Syntax**
```rust
let closure_name = |parameters| -> ReturnType {
    // Code to execute
};
```

- Closures can infer their parameter and return types, so you don’t always need to specify them explicitly.

---

### **Example 1: Basic Closure**
```rust
fn main() {
    let add = |a, b| a + b; // Closure to add two numbers
    let result = add(5, 3);
    println!("The sum is: {}", result);
}
```

#### **Output**
```
The sum is: 8
```

---

### **Example 2: Capturing Environment**

Closures can capture variables from their surrounding scope.

```rust
fn main() {
    let x = 10;
    let add_x = |y| x + y; // Closure captures x from the environment
    let result = add_x(5);
    println!("The result is: {}", result);
}
```

#### **Output**
```
The result is: 15
```

---

### **Example 3: Closures as Function Arguments**

Closures can be passed as arguments to functions.

```rust
fn apply_operation<F>(x: i32, y: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32, // F is a closure that takes two i32s and returns an i32
{
    operation(x, y)
}

fn main() {
    let add = |a, b| a + b;
    let multiply = |a, b| a * b;

    println!("Addition: {}", apply_operation(5, 3, add));
    println!("Multiplication: {}", apply_operation(5, 3, multiply));
}
```

#### **Output**
```
Addition: 8
Multiplication: 15
```

---

## **3. Putting It All Together**

Let’s write a program that uses generics and closures:

```rust
fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

fn main() {
    // Using generics
    print_value(42);
    print_value("Hello");

    // Using closures
    let add = |a, b| a + b;
    let result = add(5, 3);
    println!("The sum is: {}", result);

    // Capturing environment
    let x = 10;
    let add_x = |y| x + y;
    let result = add_x(5);
    println!("The result is: {}", result);

    // Closures as function arguments
    let multiply = |a, b| a * b;
    println!("Multiplication: {}", apply_operation(5, 3, multiply));
}

fn apply_operation<F>(x: i32, y: i32, operation: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    operation(x, y)
}
```

#### **Output**
```
Value: 42
Value: Hello
The sum is: 8
The result is: 15
Multiplication: 15
```

---

## **4. Key Points**
- **Function Overloading**: Rust doesn’t support it directly, but you can use generics or different function names.
- **Closures**: Anonymous functions that can capture their environment.
- **Closures as Arguments**: Closures can be passed to functions for flexible behavior.

---

## **5. Practice Exercises**

1. **Generics**:
   - Write a generic function called `print_twice` that takes a value of any type (that can be printed) and prints it twice.

2. **Closures**:
   - Write a closure that multiplies two numbers and call it with arguments `5` and `3`.

3. **Closures as Arguments**:
   - Write a function called `apply_twice` that takes a closure and a value, applies the closure twice to the value, and returns the result.

---

## **6. Solutions to Practice Exercises**

### **Exercise 1: Generics**
```rust
fn print_twice<T: std::fmt::Display>(value: T) {
    println!("{} {}", value, value);
}

fn main() {
    print_twice(42);
    print_twice("Hello");
}
```

#### **Output**
```
42 42
Hello Hello
```

---

### **Exercise 2: Closures**
```rust
fn main() {
    let multiply = |a, b| a * b;
    let result = multiply(5, 3);
    println!("The product is: {}", result);
}
```

#### **Output**
```
The product is: 15
```

---

### **Exercise 3: Closures as Arguments**
```rust
fn apply_twice<F>(value: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(operation(value))
}

fn main() {
    let square = |x| x * x;
    let result = apply_twice(2, square);
    println!("The result is: {}", result);
}
```

#### **Output**
```
The result is: 16
```

---
