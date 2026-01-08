fn main() {
    let vec: Vec<i32> = vec![1, 2, 3];

    for item in vec.iter() {
        // Vector ownership is not transfered, it takes refrence of elements only
        println!("{}", item);
    }

    println!("{:?}", vec); // This will run successfully

    for item in vec.into_iter() {
        // Vector ownership is transfered
        println!("{}", item);
    }

    // println!("{:?}", vec); // This will give an compile time error, because `vec` ownership has transfered to `into_itr()`, so `vec` is no longer owner
}

// Notes 📝

// Iterators works behind the scene, means when we doesn't specify type in for loop with `vec`, it automatially assign `vec` with `into_iter()`, this is the default behaviour
// In `for` loop instead of only `vec` we can write it `&vec`, so it takes the reference of the elements not the ownership

// So when we write `for items in vec`, it autometically converts into `vec.into_iter()` behind the scene which takes ownership of all the elements
// to remove the ownership, we have to specify `&vec` or `vec.iter()`

// Adding `into_iter()` in for loops variable is its default behaviour of rust.

// Tip - when we specify var in `for` loop, rust automatically sets the data type with elements, see that data type
