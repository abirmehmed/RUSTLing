# **Day 3: Control Flow in Rust**

Welcome to Day 3 of your Rust journey! Today, we’ll explore **control flow**, which allows you to make decisions and repeat actions in your programs. Specifically, we’ll cover **conditional statements** (`if/else`) and **loops** (`loop`, `while`). These are fundamental constructs that enable you to write dynamic and flexible programs.

---

## **1. Conditional Statements (`if/else`)**

Conditional statements allow your program to make decisions based on certain conditions. In Rust, the primary conditional statement is `if/else`.

### **Syntax**
```rust
if condition {
    // Code to execute if the condition is true
} else if another_condition {
    // Code to execute if another_condition is true
} else {
    // Code to execute if none of the conditions are true
}
```

### **Key Points**
- The `condition` must evaluate to a boolean (`true` or `false`).
- The `else if` and `else` blocks are optional.
- Rust does not require parentheses around the condition, but curly braces `{}` are mandatory.

### **Example**
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

### **Using `if` in a `let` Statement**
In Rust, `if` is an expression, meaning it can return a value. This allows you to use `if` on the right-hand side of a `let` statement.

#### **Example**
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

## **2. Loops**

Loops allow you to execute a block of code repeatedly. Rust provides three types of loops:
1. `loop`: Infinite loop until explicitly stopped.
2. `while`: Loop while a condition is true.
3. `for`: Iterate over a collection (covered in a later lesson).

---

### **2.1. `loop`**
The `loop` keyword creates an infinite loop. You can exit the loop using the `break` keyword.

#### **Syntax**
```rust
loop {
    // Code to execute repeatedly
    if condition {
        break; // Exit the loop
    }
}
```

#### **Example**
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

### **2.2. `while` Loop**
The `while` loop repeats a block of code as long as a condition is true.

#### **Syntax**
```rust
while condition {
    // Code to execute while the condition is true
}
```

#### **Example**
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

### **2.3. Returning Values from Loops**
You can use the `break` keyword to return a value from a loop.

#### **Example**
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

## **3. Putting It All Together**

Let’s write a program that uses both conditional statements and loops to simulate a simple guessing game:

```rust
use std::io;

fn main() {
    let secret_number = 42;
    let mut guess = String::new();

    loop {
        println!("Guess the number (between 1 and 100):");

        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }
}
```

---

## **4. Key Takeaways**
- **Conditional Statements**: Use `if/else` to make decisions based on conditions.
- **Loops**:
  - Use `loop` for infinite loops and `break` to exit.
  - Use `while` to loop while a condition is true.
- **Returning Values**: You can return values from loops using `break`.

---

## **5. Practice Exercises**
1. Write a program that prints all even numbers between 1 and 20 using a `while` loop.
2. Write a program that calculates the factorial of a number using a `loop`.
3. Modify the guessing game to limit the number of attempts to 5.

---
