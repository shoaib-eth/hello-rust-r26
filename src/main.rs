fn main() {
    let num = vec![1, 2, 3, 4, 5];

    let result = num
        .iter()
        .filter(|x| *x % 2 != 0) // Keeps only odd numbers - [1, 3, 5]
        .map(|x| *x + 1) // Increment each odd number - [2, 4, 6]
        .find(|x| *x == 6); // find first item equal to 6 -> Some(6)

    println!("{:?}", result.unwrap()); // print 6
}
