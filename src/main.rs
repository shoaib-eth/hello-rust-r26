fn main() {
    let mut arr: [&str; 3] = ["Hello", "World", "Coders"];
    write_arr(&mut arr); // Array directly passed
    println!("arr = {:?}", arr);
}

fn write_arr(arr1: &mut [&str; 3]) { // arr1 new copy of arr
    arr1[0] = "Hey 👋";
    println!("arr1 = {:?}", arr1);
}

// Output:

// arr1 = ["Hey 👋", "World", "Coders"]
// arr = ["Hey 👋", "World", "Coders"]

// The the result will be same because it points only one array and some changes in it