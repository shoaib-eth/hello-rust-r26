// Vector Dynamic Array

fn main() {
    let mut v: Vec<i32> = Vec::new(); // Vector Declaration
    let mut v1 = Vec::<i32>::new(); // Another Way For Vector Declaration
    let mut v2 = vec![1, 2, 3, 4, 5]; // Initialized Vector like Fixed Type Array, we an change this in future by `push()` 

    v.push(10);
    v.push(11);
    v.push(12);

    v1.push(20);
    v1.push(21);
    v1.push(22);

    println!("Vector v = {:?}", v);
    println!("Vector v1 = {:?}", v1);
    println!("Vector v2 = {:?}", v2);

    // Add new value to v2 (Initialized Vector)
    v2.push(6);
    println!("Vector v2 = {:?}", v2);

    // Delete a value from v2 
    v2.pop();
    println!("Vector v2 = {:?}", v2);

    println!("Length of v2 = {}", v2.len());
}


// Output:

/* 
Vector v = [10, 11, 12]
Vector v1 = [20, 21, 22]
Vector v2 = [1, 2, 3, 4, 5]
Vector v2 = [1, 2, 3, 4, 5, 6]
Vector v2 = [1, 2, 3, 4, 5]
Length of v2 = 5
*/