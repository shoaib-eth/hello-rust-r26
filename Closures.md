# 🦀 Rust Closures – Complete Notes (With Ownership & Borrowing)

## What are Closures in Rust?
Closures are **anonymous functions** that you can:
- Store in variables
- Pass as arguments
- Return from functions

Closures are similar to functions, **but more powerful**.

Key difference:
> Closures can **capture variables from their surrounding environment**.

---

## Why Closures Exist
Closures are useful when you want:
- Short, inline logic
- Behavior as a value
- Functions that remember their environment

They are heavily used in:
- Iterators (`map`, `filter`, `for_each`)
- Callbacks
- Functional-style programming

---

## Basic Syntax of a Closure

```rust
|parameters| expression
```

Or with a block:

```rust
|parameters| {
    // logic
}
```

Example:
```rust
let add = |a: i32, b: i32| a + b;
```

---

## Example 1 – Simple Closure

```rust
fn main() {
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(5));
}
```

### Step-by-Step Explanation

- `|x: i32| x + 1` → defines a closure
- `add_one` stores the closure
- `add_one(5)` calls the closure like a function

Important notes:
- Closures can infer types, but here type is explicit
- This closure does NOT capture anything from outside

This behaves almost exactly like a normal function.

---

## Closures vs Functions

| Feature | Function | Closure |
|------|--------|--------|
| Name | Required | Anonymous |
| Capture environment | ❌ | ✅ |
| Type inference | ❌ | ✅ |
| Use as value | Limited | Easy |

---

## How Closures Capture Variables

Closures can capture variables in **three ways**:

1. By reference (`&T`)
2. By mutable reference (`&mut T`)
3. By value (move ownership)

Rust decides this **automatically** based on usage.

---

## Example 2 – Mutable Borrow Capture

```rust
let mut counter = 0;
let mut increase_counter = || {
    counter = counter + 1;
    println!("{}", counter);
};
increase_counter();  // 1
increase_counter();  // 2
increase_counter();  // 3
```

### What Is Happening Here?

- `counter` is defined outside the closure
- The closure **modifies** `counter`

This means:
> The closure captures `counter` as `&mut counter`

---

### Why `mut` Is Required Twice?

```rust
let mut counter = 0;
let mut increase_counter = || { ... };
```

- `counter` must be mutable (it is modified)
- `increase_counter` must be mutable because:
  - The closure holds a mutable borrow
  - Calling it changes its internal state

---

### Ownership & Borrowing Reminder

- While `increase_counter` exists:
  - No other mutable or immutable borrow of `counter` is allowed

This is Rust preventing **data races at compile time**.

---

## Closure Traits (`Fn`, `FnMut`, `FnOnce`)

Every closure implements **one of these traits**:

### 1. `Fn`
- Captures by immutable reference
- Can be called multiple times

### 2. `FnMut`
- Captures by mutable reference
- Can be called multiple times

### 3. `FnOnce`
- Takes ownership of captured values
- Can be called only once

---

## Example 3 – Borrowing, Not Ownership

```rust
fn main() {
    let x = String::from("Hello");
    let consume_and_return_x = || &x;
    println!("{}", x);
    let y = consume_and_return_x();  
    println!("{}", y);

    let z = y;
    println!("{}", z);
}
```

---

### Line-by-Line Explanation

```rust
let x = String::from("Hello");
```
- `x` owns the `String`

```rust
let consume_and_return_x = || &x;
```
- Closure returns `&String`
- Captures `x` as an **immutable reference**

Important:
> The name is misleading – nothing is consumed

---

```rust
println!("{}", x);
```
- Allowed because `x` is only immutably borrowed

---

```rust
let y = consume_and_return_x();
```
- `y` is `&String`
- Still no ownership transfer

---

```rust
let z = y;
```
- Copying a reference
- Both `y` and `z` point to the same `x`

Ownership remains with `x`.

---

## What If Ownership Was Moved?

```rust
let consume = move || x;
```

- `x` would be moved into the closure
- `x` cannot be used afterward
- Closure becomes `FnOnce`

---

## `move` Closures

The `move` keyword forces ownership capture.

```rust
let s = String::from("Hi");
let c = move || s;
```

Used when:
- Sending closures to threads
- Ensuring data lives long enough

---

## Where Closures Are Used in Real Code

### Iterators
```rust
let nums = vec![1, 2, 3];
nums.iter().for_each(|n| println!("{}", n));
```

### Filtering
```rust
let evens: Vec<_> = nums.into_iter().filter(|n| n % 2 == 0).collect();
```

### Callbacks
Closures are passed as behavior.

---

## Common Beginner Mistakes

- Forgetting `mut` on closure
- Confusing move vs borrow
- Assuming closure always takes ownership

---

## Mental Model (Very Important)

Think of closures as:
> Functions that **carry a backpack** of borrowed or owned variables

What’s inside the backpack depends on how you use them.

---

## Summary

- Closures are anonymous functions
- They can capture environment variables
- Capture can be by reference, mutable reference, or ownership
- Rust enforces safety using borrow rules
- `Fn`, `FnMut`, `FnOnce` describe closure behavior

---

## Interview Tip
"Closures in Rust are anonymous functions that can capture their environment safely using ownership and borrowing rules."

---

**End of Notes**