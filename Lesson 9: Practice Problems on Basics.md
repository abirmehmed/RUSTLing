# **Lesson 9: Practice Problems on Basics**

Welcome to Lesson 9! Today, we’ll solve **practice problems** that involve **variables**, **control flow**, and **functions**. These problems will help you solidify your understanding of the basics of Rust. Let’s dive in!

Sure! Here’s a **list of all 20 problems** from **Problem 1** to **Problem 20**, along with their descriptions:

---

### **Problem 1: Even or Odd**
Write a function called `is_even` that takes an integer as input and returns `true` if the number is even and `false` if it’s odd.

---

### **Problem 2: Find the Maximum**
Write a function called `find_max` that takes three integers as input and returns the largest one.

---

### **Problem 3: Factorial**
Write a function called `factorial` that takes an integer `n` and returns its factorial (i.e., `n! = n * (n-1) * ... * 1`).

---

### **Problem 4: Fibonacci Sequence**
Write a function called `fibonacci` that takes an integer `n` and returns the `n`-th number in the Fibonacci sequence.

---

### **Problem 5: Sum of Digits**
Write a function called `sum_of_digits` that takes an integer and returns the sum of its digits.

---

### **Problem 6: Prime Number Check**
Write a function called `is_prime` that takes an integer and returns `true` if it’s a prime number and `false` otherwise.

---

### **Problem 7: Reverse a String**
Write a function called `reverse_string` that takes a string slice (`&str`) and returns the reversed string.

---

### **Problem 8: Count Vowels**
Write a function called `count_vowels` that takes a string slice (`&str`) and returns the number of vowels (a, e, i, o, u) in the string.

---

### **Problem 9: Palindrome Check**
Write a function called `is_palindrome` that takes a string slice (`&str`) and returns `true` if the string is a palindrome (reads the same backward as forward) and `false` otherwise.

---

### **Problem 10: FizzBuzz**
Write a function called `fizzbuzz` that takes an integer `n` and prints the numbers from 1 to `n`. For multiples of 3, print "Fizz" instead of the number. For multiples of 5, print "Buzz". For multiples of both 3 and 5, print "FizzBuzz".

---

### **Problem 11: Find the Minimum**
Write a function called `find_min` that takes three integers as input and returns the smallest one.

---

### **Problem 12: Check Leap Year**
Write a function called `is_leap_year` that takes a year as input and returns `true` if it’s a leap year and `false` otherwise.

---

### **Problem 13: Calculate Power**
Write a function called `power` that takes two integers, `base` and `exponent`, and returns `base` raised to the power of `exponent`.

---

### **Problem 14: Count Words in a String**
Write a function called `count_words` that takes a string slice (`&str`) and returns the number of words in the string.

---

### **Problem 15: Check if a Number is Palindrome**
Write a function called `is_number_palindrome` that takes an integer and returns `true` if the number is a palindrome (reads the same backward as forward) and `false` otherwise.

---

### **Problem 16: Find the GCD**
Write a function called `gcd` that takes two integers and returns their greatest common divisor (GCD).

---

### **Problem 17: Check if a String Contains a Substring**
Write a function called `contains_substring` that takes two string slices (`&str`) and returns `true` if the second string is a substring of the first string, and `false` otherwise.

---

### **Problem 18: Find the Length of the Longest Word**
Write a function called `longest_word_length` that takes a string slice (`&str`) and returns the length of the longest word in the string.

---

### **Problem 19: Check if a Number is Armstrong**
Write a function called `is_armstrong` that takes an integer and returns `true` if it’s an Armstrong number (a number that is equal to the sum of its own digits each raised to the power of the number of digits) and `false` otherwise.

---

### **Problem 20: Generate Fibonacci Sequence Up to N**
Write a function called `fibonacci_sequence` that takes an integer `n` and returns a vector containing the Fibonacci sequence up to `n`.

---

# **Solutions**

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


---

## **Problem 11: Find the Minimum**

### **Problem Statement**
Write a function called `find_min` that takes three integers as input and returns the smallest one.

### **Solution**
```rust
fn find_min(a: i32, b: i32, c: i32) -> i32 {
    if a <= b && a <= c {
        a
    } else if b <= a && b <= c {
        b
    } else {
        c
    }
}

fn main() {
    let a = 10;
    let b = 20;
    let c = 15;
    println!("The minimum is: {}", find_min(a, b, c));
}
```

#### **Output**
```
The minimum is: 10
```

---

## **Problem 12: Check Leap Year**

### **Problem Statement**
Write a function called `is_leap_year` that takes a year as input and returns `true` if it’s a leap year and `false` otherwise. A leap year is divisible by 4 but not by 100, unless it’s also divisible by 400.

### **Solution**
```rust
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn main() {
    let year = 2024;
    if is_leap_year(year) {
        println!("{} is a leap year", year);
    } else {
        println!("{} is not a leap year", year);
    }
}
```

#### **Output**
```
2024 is a leap year
```

---

## **Problem 13: Calculate Power**

### **Problem Statement**
Write a function called `power` that takes two integers, `base` and `exponent`, and returns `base` raised to the power of `exponent`.

### **Solution**
```rust
fn power(base: i32, exponent: u32) -> i32 {
    let mut result = 1;
    for _ in 0..exponent {
        result *= base;
    }
    result
}

fn main() {
    let base = 2;
    let exponent = 3;
    println!("{}^{} = {}", base, exponent, power(base, exponent));
}
```

#### **Output**
```
2^3 = 8
```

---

## **Problem 14: Count Words in a String**

### **Problem Statement**
Write a function called `count_words` that takes a string slice (`&str`) and returns the number of words in the string.

### **Solution**
```rust
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn main() {
    let s = "Hello, world! This is Rust.";
    println!("Number of words: {}", count_words(s));
}
```

#### **Output**
```
Number of words: 5
```

---

## **Problem 15: Check if a Number is Palindrome**

### **Problem Statement**
Write a function called `is_number_palindrome` that takes an integer and returns `true` if the number is a palindrome (reads the same backward as forward) and `false` otherwise.

### **Solution**
```rust
fn is_number_palindrome(mut num: i32) -> bool {
    let original = num;
    let mut reversed = 0;
    while num != 0 {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }
    original == reversed
}

fn main() {
    let num = 12321;
    if is_number_palindrome(num) {
        println!("{} is a palindrome", num);
    } else {
        println!("{} is not a palindrome", num);
    }
}
```

#### **Output**
```
12321 is a palindrome
```

---

## **Problem 16: Find the GCD**

### **Problem Statement**
Write a function called `gcd` that takes two integers and returns their greatest common divisor (GCD).

### **Solution**
```rust
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let a = 56;
    let b = 98;
    println!("GCD of {} and {} is: {}", a, b, gcd(a, b));
}
```

#### **Output**
```
GCD of 56 and 98 is: 14
```

---

## **Problem 17: Check if a String Contains a Substring**

### **Problem Statement**
Write a function called `contains_substring` that takes two string slices (`&str`) and returns `true` if the second string is a substring of the first string, and `false` otherwise.

### **Solution**
```rust
fn contains_substring(s: &str, sub: &str) -> bool {
    s.contains(sub)
}

fn main() {
    let s = "Hello, world!";
    let sub = "world";
    if contains_substring(s, sub) {
        println!("'{}' contains '{}'", s, sub);
    } else {
        println!("'{}' does not contain '{}'", s, sub);
    }
}
```

#### **Output**
```
'Hello, world!' contains 'world'
```

---

## **Problem 18: Find the Length of the Longest Word**

### **Problem Statement**
Write a function called `longest_word_length` that takes a string slice (`&str`) and returns the length of the longest word in the string.

### **Solution**
```rust
fn longest_word_length(s: &str) -> usize {
    s.split_whitespace().map(|word| word.len()).max().unwrap_or(0)
}

fn main() {
    let s = "Hello, world! This is Rust.";
    println!("Length of the longest word: {}", longest_word_length(s));
}
```

#### **Output**
```
Length of the longest word: 6
```

---

## **Problem 19: Check if a Number is Armstrong**

### **Problem Statement**
Write a function called `is_armstrong` that takes an integer and returns `true` if it’s an Armstrong number (a number that is equal to the sum of its own digits each raised to the power of the number of digits) and `false` otherwise.

### **Solution**
```rust
fn is_armstrong(mut num: u32) -> bool {
    let original = num;
    let num_digits = num.to_string().len() as u32;
    let mut sum = 0;
    while num != 0 {
        let digit = num % 10;
        sum += digit.pow(num_digits);
        num /= 10;
    }
    original == sum
}

fn main() {
    let num = 153;
    if is_armstrong(num) {
        println!("{} is an Armstrong number", num);
    } else {
        println!("{} is not an Armstrong number", num);
    }
}
```

#### **Output**
```
153 is an Armstrong number
```

---

## **Problem 20: Generate Fibonacci Sequence Up to N**

### **Problem Statement**
Write a function called `fibonacci_sequence` that takes an integer `n` and returns a vector containing the Fibonacci sequence up to `n`.

### **Solution**
```rust
fn fibonacci_sequence(n: u32) -> Vec<u32> {
    let mut sequence = Vec::new();
    let mut a = 0;
    let mut b = 1;
    while a <= n {
        sequence.push(a);
        let temp = a;
        a = b;
        b = temp + b;
    }
    sequence
}

fn main() {
    let n = 20;
    println!("Fibonacci sequence up to {}: {:?}", n, fibonacci_sequence(n));
}
```

#### **Output**
```
Fibonacci sequence up to 20: [0, 1, 1, 2, 3, 5, 8, 13]
```

---


