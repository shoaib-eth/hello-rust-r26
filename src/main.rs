fn main() {
    let vec = vec![1, 2, 3, 4];

    let double_vec: Vec<i32> = vec.iter().map(|x| x * 2).collect(); // `iter()` passes reference of elements
    println!("{:?}", double_vec);

    let even_vec: Vec<&i32> = vec.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", even_vec);

    let double_vec: Vec<i32> = vec.into_iter().map(|x| x * 2).collect(); // `into_iter()` passes ownership of elements
    println!("{:?}", double_vec);

    match vec
        .into_iter()
        .reduce(|accumulator, Item| accumulator + Item)
    {
        Some(sum) => println!("The sum of vector element is {}", sum),
        None => println!("None"),
    }
}

// Note 📝 - Comment out `double_vec`, `even_vec`, and `double_vec` lines of code to run the match function code.
