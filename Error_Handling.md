# 🦀 Rust Error Handling – Complete Notes

## What is Error Handling in Rust?
Error handling is the mechanism used to **detect, represent, and respond to errors** during program execution.

Rust treats errors as **normal values**, not exceptions.
This design forces developers to handle failures explicitly and safely.

In simple terms:
- Rust does NOT have try-catch exceptions
- Errors are handled using types and pattern matching

---

## Why Error Handling is Important
Error handling helps to:
- Prevent program crashes
- Avoid undefined behavior
- Make programs more reliable and predictable
- Handle failure cases explicitly

Rust’s philosophy:
> "Make invalid states unrepresentable"

---

## Two Main Types of Errors in Rust
Rust categorizes errors into **two types**:

1. **Recoverable Errors**
2. **Unrecoverable Errors**

---

## 1. Recoverable Errors
Recoverable errors are errors from which the program can **gracefully recover**.

Examples:
- File not found
- Invalid user input
- Network request failed

Rust represents recoverable errors using the `Result<T, E>` type.

---

## The `Result<T, E>` Enum

Definition:
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Meaning:
- `Ok(T)` → Operation succeeded
- `Err(E)` → Operation failed

---

## Basic `Result` Example

```rust
use std::fs::File;

fn main() {
    let file = File::open("data.txt");

    match file {
        Ok(f) => println!("File opened successfully"),
        Err(e) => println!("Error opening file: {}", e),
    }
}
```

Here:
- The program does not crash
- Both success and failure are handled explicitly

---

## Handling Errors Using `match`
`match` is the most explicit way to handle errors.

Advantages:
- Full control over error handling
- Clear logic

Disadvantages:
- Verbose

---

## Using `unwrap()`
`unwrap()` extracts the value inside `Ok`.

```rust
let file = File::open("data.txt").unwrap();
```

Behavior:
- Works if result is `Ok`
- Panics if result is `Err`

Use only when:
- You are 100% sure the operation cannot fail

---

## Using `expect()`
Similar to `unwrap()`, but allows a custom error message.

```rust
let file = File::open("data.txt").expect("Failed to open file");
```

Preferred over `unwrap()` because it gives context.

---

## 2. Unrecoverable Errors
Unrecoverable errors are errors that **cannot be handled safely**.

Examples:
- Index out of bounds
- Integer overflow (in debug mode)
- Logic bugs

Rust handles unrecoverable errors using `panic!`.

---

## The `panic!` Macro

```rust
panic!("Something went terribly wrong!");
```

Behavior:
- Program prints error message
- Stack is unwound
- Program exits

---

## Panic Example

```rust
fn main() {
    let v = vec![1, 2, 3];
    println!("{}", v[99]);
}
```

This causes a panic due to invalid index access.

---

## Error Propagation Using `?`
The `?` operator simplifies error handling.

Instead of handling errors immediately, it **propagates** them to the caller.

---

## Example Without `?`

```rust
fn read_file() -> Result<String, std::io::Error> {
    let file = std::fs::read_to_string("data.txt");

    match file {
        Ok(content) => Ok(content),
        Err(e) => Err(e),
    }
}
```

---

## Same Example With `?`

```rust
fn read_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("data.txt")?;
    Ok(content)
}
```

Benefits:
- Cleaner code
- Less boilerplate

---

## Rules of `?` Operator
- Can only be used in functions returning `Result` or `Option`
- Automatically returns the error to the caller

---

## The `Option<T>` Type
Used when a value **may or may not exist**, but it is not an error.

Definition:
```rust
enum Option<T> {
    Some(T),
    None,
}
```

---

## `Option` Example

```rust
fn find_even(n: i32) -> Option<i32> {
    if n % 2 == 0 {
        Some(n)
    } else {
        None
    }
}
```

Here:
- `None` means absence of value, not failure

---

## Converting `Option` to `Result`

```rust
fn get_value(opt: Option<i32>) -> Result<i32, String> {
    opt.ok_or("Value not found".to_string())
}
```

---

## Custom Error Types
Rust allows defining custom error types using `enum`.

```rust
enum AppError {
    NotFound,
    PermissionDenied,
}
```

These can be used with `Result<T, AppError>`.

---

## Best Practices
- Use `Result` for recoverable errors
- Use `panic!` only for bugs
- Prefer `?` over `match` when possible
- Avoid `unwrap()` in production code

---

## Summary
- Rust has no exceptions
- Errors are values
- `Result` handles recoverable errors
- `panic!` handles unrecoverable errors
- `?` simplifies error propagation
- Explicit handling leads to safer programs

---

## Interview Tip
"Rust enforces explicit error handling using Result and Option, eliminating hidden runtime exceptions."

---

**End of Notes**

