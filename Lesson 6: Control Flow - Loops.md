# **Lesson 6: Control Flow - Loops**

Welcome to Lesson 6! Today, we’ll explore **loops** in Rust. Loops allow you to execute a block of code repeatedly. We’ll cover three types of loops: `loop`, `while`, and how to **return values from loops**. Let’s dive in!

---

## **1. `loop` (Infinite Loops)**

The `loop` keyword creates an **infinite loop**. It will keep running until you explicitly stop it using the `break` keyword.

### **Syntax**
```rust
loop {
    // Code to execute repeatedly
    if condition {
        break; // Exit the loop
    }
}
```

### **Example**
```rust
fn main() {
    let mut count = 0;

    loop {
        println!("Count: {}", count);
        count += 1;

        if count == 5 {
            break; // Exit the loop when count reaches 5
        }
    }
}
```

#### **Output**
```
Count: 0
Count: 1
Count: 2
Count: 3
Count: 4
```

---

## **2. `while` Loops**

The `while` loop repeats a block of code as long as a condition is true.

### **Syntax**
```rust
while condition {
    // Code to execute while the condition is true
}
```

### **Example**
```rust
fn main() {
    let mut count = 0;

    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
}
```

#### **Output**
```
Count: 0
Count: 1
Count: 2
Count: 3
Count: 4
```

---

## **3. Returning Values from Loops**

In Rust, you can use the `break` keyword to **return a value** from a loop. This is useful when you want to compute a result inside a loop and return it.

### **Syntax**
```rust
let result = loop {
    // Code to execute
    if condition {
        break value; // Return a value from the loop
    }
};
```

### **Example**
```rust
fn main() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 5 {
            break count * 2; // Return a value from the loop
        }
    };

    println!("The result is: {}", result);
}
```

#### **Output**
```
The result is: 10
```

---

## **4. Putting It All Together**

Let’s write a program that uses all three types of loops:

```rust
fn main() {
    // Example of a loop
    let mut count = 0;

    loop {
        println!("Loop: Count = {}", count);
        count += 1;

        if count == 3 {
            break; // Exit the loop
        }
    }

    // Example of a while loop
    let mut number = 3;

    while number != 0 {
        println!("While: Number = {}", number);
        number -= 1;
    }

    // Example of returning a value from a loop
    let result = loop {
        count += 1;

        if count == 5 {
            break count * 2; // Return a value from the loop
        }
    };

    println!("The result is: {}", result);
}
```

#### **Output**
```
Loop: Count = 0
Loop: Count = 1
Loop: Count = 2
While: Number = 3
While: Number = 2
While: Number = 1
The result is: 10
```

---

## **5. Key Points**
- **`loop`**: Creates an infinite loop. Use `break` to exit.
- **`while`**: Repeats a block of code while a condition is true.
- **Returning Values**: Use `break value` to return a value from a loop.

---

## **6. Practice Exercises**

1. **`loop`**:
   - Write a program that uses a `loop` to print numbers from 1 to 5.

2. **`while`**:
   - Write a program that uses a `while` loop to print numbers from 5 to 1.

3. **Returning Values**:
   - Write a program that uses a `loop` to calculate the sum of numbers from 1 to 10 and returns the result.

---

## **7. Solutions to Practice Exercises**

### **Exercise 1: `loop`**
```rust
fn main() {
    let mut count = 1;

    loop {
        println!("Count: {}", count);
        count += 1;

        if count > 5 {
            break;
        }
    }
}
```

#### **Output**
```
Count: 1
Count: 2
Count: 3
Count: 4
Count: 5
```

---

### **Exercise 2: `while`**
```rust
fn main() {
    let mut number = 5;

    while number > 0 {
        println!("Number: {}", number);
        number -= 1;
    }
}
```

#### **Output**
```
Number: 5
Number: 4
Number: 3
Number: 2
Number: 1
```

---

### **Exercise 3: Returning Values**
```rust
fn main() {
    let mut sum = 0;
    let mut count = 1;

    let result = loop {
        sum += count;
        count += 1;

        if count > 10 {
            break sum; // Return the sum
        }
    };

    println!("The sum is: {}", result);
}
```

#### **Output**
```
The sum is: 55
```

---

