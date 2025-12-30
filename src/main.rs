fn main() {
    let s1: String = String::from("Hello");
    let r1 = &s1;
    let r2 = &s1;

    println!("r1 = {}, r2 = {}", r1, r2);
}

/*
Notes 📝

    &s1 creates an immutable reference.
    You can have multiple immutable references at the same time.
    Immutable references allow read-only access.
    No ownership is moved from s1.

    ```
    let r1 = &s1;
    let r2 = &s1;
    ```
    ✔️ This is valid because both are immutable borrows.

📤 Output works because: r1 and r2 only read the value and No modification happens

🔑 One-Line Rule

👉 Many readers allowed, no writers 📖📖❌✍️
*/