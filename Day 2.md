# **Day 2: Basic Syntax in Rust**

Welcome to Day 2 of your Rust journey! Today, we’ll dive into the **basic syntax** of Rust, focusing on variables, data types, and operators. These are the building blocks of any Rust program, and mastering them will help you write more complex code in the future.

---

## **1. Variables**

### **What are Variables?**
Variables are used to store data in a program. In Rust, you declare variables using the `let` keyword.

### **Syntax**
```rust
let variable_name = value;
```

### **Example**
```rust
let x = 10; // x is an immutable variable with the value 10
```

### **Mutability**
By default, variables in Rust are **immutable**, meaning their value cannot be changed after assignment. To make a variable mutable, use the `mut` keyword.

#### **Immutable Variable**
```rust
let x = 10;
// x = 20; // This will cause a compile-time error
```

#### **Mutable Variable**
```rust
let mut y = 10;
y = 20; // This is allowed because y is mutable
```

### **Why Immutability?**
Immutability is a key feature of Rust that helps prevent bugs by ensuring that once a value is assigned, it cannot be accidentally changed.

---

## **2. Data Types**

Rust is a **statically typed** language, meaning the type of every variable is known at compile time. However, Rust can often infer the type, so you don’t always need to explicitly specify it.

### **Basic Data Types**
1. **Integers**:
   - Signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
   - Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
   - Example:
     ```rust
     let a: i32 = 42; // 32-bit signed integer
     let b: u64 = 100; // 64-bit unsigned integer
     ```

2. **Floats**:
   - Floating-point numbers: `f32`, `f64`
   - Example:
     ```rust
     let pi: f64 = 3.14159; // 64-bit floating-point number
     ```

3. **Booleans**:
   - Represents `true` or `false`.
   - Example:
     ```rust
     let is_rust_fun: bool = true;
     ```

4. **Characters**:
   - Represents a single Unicode character.
   - Example:
     ```rust
     let letter: char = 'R';
     ```

5. **Tuples**:
   - A collection of values of different types.
   - Example:
     ```rust
     let tuple: (i32, f64, char) = (42, 3.14, 'R');
     ```

6. **Arrays**:
   - A fixed-size collection of values of the same type.
   - Example:
     ```rust
     let arr: [i32; 3] = [1, 2, 3];
     ```

---

## **3. Operators**

Operators are symbols that perform operations on variables and values. Rust supports the following types of operators:

### **Arithmetic Operators**
- `+` (Addition)
- `-` (Subtraction)
- `*` (Multiplication)
- `/` (Division)
- `%` (Modulus)

#### **Example**
```rust
let a = 10;
let b = 3;

let sum = a + b; // 13
let difference = a - b; // 7
let product = a * b; // 30
let quotient = a / b; // 3 (integer division)
let remainder = a % b; // 1
```

### **Comparison Operators**
- `==` (Equal to)
- `!=` (Not equal to)
- `>` (Greater than)
- `<` (Less than)
- `>=` (Greater than or equal to)
- `<=` (Less than or equal to)

#### **Example**
```rust
let x = 10;
let y = 20;

println!("x == y: {}", x == y); // false
println!("x != y: {}", x != y); // true
println!("x > y: {}", x > y); // false
```

### **Logical Operators**
- `&&` (Logical AND)
- `||` (Logical OR)
- `!` (Logical NOT)

#### **Example**
```rust
let is_rust_fun = true;
let is_learning_easy = false;

println!("Rust is fun AND learning is easy: {}", is_rust_fun && is_learning_easy); // false
println!("Rust is fun OR learning is easy: {}", is_rust_fun || is_learning_easy); // true
println!("Rust is NOT fun: {}", !is_rust_fun); // false
```

---

## **4. Putting It All Together**

Let’s write a simple program that uses variables, data types, and operators:

```rust
fn main() {
    // Variables
    let mut x = 10;
    let y = 5;

    // Arithmetic operations
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = x % y;

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);

    // Comparison
    println!("x > y: {}", x > y);

    // Logical operations
    let is_rust_fun = true;
    let is_learning_easy = false;
    println!("Rust is fun AND learning is easy: {}", is_rust_fun && is_learning_easy);
}
```

---

## **5. Key Takeaways**
- **Variables**: Use `let` to declare variables and `mut` to make them mutable.
- **Data Types**: Rust has strong type inference, but you can explicitly specify types like `i32`, `f64`, `bool`, etc.
- **Operators**: Use arithmetic, comparison, and logical operators to manipulate data.

---

## **6. Practice Exercise**
1. Declare two variables, `a` and `b`, and perform all arithmetic operations on them.
2. Write a program to check if a number is even or odd using the modulus operator (`%`).
3. Use logical operators to check if a number is between 10 and 20.

---

