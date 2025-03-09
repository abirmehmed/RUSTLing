# **Lesson 4: Basic Syntax - Operators**

Welcome to Lesson 4! Today, we’ll explore **operators** in Rust. Operators are symbols that perform operations on variables and values. We’ll cover three main types of operators: **arithmetic**, **comparison**, and **logical**. Let’s dive in!

---

## **1. Arithmetic Operators**

Arithmetic operators are used to perform basic mathematical operations like addition, subtraction, multiplication, and division.

### **Arithmetic Operators in Rust**
| Operator | Description          | Example       |
|----------|----------------------|---------------|
| `+`      | Addition             | `5 + 3 = 8`   |
| `-`      | Subtraction          | `5 - 3 = 2`   |
| `*`      | Multiplication       | `5 * 3 = 15`  |
| `/`      | Division             | `10 / 2 = 5`  |
| `%`      | Modulus (Remainder)  | `10 % 3 = 1`  |

### **Example**
```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("a + b = {}", a + b); // 13
    println!("a - b = {}", a - b); // 7
    println!("a * b = {}", a * b); // 30
    println!("a / b = {}", a / b); // 3 (integer division)
    println!("a % b = {}", a % b); // 1
}
```

#### **Output**
```
a + b = 13
a - b = 7
a * b = 30
a / b = 3
a % b = 1
```

---

## **2. Comparison Operators**

Comparison operators are used to compare two values. They return a boolean value (`true` or `false`).

### **Comparison Operators in Rust**
| Operator | Description              | Example       |
|----------|--------------------------|---------------|
| `==`     | Equal to                 | `5 == 5` → `true` |
| `!=`     | Not equal to             | `5 != 3` → `true` |
| `>`      | Greater than             | `5 > 3` → `true`  |
| `<`      | Less than                | `5 < 3` → `false` |
| `>=`     | Greater than or equal to | `5 >= 5` → `true` |
| `<=`     | Less than or equal to    | `5 <= 3` → `false`|

### **Example**
```rust
fn main() {
    let x = 10;
    let y = 20;

    println!("x == y: {}", x == y); // false
    println!("x != y: {}", x != y); // true
    println!("x > y: {}", x > y);   // false
    println!("x < y: {}", x < y);   // true
    println!("x >= y: {}", x >= y); // false
    println!("x <= y: {}", x <= y); // true
}
```

#### **Output**
```
x == y: false
x != y: true
x > y: false
x < y: true
x >= y: false
x <= y: true
```

---

## **3. Logical Operators**

Logical operators are used to combine multiple conditions. They also return a boolean value (`true` or `false`).

### **Logical Operators in Rust**
| Operator | Description          | Example                     |
|----------|----------------------|-----------------------------|
| `&&`     | Logical AND          | `true && false` → `false`   |
| `||`     | Logical OR           | `true || false` → `true`    |
| `!`      | Logical NOT          | `!true` → `false`           |

### **Example**
```rust
fn main() {
    let is_rust_fun = true;
    let is_learning_easy = false;

    println!("Rust is fun AND learning is easy: {}", is_rust_fun && is_learning_easy); // false
    println!("Rust is fun OR learning is easy: {}", is_rust_fun || is_learning_easy); // true
    println!("Rust is NOT fun: {}", !is_rust_fun); // false
}
```

#### **Output**
```
Rust is fun AND learning is easy: false
Rust is fun OR learning is easy: true
Rust is NOT fun: false
```

---

## **4. Putting It All Together**

Let’s write a program that uses all three types of operators:

```rust
fn main() {
    // Arithmetic operators
    let a = 10;
    let b = 3;
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    println!("a % b = {}", a % b);

    // Comparison operators
    let x = 10;
    let y = 20;
    println!("x == y: {}", x == y);
    println!("x != y: {}", x != y);
    println!("x > y: {}", x > y);
    println!("x < y: {}", x < y);
    println!("x >= y: {}", x >= y);
    println!("x <= y: {}", x <= y);

    // Logical operators
    let is_rust_fun = true;
    let is_learning_easy = false;
    println!("Rust is fun AND learning is easy: {}", is_rust_fun && is_learning_easy);
    println!("Rust is fun OR learning is easy: {}", is_rust_fun || is_learning_easy);
    println!("Rust is NOT fun: {}", !is_rust_fun);
}
```

#### **Output**
```
a + b = 13
a - b = 7
a * b = 30
a / b = 3
a % b = 1
x == y: false
x != y: true
x > y: false
x < y: true
x >= y: false
x <= y: true
Rust is fun AND learning is easy: false
Rust is fun OR learning is easy: true
Rust is NOT fun: false
```

---

## **5. Practice Exercises**

1. **Arithmetic Operators**:
   - Declare two variables `a` and `b` with values `15` and `4`.
   - Perform all arithmetic operations on them and print the results.

2. **Comparison Operators**:
   - Declare two variables `x` and `y` with values `25` and `30`.
   - Compare them using all comparison operators and print the results.

3. **Logical Operators**:
   - Declare two boolean variables `p` and `q` with values `true` and `false`.
   - Combine them using logical operators and print the results.

---

## **6. Key Takeaways**
- **Arithmetic Operators**: Perform basic math operations (`+`, `-`, `*`, `/`, `%`).
- **Comparison Operators**: Compare two values (`==`, `!=`, `>`, `<`, `>=`, `<=`).
- **Logical Operators**: Combine conditions (`&&`, `||`, `!`).

---
