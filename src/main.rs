fn main() {
    let arr: [&str; 3] = ["Hello", "World", "Coders"];
    write_arr(arr); // Array directly passed
    println!("arr = {:?}", arr);
}

fn write_arr(mut arr1: [&str; 3]) { // arr1 new copy of arr
    arr1[0] = "Hey 👋";
    println!("arr1 = {:?}", arr1);
}

// Output:

// arr1 = ["Hey 👋", "World", "Coders"]
// arr = ["Hello", "World", "Coders"]

// There is no ownership concept because array is fixed type
// But this could be very expensive for memory because it copies entire array, and array could be very long.