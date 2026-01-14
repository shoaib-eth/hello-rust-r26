# 🦀 Rust Smart Pointers – Complete Notes (Ownership, Rules & Examples)

## What are Smart Pointers in Rust? 🧠

In Rust, a **smart pointer** is a data structure that:
- Acts like a pointer (points to data)
- **Manages ownership, memory, and borrowing rules automatically**

Simple definition:
> Smart pointers = pointers + extra rules + automatic cleanup


---

## Why Smart Pointers Exist 🤔

Normal references (`&T`, `&mut T`):
- Do NOT own data
- Do NOT manage memory

Smart pointers:
- Own data
- Clean up automatically (via `Drop`)
- Enforce safety rules

They are heavily used when:
- Data must live on the heap
- Ownership needs to be shared
- Mutability rules must be relaxed safely

---

## Common Smart Pointer Traits 🔗

Most smart pointers implement:

- `Deref` → behave like references (`*ptr`)
- `Drop` → cleanup logic when value goes out of scope

This is why smart pointers feel like normal references in use.

---

## Types of Smart Pointers in Rust 📦

According to Rust standard library and your slides fileciteturn1file0, the main smart pointers are:

1. `Box<T>`
2. `Rc<T>`
3. `Arc<T>`
4. `RefCell<T>`
5. `Mutex<T>`
6. `RwLock<T>`

We will go one by one — **nothing skipped** ✅

---

## 1️⃣ `Box<T>` – Heap Allocation with Single Ownership 📦

### What is `Box<T>`?

`Box<T>`:
- Allocates data on the **heap**
- Has **single ownership**
- Very lightweight

This matches slide description exactly fileciteturn1file0

---

### Example: Using `Box<T>`

```rust
fn main() {
    let x = Box::new(5);
    println!("{}", x);
}
```

Explanation 👶:
- `5` is stored on heap
- `x` owns the value
- When `x` goes out of scope → memory is freed

---

### When to Use `Box<T>`?

- Large data types
- Recursive data structures
- Trait objects (`Box<dyn Trait>`)

Example: Recursive enum

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

---

## 2️⃣ `Rc<T>` – Reference Counted (Single-Threaded) 🔢

### What is `Rc<T>`?

`Rc<T>` allows:
- **Multiple owners** of the same data
- Only in **single-threaded** programs

Ownership is tracked using a **reference count**.

---

### Example: `Rc<T>`

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("count = {}", Rc::strong_count(&a));
}
```

Explanation:
- Data is freed only when count becomes `0`
- No mutable access allowed directly

---

### Rules of `Rc<T>` 🔐

- ❌ No mutable borrowing
- ❌ Not thread-safe
- ✅ Multiple readers allowed

---

## 3️⃣ `Arc<T>` – Atomic Reference Counted (Multi-Threaded) 🧵

### What is `Arc<T>`?

`Arc<T>` is like `Rc<T>`, but:
- Thread-safe
- Uses atomic operations

---

### Example: `Arc<T>`

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(5);

    let handles: Vec<_> = (0..3).map(|_| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("{}", data);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
}
```

---

### Rules of `Arc<T>` 🔐

- Thread-safe ✅
- Slower than `Rc<T>` due to atomics
- Still immutable unless combined with `Mutex` or `RwLock`

---

## 4️⃣ `RefCell<T>` – Interior Mutability (Single-Threaded) 🔓

### What is Interior Mutability?

Interior mutability allows:
> Mutating data even when it is immutably borrowed

Rules checked at **runtime**, not compile time.

---

### Example: `RefCell<T>`

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);

    *x.borrow_mut() += 1;
    println!("{}", x.borrow());
}
```

---

### Rules of `RefCell<T>` ⚠️

- Single-threaded only
- Panics at runtime if borrow rules are violated
- Allows:
  - Many immutable borrows OR
  - One mutable borrow

---

## 5️⃣ `Mutex<T>` – Interior Mutability with Thread Safety 🔒

### What is `Mutex<T>`?

`Mutex<T>`:
- Allows mutation across threads
- Locks data before access

---

### Example: `Mutex<T>`

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..5).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    println!("{}", *counter.lock().unwrap());
}
```

---

### Rules of `Mutex<T>` 🔐

- Thread-safe
- Blocking lock
- Possible deadlocks if misused

---

## 6️⃣ `RwLock<T>` – Multiple Readers, One Writer 📖✏️

### What is `RwLock<T>`?

`RwLock<T>`:
- Many readers allowed
- Only one writer at a time

---

### Example: `RwLock<T>`

```rust
use std::sync::RwLock;

fn main() {
    let data = RwLock::new(5);

    let r1 = data.read().unwrap();
    let r2 = data.read().unwrap();
    drop(r1);
    drop(r2);

    let mut w = data.write().unwrap();
    *w += 1;
}
```

---

## Smart Pointer Comparison Table 📊

| Type | Ownership | Thread Safe | Mutability |
|----|---------|------------|-----------|
| Box<T> | Single | ❌ | Normal |
| Rc<T> | Shared | ❌ | Immutable |
| Arc<T> | Shared | ✅ | Immutable |
| RefCell<T> | Single | ❌ | Runtime-checked |
| Mutex<T> | Shared | ✅ | Locked |
| RwLock<T> | Shared | ✅ | Read/Write |

---

## Common Smart Pointer Combos 🧩

- `Rc<RefCell<T>>` → shared + mutable (single-threaded)
- `Arc<Mutex<T>>` → shared + mutable (multi-threaded)

---

## Common Beginner Mistakes 🚨

- Using `Rc` in threads ❌
- Overusing `RefCell`
- Forgetting deadlock risks
- Using smart pointers when references are enough

---

## Mental Model 🧠

Think like this:

- `Box` → single owner box 📦
- `Rc` → shared ownership counter 🔢
- `Arc` → thread-safe counter 🧵
- `RefCell` → runtime borrow checker 🔓
- `Mutex` → locked room 🔒
- `RwLock` → library room 📖✏️

---

## Summary ✨

- Smart pointers manage memory safely
- Different types solve different ownership problems
- Rust forces you to choose safety explicitly

---

## Interview Tip 💼

"Rust smart pointers combine ownership, borrowing, and automatic memory management to enable safe and expressive system-level programming."

---

**End of Notes** 🦀🔥

