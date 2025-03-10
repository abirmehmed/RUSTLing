# **Lesson 5: Control Flow - Conditional Statements**

Welcome to Lesson 5! Today, we’ll dive into **conditional statements** in Rust. Conditional statements allow your program to make decisions based on certain conditions. We’ll cover the `if` and `else` statements, as well as how to use `if` in a `let` statement. Let’s get started!

---

## **1. `if` and `else` Statements**

The `if` statement allows you to execute a block of code if a condition is true. You can also use `else` to execute a different block of code if the condition is false.

### **Syntax**
```rust
if condition {
    // Code to execute if the condition is true
} else {
    // Code to execute if the condition is false
}
```

### **Example**
```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }
}
```

#### **Output**
```
The number is greater than or equal to 5
```

---

### **`else if` for Multiple Conditions**

You can use `else if` to check multiple conditions.

#### **Example**
```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is exactly 5");
    } else {
        println!("The number is greater than 5");
    }
}
```

#### **Output**
```
The number is greater than 5
```

---

## **2. Using `if` in a `let` Statement**

In Rust, `if` is an expression, meaning it can return a value. This allows you to use `if` on the right-hand side of a `let` statement.

### **Syntax**
```rust
let variable_name = if condition {
    value_if_true
} else {
    value_if_false
};
```

### **Example**
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

#### **Output**
```
The value of number is: 5
```

---

## **3. Putting It All Together**

Let’s write a program that uses both `if/else` statements and `if` in a `let` statement:

```rust
fn main() {
    let number = 7;

    // Using if/else statements
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is exactly 5");
    } else {
        println!("The number is greater than 5");
    }

    // Using if in a let statement
    let description = if number < 5 {
        "less than 5"
    } else if number == 5 {
        "exactly 5"
    } else {
        "greater than 5"
    };

    println!("The number is {}", description);
}
```

#### **Output**
```
The number is greater than 5
The number is greater than 5
```

---

## **4. Key Points**
- **`if` Statements**: Execute code based on a condition.
- **`else` and `else if`**: Handle multiple conditions.
- **`if` in `let` Statements**: Use `if` as an expression to assign values.

---

## **5. Practice Exercises**

1. **Basic `if/else`**:
   - Write a program that checks if a number is even or odd using `if/else`.

2. **Multiple Conditions**:
   - Write a program that checks if a number is positive, negative, or zero using `if`, `else if`, and `else`.

3. **`if` in `let`**:
   - Write a program that assigns a value to a variable using `if` in a `let` statement. For example, assign `"even"` or `"odd"` based on whether a number is even or odd.

---

## **6. Solutions to Practice Exercises**

### **Exercise 1: Even or Odd**
```rust
fn main() {
    let number = 4;

    if number % 2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }
}
```

#### **Output**
```
The number is even
```

---

### **Exercise 2: Positive, Negative, or Zero**
```rust
fn main() {
    let number = -3;

    if number > 0 {
        println!("The number is positive");
    } else if number < 0 {
        println!("The number is negative");
    } else {
        println!("The number is zero");
    }
}
```

#### **Output**
```
The number is negative
```

---

### **Exercise 3: `if` in `let`**
```rust
fn main() {
    let number = 7;
    let description = if number % 2 == 0 { "even" } else { "odd" };

    println!("The number is {}", description);
}
```

#### **Output**
```
The number is odd
```

---
