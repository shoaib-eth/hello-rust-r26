# 🦀 Rust Iterator Adapter Methods – Complete Notes (With Examples & Ownership)

## What are Iterators in Rust? 🔁
In Rust, an **iterator** is something that lets you process a sequence of values **one by one**.

Instead of using traditional loops (`for`, `while`) directly, Rust encourages:
> **Iterator-based programming** 🧠✨

Why?
- More expressive
- Less error‑prone
- Very powerful when combined with closures

---

## Iterator Pipeline Mental Model 🏗️

Think of iterators like a **factory conveyor belt** 🏭:

1. **Source** → `iter()`, `iter_mut()`, `into_iter()`
2. **Adapters** → `map`, `filter`, `take`, `skip`, etc.
3. **Consumer** → `collect`, `reduce`, `for_each`

Nothing actually runs until a **consumer** is called.

---

## Given Example Code (Reference)

```rust
fn main() {
    let vec = vec![1, 2, 3, 4];

    let double_vec: Vec<i32> = vec.iter().map(|x| x * 2).collect();
    println!("{:?}", double_vec);

    let even_vec: Vec<&i32> = vec.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even_vec);

    let double_vec: Vec<i32> = vec.into_iter().map(|x| x * 2).collect();
    println!("{:?}", double_vec);

    match vec.into_iter().reduce(|accumulator, item| accumulator + item) {
        Some(sum) => println!("The sum of vector element is {}", sum),
        None => println!("None")
    }
}
```

---

## `iter()` – Borrowing Elements 🧾

```rust
vec.iter()
```

### What it does:
- Iterates over **references** to elements
- Type: `&T`
- Ownership stays with the collection

### Example

```rust
let double_vec: Vec<i32> = vec.iter()
    .map(|x| x * 2)
    .collect();
```

Explanation step-by-step 👶:
- `iter()` → gives `&i32`
- `map(|x| x * 2)` → Rust auto-derefs `x`
- `collect()` → builds a new `Vec<i32>`

👉 `vec` is still usable after this.

---

## `map()` – Transforming Values 🔄

```rust
map(|x| x * 2)
```

### What it does:
- Takes each item
- Applies a closure
- Returns a **new iterator**

Important:
- `map` does NOT modify the original data
- It is **lazy** 😴 (runs only when consumed)

---

## `collect()` – Consuming the Iterator 📦

```rust
collect::<Vec<i32>>()
```

### What it does:
- Consumes the iterator
- Builds a collection (`Vec`, `HashMap`, etc.)

Without `collect`, nothing executes ❌

---

## `filter()` – Selecting Values 🎯

```rust
let even_vec: Vec<&i32> = vec.iter()
    .filter(|x| *x % 2 == 0)
    .collect();
```

### Explanation:
- `iter()` → gives `&i32`
- `filter()` closure receives `&&i32`
- `*x` → dereference to `i32`

Result:
- `Vec<&i32>` (references, not owned values)

Ownership reminder 🔐:
> Original vector still owns the data

---

## `into_iter()` – Taking Ownership 🧨

```rust
vec.into_iter()
```

### What it does:
- Moves elements out of the collection
- Type: `T`
- Collection becomes unusable after this

### Example

```rust
let double_vec: Vec<i32> = vec.into_iter()
    .map(|x| x * 2)
    .collect();
```

Explanation:
- `x` is `i32` (owned)
- No references involved
- Faster & simpler in many cases

⚠️ After this:
```rust
vec // ❌ cannot be used
```

---

## `reduce()` – Folding Values Into One 🧮

```rust
vec.into_iter().reduce(|acc, item| acc + item)
```

### What it does:
- Combines all elements into **one value**
- Uses a closure repeatedly

### Step-by-step:

Vector: `[1, 2, 3, 4]`

- `acc = 1`, `item = 2` → `3`
- `acc = 3`, `item = 3` → `6`
- `acc = 6`, `item = 4` → `10`

Result:
```rust
Some(10)
```

Why `Option`? 🤔
- Empty iterator → `None`

---

## Why Code Needs Commenting 📝

```rust
// Note: Comment out earlier uses of vec
```

Reason:
- `into_iter()` **moves ownership**
- You cannot use `vec` again after it

Rust enforces this at **compile time** 💪

---

## Other Important Iterator Adapter Methods ⭐

### `take(n)`
Takes first `n` elements

```rust
vec.iter().take(2)
```

---

### `skip(n)`
Skips first `n` elements

```rust
vec.iter().skip(1)
```

---

### `enumerate()`
Gives index + value

```rust
vec.iter().enumerate()
```

---

### `any()` / `all()`
Checks conditions

```rust
vec.iter().any(|x| *x > 3);
vec.iter().all(|x| *x > 0);
```

---

## Ownership Summary Table 🔐

| Method | Item Type | Ownership |
|------|---------|-----------|
| `iter()` | `&T` | Borrow |
| `iter_mut()` | `&mut T` | Mutable borrow |
| `into_iter()` | `T` | Move |

---

## Common Beginner Mistakes 🚨

- Forgetting `collect()`
- Confusing `iter()` vs `into_iter()`
- Using `vec` after `into_iter()`

---

## Big Picture 🧠

Rust iterators:
- Are lazy
- Use closures heavily
- Are zero‑cost abstractions
- Replace most loops safely

---

## Summary ✨

- Iterators process sequences safely
- Adapters (`map`, `filter`) transform data
- Consumers (`collect`, `reduce`) execute logic
- Ownership rules still apply

---

## Interview Tip 💼
"Rust iterators are lazy, composable, and zero‑cost, allowing expressive data processing while enforcing ownership and borrowing at compile time."

---

**End of Notes** 🦀🔥

