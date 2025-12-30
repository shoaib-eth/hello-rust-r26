fn main() {
    let mut s1 = String::from("Hello");   // Make this mutable by `mut` keyword
    append_string(&mut s1);    // Applied `&mut`
    println!("The New String is {} ", s1);

    // let len = calculate_string(&mut s1); 
    // println!("The length of {} is {}", s1, len);
}

// fn calculate_string(s2: &String) -> usize {
//     return s2.len();
// }

fn append_string(s3: &mut String) {  // Applied `&mut` 
    s3.push_str(" World");
}


// Note 📝

// If we want to change the borrowed string, we have to apply `&mut` keyword.

// Without `&mut` keyword or only with `&` keyword, the string changes in borrow is not possible, gives an error

/* 
1️⃣ What is Borrowing?

   Borrowing means using a value without taking ownership.
   It is done using references:
   1. Immutable borrow → &
   2. Mutable borrow → &mut 

2️⃣ Mutable Borrow (&mut)

   Allows modifying the original value.
   Only ONE mutable borrow allowed at a time.
   The original variable must be mutable (mut).

✅ From your code:

   let mut s1 = String::from("Hello");
   append_string(&mut s1);

3️⃣ Why mut s1 is required?

   append_string modifies the string.
   Rust enforces safety → modification allowed only if: variable is mut and borrow is &mut

❌ Without mut:

   let s1 = String::from("Hello"); // ❌ error
   append_string(&mut s1);

4️⃣ Function Using Mutable Borrow
   fn append_string(s3: &mut String) {
      s3.push_str(" World");
   }

   s3 is a mutable reference
   No ownership transfer
   Changes reflect in s1

📤 Output: Hello World

5️⃣ Immutable Borrow (&String)

   1. Used when only reading data
   2. Multiple immutable borrows allowed
   3. No modification allowed


6️⃣ Borrowing Rules (Very Important 🔥)

✔️ Any number of immutable borrows

✔️ Only one mutable borrow

❌ Mutable and immutable borrow together → compile error

7️⃣ Real-Life Analogy 🧠

&String → Reading a book 📖

&mut String → Editing the book ✍️

Only one editor, but many readers allowed

✅ Final Summary

&mut → modify data
& → read-only

Borrowing avoids ownership transfer
Rust checks everything at compile time
*/