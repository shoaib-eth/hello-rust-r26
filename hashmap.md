# 🦀 Rust HashMap – Complete Notes (With Ownership, Borrowing & Examples)

## What is a HashMap in Rust? 🗂️
A **HashMap** is a collection that stores data as **key–value pairs**.

Simple definition:
> HashMap = 🔑 Key → 📦 Value

Rust HashMap is provided by:
```rust
use std::collections::HashMap;
```

It is used when:
- You want fast lookup by key ⚡
- Data has no fixed order
- You want dictionary / map–like behavior

---

## Why HashMap Exists 🤔

Vectors (`Vec`) are good when:
- Data is ordered
- Access is by index

HashMaps are good when:
- Access is by **name / id / key**
- You don’t care about order

Real-life analogy 🌍:
> Phone contacts 📱 (name → number)

---

## HashMap Basic Syntax

```rust
let mut map: HashMap<KeyType, ValueType> = HashMap::new();
```

- HashMap is **growable**
- Most operations are **O(1)** average time

---

## Given Example Code (Reference)

```rust
use std::collections::HashMap;

fn main() {
    let mut students: HashMap<String, u32> = HashMap::new();

    students.insert("Ravi".to_owned(), 100);
    students.insert("Raju".to_owned(), 10);
    students.insert("Lalu".to_owned(), 1);

    for (student, marks) in students.iter() {
        println!("Student name: {:?} marks={}", student, marks);
    }

    students.insert("Raju".to_owned(), 200);

    match students.get("Raju") {
        Some(marks) => println!("Found: {}", marks),
        None => println!("Not Found"),
    }
}
```

---

## Step-by-Step Explanation 🧠

### 1️⃣ Creating a HashMap

```rust
let mut students: HashMap<String, u32> = HashMap::new();
```

- `String` → key type
- `u32` → value type
- `mut` required because we insert/update data

---

### 2️⃣ Inserting Values

```rust
students.insert("Ravi".to_owned(), 100);
```

Important ownership concept 🔐:
- HashMap **takes ownership** of keys and values
- That’s why we use `String`, not `&str`

---

### 3️⃣ Iterating Over HashMap

```rust
for (student, marks) in students.iter() {
    println!("Student name: {:?} marks={}", student, marks);
}
```

Explanation:
- `iter()` → borrows the HashMap
- `student` → `&String`
- `marks` → `&u32`

Order ⚠️:
> HashMap does NOT guarantee order

---

### 4️⃣ Updating an Existing Key

```rust
students.insert("Raju".to_owned(), 200);
```

Behavior:
- Old value (`10`) is **overwritten**
- New value becomes `200`

HashMap rule:
> Keys are unique 🔑

---

### 5️⃣ Reading Values Using `get()`

```rust
match students.get("Raju") {
    Some(marks) => println!("Found: {}", marks),
    None => println!("Not Found"),
}
```

Explanation:
- `get()` returns `Option<&V>`
- Safe access (no panic)

---

## Ownership & Borrowing Reminder 🔐

- `insert()` → moves ownership
- `get()` → immutable borrow
- `iter()` → immutable borrow
- You cannot modify HashMap while iterating

Rust enforces safety at compile time 💪

---

## Important HashMap Methods (Must Know) ⭐

---

### `entry()` – Insert Only If Key Missing 🧠

```rust
students.entry("Amit".to_owned()).or_insert(50);
```

Meaning:
- If key exists → do nothing
- If missing → insert `50`

Very useful for counters 📊

---

### Frequency Counter Example 📊

```rust
let mut freq = HashMap::new();

for ch in "hello".chars() {
    *freq.entry(ch).or_insert(0) += 1;
}
```

Result:
```text
h:1, e:1, l:2, o:1
```

---

### `contains_key()` – Check Existence 🔍

```rust
if students.contains_key("Ravi") {
    println!("Ravi exists");
}
```

---

### `remove()` – Delete Entry ❌

```rust
students.remove("Lalu");
```

---

### Mutable Access with `get_mut()` ✏️

```rust
if let Some(marks) = students.get_mut("Ravi") {
    *marks += 10;
}
```

---

## HashMap with Custom Types 🧩

```rust
#[derive(Hash, Eq, PartialEq)]
struct StudentId(u32);

let mut map: HashMap<StudentId, String> = HashMap::new();
```

Key requirements:
- `Hash`
- `Eq`
- `PartialEq`

---

## HashMap vs BTreeMap 🌳

| Feature | HashMap | BTreeMap |
|------|--------|----------|
| Order | No | Yes (sorted) |
| Speed | Faster | Slightly slower |
| Use case | Fast lookup | Ordered traversal |

---

## Common Beginner Mistakes 🚨

- Using `&str` as key without ownership
- Expecting sorted order
- Mutating during iteration
- Forgetting `mut`

---

## Mental Model 🧠

Think of HashMap as:
> A locker room 🗄️ where each locker has a unique key

You can:
- Put items
- Replace items
- Check items

---

## Summary ✨

- HashMap stores key–value pairs
- Keys are unique
- Ownership matters
- `get()` returns `Option`
- `entry()` is very powerful

---

## Interview Tip 💼

"Rust HashMap provides fast key-based lookup while enforcing ownership and borrowing rules for safe concurrent access."

---

**End of Notes** 🦀🔥

