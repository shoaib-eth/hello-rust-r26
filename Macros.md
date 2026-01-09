# 🦀 Rust Macros – Complete Notes

## What are Macros in Rust?
Macros are a **metaprogramming feature** in Rust.
They allow you to write **code that generates other code** at **compile time**.

- Macros expand before the program is compiled
- Functions run at runtime, macros expand at compile time

In simple terms:
- **Functions work on values**
- **Macros work on Rust code itself**

---

## Why Do Macros Exist?
Macros are useful when:
- You want to avoid repeating similar code
- You need a variable number of arguments
- You want zero runtime overhead
- You want to generate code automatically

Think of macros as a **code template system** that the compiler fills in.

---

## Macro vs Function

| Feature | Function | Macro |
|------|--------|-------|
| Execution time | Runtime | Compile time |
| Arguments | Typed values | Token trees |
| Flexibility | Limited | Very high |
| Overhead | Possible | Zero-cost |
| Syntax | `func()` | `macro!()` |

---

## Types of Macros in Rust
Rust has **three main kinds of macros**:

1. Declarative macros (`macro_rules!`)
2. Procedural macros
3. Built-in macros

---

## 1. Declarative Macros (`macro_rules!`)
These are the most commonly used macros in Rust.
They work using **pattern matching**.

Example:
```rust
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

fn main() {
    say_hello!();
}
```

Important points:
- `say_hello!()` is NOT a function call
- The macro expands into valid Rust code during compilation

---

## How `macro_rules!` Pattern Matching Works

```rust
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    let result = add!(2, 3);
    println!("{}", result);
}
```

Here:
- `$a` and `$b` are macro variables
- `expr` means the macro expects expressions

Common macro matchers:

| Matcher | Meaning |
|------|--------|
| `expr` | Expression |
| `ident` | Identifier |
| `ty` | Type |
| `pat` | Pattern |
| `stmt` | Statement |
| `block` | Code block |

---

## Macro Hygiene
Rust macros are **hygienic**.
This means:
- Variables defined inside macros do not conflict with outside variables
- Accidental name clashes are prevented

Example:
```rust
macro_rules! demo {
    () => {
        let x = 10;
    };
}
```

The `x` inside the macro is isolated from the rest of the code.

---

## 2. Procedural Macros
Procedural macros are more powerful and advanced.
They operate directly on Rust's **syntax tree**.

They must be written in a **separate crate** with:
```toml
[lib]
proc-macro = true
```

Procedural macros come in three forms:

### a) Custom Derive Macros
Used with `#[derive(...)]`

Example:
```rust
#[derive(Debug, Clone)]
struct User {
    id: u32,
}
```

The `Debug` and `Clone` implementations are generated automatically.

---

### b) Attribute-like Macros
Used as attributes on items.

Example:
```rust
#[route(GET, "/home")]
fn home() {}
```

Commonly used in web frameworks.

---

### c) Function-like Procedural Macros
They look like function calls but are macros.

Example:
```rust
my_macro!(some, tokens);
```

---

## 3. Built-in Macros
Rust provides many built-in macros.

Common examples:

| Macro | Purpose |
|----|--------|
| `println!` | Print to console |
| `format!` | Create formatted string |
| `vec!` | Create a vector |
| `dbg!` | Debug printing |
| `assert!` | Runtime assertion |
| `panic!` | Abort program |

Important:
- `println!` is a macro, not a function

---

## Advantages of Macros
- Zero runtime cost
- Extremely flexible
- Enable powerful abstractions
- Used heavily in Rust standard library

---

## Disadvantages of Macros
- Harder to read than functions
- Error messages can be confusing
- Debugging is more difficult

Rule of thumb:
> Use functions first, macros only when necessary

---

## When NOT to Use Macros
Avoid macros when:
- A normal function can solve the problem
- Code readability suffers
- Logic becomes difficult to understand

---

## Summary
- Macros generate code at compile time
- `macro_rules!` is the most common macro system
- Procedural macros are powerful but complex
- Macros provide zero-cost abstractions
- Overusing macros can harm readability

---

## Interview Tip
"Macros in Rust enable compile-time code generation and help achieve zero-cost abstractions."

---

**End of Notes**

