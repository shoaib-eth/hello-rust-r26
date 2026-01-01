fn main() {
    let arr: [&str; 3] = ["Hello", "World", "Coders"];
    read_arr(arr); // Array directly passed
    println!("arr = {:?}", arr);
}

// This could be expensive, because it copies the array then read
fn read_arr(arr1: [&str; 3]) {
    println!("arr2 = {:?}", arr1);
}

