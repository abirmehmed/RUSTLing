# **Lesson 9: Practice Problems on Basics**

Welcome to Lesson 9! Today, we’ll solve **practice problems** that involve **variables**, **control flow**, and **functions**. These problems will help you solidify your understanding of the basics of Rust. Let’s dive in!

---

## **Problem 1: Even or Odd**

### **Problem Statement**
Write a function called `is_even` that takes an integer as input and returns `true` if the number is even and `false` if it’s odd.

### **Solution**
```rust
fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn main() {
    let number = 7;
    if is_even(number) {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
```

#### **Output**
```
7 is odd
```

---

## **Problem 2: Find the Maximum**

### **Problem Statement**
Write a function called `find_max` that takes three integers as input and returns the largest one.

### **Solution**
```rust
fn find_max(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        a
    } else if b >= a && b >= c {
        b
    } else {
        c
    }
}

fn main() {
    let a = 10;
    let b = 20;
    let c = 15;
    println!("The maximum is: {}", find_max(a, b, c));
}
```

#### **Output**
```
The maximum is: 20
```

---

## **Problem 3: Factorial**

### **Problem Statement**
Write a function called `factorial` that takes an integer `n` and returns its factorial (i.e., `n! = n * (n-1) * ... * 1`).

### **Solution**
```rust
fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    let n = 5;
    println!("{}! = {}", n, factorial(n));
}
```

#### **Output**
```
5! = 120
```

---

## **Problem 4: Fibonacci Sequence**

### **Problem Statement**
Write a function called `fibonacci` that takes an integer `n` and returns the `n`-th number in the Fibonacci sequence. The Fibonacci sequence is defined as:
- `fib(0) = 0`
- `fib(1) = 1`
- `fib(n) = fib(n-1) + fib(n-2)` for `n > 1`

### **Solution**
```rust
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n = 6;
    println!("fib({}) = {}", n, fibonacci(n));
}
```

#### **Output**
```
fib(6) = 8
```

---

## **Problem 5: Sum of Digits**

### **Problem Statement**
Write a function called `sum_of_digits` that takes an integer and returns the sum of its digits.

### **Solution**
```rust
fn sum_of_digits(mut num: i32) -> i32 {
    let mut sum = 0;
    while num != 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

fn main() {
    let num = 12345;
    println!("Sum of digits of {} is: {}", num, sum_of_digits(num));
}
```

#### **Output**
```
Sum of digits of 12345 is: 15
```

---

## **Problem 6: Prime Number Check**

### **Problem Statement**
Write a function called `is_prime` that takes an integer and returns `true` if it’s a prime number and `false` otherwise.

### **Solution**
```rust
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let num = 29;
    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
```

#### **Output**
```
29 is a prime number
```

---

## **Problem 7: Reverse a String**

### **Problem Statement**
Write a function called `reverse_string` that takes a string slice (`&str`) and returns the reversed string.

### **Solution**
```rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s = "hello";
    println!("Reversed string: {}", reverse_string(s));
}
```

#### **Output**
```
Reversed string: olleh
```

---

## **Problem 8: Count Vowels**

### **Problem Statement**
Write a function called `count_vowels` that takes a string slice (`&str`) and returns the number of vowels (a, e, i, o, u) in the string.

### **Solution**
```rust
fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

fn main() {
    let s = "Hello, world!";
    println!("Number of vowels: {}", count_vowels(s));
}
```

#### **Output**
```
Number of vowels: 3
```

---

## **Problem 9: Palindrome Check**

### **Problem Statement**
Write a function called `is_palindrome` that takes a string slice (`&str`) and returns `true` if the string is a palindrome (reads the same backward as forward) and `false` otherwise.

### **Solution**
```rust
fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase(); // Case-insensitive check
    s.chars().eq(s.chars().rev())
}

fn main() {
    let s = "racecar";
    if is_palindrome(s) {
        println!("{} is a palindrome", s);
    } else {
        println!("{} is not a palindrome", s);
    }
}
```

#### **Output**
```
racecar is a palindrome
```

---

## **Problem 10: FizzBuzz**

### **Problem Statement**
Write a function called `fizzbuzz` that takes an integer `n` and prints the numbers from 1 to `n`. For multiples of 3, print "Fizz" instead of the number. For multiples of 5, print "Buzz". For multiples of both 3 and 5, print "FizzBuzz".

### **Solution**
```rust
fn fizzbuzz(n: u32) {
    for i in 1..=n {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

fn main() {
    fizzbuzz(15);
}
```

#### **Output**
```
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

---


