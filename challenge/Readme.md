Below is a set of **programming challenges** designed to test and enhance your skills in **Rust** and **system programming**. These challenges range from beginner to advanced levels, focusing on Rust’s ownership model, concurrency, file I/O, and low-level system interactions. Each challenge includes a problem statement, requirements, and expected output to guide implementation. They are crafted to align with the concepts discussed earlier (e.g., threads, `Option`, `Mutex`, system calls) and to deepen your understanding of Rust.

---

### Rust Programming Challenges

#### Challenge 1: File Line Counter (Beginner)
**Problem**: Write a Rust program that reads a text file and counts the number of lines, words, and characters (excluding whitespace). Handle errors gracefully.

**Requirements**:
- Use `std::fs` for file I/O.
- Return an `Option<(usize, usize, usize)>` for (lines, words, characters).
- Handle file-not-found errors using `Result`.
- Input: A file path (e.g., `input.txt`).
- Output: Print the counts or an error message.

**Example**:
```plaintext
// input.txt
Hello world
Rust is awesome
```
**Expected Output**:
```
Lines: 2, Words: 5, Characters: 23
```

---

#### Challenge 2: Threaded Sum Calculator (Intermediate)
**Problem**: Create a Rust program that splits a large vector of integers into chunks, computes the sum of each chunk in separate threads, and combines the results.

**Requirements**:
- Use `std::thread` for threading.
- Use `Arc` and `Mutex` for thread-safe sharing of the final sum.
- Input: A vector of integers (e.g., `vec![1, 2, 3, 4, 5, 6]`).
- Output: The total sum of all integers.
- Ensure no data races and proper thread synchronization.

**Example**:
```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
```
**Expected Output**:
```
Total sum: 21
```

---

#### Challenge 3: System Resource Monitor (Intermediate)
**Problem**: Build a Rust command-line tool that monitors and logs CPU usage periodically to a file, using the `sysinfo` crate.

**Requirements**:
- Use the `sysinfo` crate to access CPU usage.
- Log the CPU usage every 2 seconds to a file (e.g., `cpu_log.txt`) using `std::fs`.
- Use `std::thread::sleep` for timing.
- Handle errors with `Result`.
- Output: A log file with timestamped CPU usage entries.

**Example Output (cpu_log.txt)**:
```
2025-09-15 11:54:00: CPU Usage: 23.5%
2025-09-15 11:54:02: CPU Usage: 25.1%
```

---

#### Challenge 4: Safe File Downloader (Advanced)
**Problem**: Write a Rust program that downloads a file from a given URL and saves it to disk, handling errors and ensuring thread safety for concurrent downloads.

**Requirements**:
- Use the `reqwest` crate for HTTP requests (async mode with `tokio`).
- Save the file using `std::fs`.
- Allow multiple URLs to be downloaded concurrently using `tokio::task`.
- Use `Arc<Mutex<>>` to track download progress safely.
- Output: Save files to disk and print success/failure for each URL.

**Example**:
```rust
let urls = vec!["https://example.com/file1.txt", "https://example.com/file2.txt"];
```
**Expected Output**:
```
Downloaded file1.txt successfully
Failed to download file2.txt: Not Found
```

---

#### Challenge 5: Custom Memory Allocator (Advanced)
**Problem**: Implement a simple custom memory allocator in Rust that manages a fixed-size memory pool, allocating and deallocating blocks.

**Requirements**:
- Use `unsafe` Rust for low-level memory management.
- Implement `allocate(size: usize)` and `deallocate(ptr: *mut u8)` functions.
- Track free and allocated blocks using a `Vec` or linked list.
- Ensure memory safety where possible (e.g., bounds checking).
- Output: Demonstrate allocation and deallocation with a test case.

**Example**:
```rust
let mut allocator = CustomAllocator::new(1024); // 1KB pool
let ptr = allocator.allocate(128); // Allocate 128 bytes
allocator.deallocate(ptr);
```
**Expected Output**:
```
Allocated 128 bytes at <address>
Deallocated successfully
```

---

### Implementation Tips
- **Cargo Setup**: Initialize each project with `cargo new challenge_name` and add dependencies (e.g., `sysinfo`, `reqwest`, `tokio`) to `Cargo.toml`.
- **Error Handling**: Use `Result` and `Option` for robust error management.
- **Concurrency**: Leverage `Arc`, `Mutex`, or `tokio` for thread-safe operations.
- **Testing**: Write unit tests using `#[test]` to verify functionality.
- **Git Workflow**: Version control each challenge with `git init`, commit changes (`git commit -m "Implement challenge"`), and create branches for features.

---

### Example Starter Code (Challenge 1: File Line Counter)
```rust
use std::fs;

fn count_file_stats(path: &str) -> Result<Option<(usize, usize, usize)>, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().filter(|c| !c.is_whitespace()).count();
    Ok(Some((lines, words, chars)))
}

fn main() {
    match count_file_stats("input.txt") {
        Ok(Some((lines, words, chars))) => println!("Lines: {}, Words: {}, Characters: {}", lines, words, chars),
        Ok(None) => println!("Empty file"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

---

### Next Steps
- **Choose a Challenge**: Pick one or more challenges to implement. I can provide detailed solutions or hints for any of them.
- **Expand Difficulty**: Want harder challenges (e.g., building a kernel module or network server in Rust)?
- **Mock Exam**: Need another exam format with these challenges as scenario questions?
- **Specific Focus**: Want to dive deeper into one concept (e.g., `unsafe` Rust or async programming)?

Let me know your preference or if you want me to provide a solution for one of these challenges!