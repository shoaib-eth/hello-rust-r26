fn main() {
    let arr = [1, 2, 3];

    for item in &arr {
        // Here `arr` is `iter()` type
        println!("{}", item);
    }
    println!("{:?}", arr);

    for item in arr {
        // Here `arr` is `into_iter()` type
        println!("{}", item);
    }
    println!("{:?}", arr);
}

// Notes 📝

// Here we printed the both `println!()` statements.

// because array is a fixed type and fixed types elements always stores in stack and give us a copy trait
// that's why our seconds `println!()` statements runs successfully and doesn't gave an error
// here ownership works but doesn't impact the code running

// if try this in `vector` case, so our second `println!()` statement will give an error, why? because vector is dynamic type and its elements stored in heap memory, so here ownership works stricktly.

// Fixed Type -> Give Copy Trait
// Dynamic Type -> Don't Give Copy Trait, Ownership Works Stricktly.
