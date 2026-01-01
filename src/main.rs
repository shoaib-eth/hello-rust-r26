fn main() {
    let arr: [&str; 3] = ["Hello", "World", "Coders"];
    read_arr(&arr); // Array directly passed
    println!("arr = {:?}", arr);
}

// It is inexpensive way of reading array, because we passed the array reference only, not copy the entire array
fn read_arr(arr1: &[&str; 3]) {
    println!("arr2 = {:?}", arr1);
}

