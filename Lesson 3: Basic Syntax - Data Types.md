# **Lesson 3: Basic Syntax - Data Types**

Welcome to Lesson 3! Today, we’ll explore Rust’s **basic data types**, including integers, floating-point numbers, booleans, and characters. Understanding these data types is essential for writing effective Rust programs. Let’s dive in!

---

## **1. Integers**

Integers are whole numbers without a fractional component. Rust supports both **signed** (positive and negative) and **unsigned** (only positive) integers.

### **Integer Types**
| Type  | Description                     | Range (Signed)          | Range (Unsigned)        |
|-------|---------------------------------|-------------------------|-------------------------|
| `i8`  | 8-bit signed integer            | -128 to 127             | -                       |
| `u8`  | 8-bit unsigned integer          | -                       | 0 to 255                |
| `i16` | 16-bit signed integer           | -32,768 to 32,767       | -                       |
| `u16` | 16-bit unsigned integer         | -                       | 0 to 65,535             |
| `i32` | 32-bit signed integer           | -2,147,483,648 to 2,147,483,647 | - |
| `u32` | 32-bit unsigned integer         | -                       | 0 to 4,294,967,295      |
| `i64` | 64-bit signed integer           | -9,223,372,036,854,775,808 to 9,223,372,036,854,775,807 | - |
| `u64` | 64-bit unsigned integer         | -                       | 0 to 18,446,744,073,709,551,615 |
| `isize` | Pointer-sized signed integer | Platform-dependent      | -                       |
| `usize` | Pointer-sized unsigned integer | -                       | Platform-dependent      |

### **Example**
```rust
fn main() {
    let a: i32 = 42; // 32-bit signed integer
    let b: u64 = 100; // 64-bit unsigned integer
    println!("a = {}, b = {}", a, b);
}
```

#### **Output**
```
a = 42, b = 100
```

---

## **2. Floating-Point Numbers**

Floating-point numbers represent numbers with a fractional component. Rust supports two floating-point types: `f32` (32-bit) and `f64` (64-bit).

### **Floating-Point Types**
| Type  | Description                     | Precision               |
|-------|---------------------------------|-------------------------|
| `f32` | 32-bit floating-point number    | ~6-9 decimal digits     |
| `f64` | 64-bit floating-point number    | ~15-17 decimal digits   |

### **Example**
```rust
fn main() {
    let pi: f64 = 3.14159; // 64-bit floating-point number
    let gravity: f32 = 9.81; // 32-bit floating-point number
    println!("pi = {}, gravity = {}", pi, gravity);
}
```

#### **Output**
```
pi = 3.14159, gravity = 9.81
```

---

## **3. Booleans**

Booleans represent truth values and can be either `true` or `false`. They are often used in conditional statements and loops.

### **Boolean Type**
| Type  | Description                     | Values                  |
|-------|---------------------------------|-------------------------|
| `bool` | Boolean                        | `true` or `false`      |

### **Example**
```rust
fn main() {
    let is_rust_fun: bool = true;
    let is_learning_easy: bool = false;
    println!("Is Rust fun? {}", is_rust_fun);
    println!("Is learning easy? {}", is_learning_easy);
}
```

#### **Output**
```
Is Rust fun? true
Is learning easy? false
```

---

## **4. Characters**

Characters represent a single Unicode scalar value. They are enclosed in single quotes (`'`).

### **Character Type**
| Type  | Description                     | Example                 |
|-------|---------------------------------|-------------------------|
| `char` | Single Unicode character       | `'R'`, `'😊'`           |

### **Example**
```rust
fn main() {
    let letter: char = 'R';
    let emoji: char = '😊';
    println!("letter = {}, emoji = {}", letter, emoji);
}
```

#### **Output**
```
letter = R, emoji = 😊
```

---

## **5. Putting It All Together**

Let’s write a program that uses all the basic data types:

```rust
fn main() {
    // Integers
    let a: i32 = 42;
    let b: u64 = 100;

    // Floating-point numbers
    let pi: f64 = 3.14159;
    let gravity: f32 = 9.81;

    // Booleans
    let is_rust_fun: bool = true;
    let is_learning_easy: bool = false;

    // Characters
    let letter: char = 'R';
    let emoji: char = '😊';

    // Print all values
    println!("a = {}, b = {}", a, b);
    println!("pi = {}, gravity = {}", pi, gravity);
    println!("Is Rust fun? {}", is_rust_fun);
    println!("Is learning easy? {}", is_learning_easy);
    println!("letter = {}, emoji = {}", letter, emoji);
}
```

#### **Output**
```
a = 42, b = 100
pi = 3.14159, gravity = 9.81
Is Rust fun? true
Is learning easy? false
letter = R, emoji = 😊
```

---

## **6. Practice Exercises**

1. **Integers**:
   - Declare two variables: one `i32` and one `u64`.
   - Perform arithmetic operations (addition, subtraction, multiplication) on them.

2. **Floating-Point Numbers**:
   - Declare two variables: one `f32` and one `f64`.
   - Perform division and multiplication on them.

3. **Booleans**:
   - Declare two boolean variables and use them in a conditional statement.

4. **Characters**:
   - Declare a `char` variable and print it along with a message.

---

## **7. Key Takeaways**
- **Integers**: Use `i32`, `u64`, etc., for whole numbers.
- **Floating-Point Numbers**: Use `f32` or `f64` for numbers with fractional parts.
- **Booleans**: Use `bool` for true/false values.
- **Characters**: Use `char` for single Unicode characters.

---
