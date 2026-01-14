# 🦀 Rust Modules & `use` – Complete Notes (With File Structure & Examples)

## What are Modules in Rust? 📦
Modules are Rust’s way of **organizing code**.

Simple definition:
> A module is a **namespace** that groups related code together.

Modules help with:
- Readability 👀
- Reusability ♻️
- Encapsulation 🔐 (public vs private)

---

## Why Modules Are Needed 🤔
As projects grow:
- One file becomes too big
- Functions get hard to manage

Modules allow you to:
- Split code into logical parts
- Control what is visible outside
- Build clean project structures

---

## Basic Module (Same File)

```rust
// main.rs
fn add() {}
fn sub() {}

fn main() {
    add();
    sub();
}
```

Explanation:
- Functions live in the same file
- Rust finds them **within the current file first**

---

## Declaring a Module

```rust
mod math;
```

This line tells Rust:
> “There is a module named `math` – go find its code.”

Rust then searches in this order:
1. Current file
2. `math.rs` in the same folder
3. `math/mod.rs`

---

## Module in a Separate File (`math.rs`)

### File Structure
```text
src/
├── main.rs
└── math.rs
```

### `main.rs`
```rust
mod math;
use math::add;

fn main() {
    add();
    math::sub();
}
```

### `math.rs`
```rust
pub fn add() {
    println!("{}", 5 + 6);
}

pub fn sub() {
    println!("{}", 6 - 5);
}
```

Key points:
- `mod math;` links the file
- `pub` makes functions accessible
- `use` brings names into scope

---

## `use` Keyword – Bringing Names Into Scope 🧲

```rust
use math::add;
```

Meaning:
- You can call `add()` directly
- Without `use`, you must write `math::add()`

---

## Using `use *` (Glob Import) ⚠️

```rust
use math::*;
```

This imports **everything public** from `math`.

Pros:
- Shorter code

Cons:
- Can reduce clarity
- Name collisions possible

Recommended:
> Prefer explicit imports in large projects.

---

## Module as a Folder (`mod.rs` Pattern)

### File Structure
```text
src/
├── main.rs
└── math/
    ├── mod.rs
    ├── add.rs
    └── sub.rs
```

---

### `main.rs`
```rust
mod math;
use math::*;

fn main() {
    add();
    sub();
}
```

---

### `math/mod.rs`
```rust
mod add;
mod sub;

pub use add::*;
pub use sub::*;
```

Explanation:
- Declares submodules
- Re-exports functions using `pub use`

---

### `add.rs`
```rust
pub fn add() {}
```

### `sub.rs`
```rust
pub fn sub() {}
```

This pattern is useful for **large modules**.

---

## Visibility Rules (`pub`) 🔐

By default:
- Everything is **private**

To expose items:
```rust
pub fn add() {}
```

Visibility rule:
> Parents cannot access private children.

---

## `crate` Keyword – Absolute Path 📍

`crate` refers to the **root of the current crate**.

Example:
```rust
use crate::add;
```

This avoids ambiguity and improves clarity.

---

## Example Using `crate`

### `main.rs`
```rust
mod add;
mod math;
use math::*;

fn main() {
    let a: u8 = 5;
    let b: u8 = 6;
    add_u8(a, b);
}
```

### `math.rs`
```rust
use crate::add;

pub fn add_u8(x: u8, y: u8) {
    add::add_num(x, y);
}
```

### `add.rs`
```rust
pub fn add_num(a: u8, b: u8) {
    println!("Result is {}", a + b);
}
```

---

## How Rust Finds Modules (Compiler Search Order) 🔍

When you write:
```rust
mod math;
```

Rust searches:
1. Same file
2. `math.rs`
3. `math/mod.rs`

This is exactly how the slides explain module resolution.

---

## Common Beginner Mistakes 🚨

- Forgetting `mod xyz;`
- Forgetting `pub`
- Confusing `use` with `mod`
- Expecting Rust to auto-detect files

---

## Mental Model 🧠

Think like this:

- `mod` → declares a module
- `use` → imports names
- `pub` → exposes items
- `crate` → root of project

---

## Summary ✨

- Modules organize code
- `mod` declares modules
- `use` brings names into scope
- `pub` controls visibility
- Rust has a strict module resolution system

---

## Interview Tip 💼

"Rust modules provide structured code organization and fine-grained visibility control using `mod`, `use`, `pub`, and the crate system."

---

**End of Notes