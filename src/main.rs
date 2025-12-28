fn main() {
    let str1 = String::from("Hello"); // here str1 owns the String
    let str2 = str1;  // Transfer of ownership from str1 to str2 here, now the original owner str1 is no longer valid
    // println!("str1: {}", str1); // This would cause a compile-time error
    println!("str2: {}", str2);  // This is valid because String does not implement the Copy trait
}

// Important Notes 🗒️ 

// let str1 = String::from("Hello");
// let str2 = str1; // str1 is moved to str2 here
// println!("str1: {}", str1); // This would cause a compile-time error
// println!("str2: {}", str2);  // This is valid because String does not implement the Copy trait
// To fix the above code, you can clone str1:
// let str1 = String::from("Hello");
// let str2 = str1.clone(); // Now str1 is cloned to str2
// println!("str1: {}", str1); // This is now valid
// println!("str2: {}", str2);  // This is also valid

// -------------------

// let a = 5;
// let b = a;
// println!("a: {}", a);
// println!("b: {}", b);