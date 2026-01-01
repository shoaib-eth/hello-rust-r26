fn main() {
    let num = 2;

    // match num {
    //     1=>println!("Number is One"),
    //     2=>println!("Number is Two"),
    //     5=>println!("Number is Five"),
    //     _=>println!("Number is Not Recongnizible")
    // }

    // We can do more type of mathematical operations, for e.g.,
    match num {
        1 | 3=>println!("Number is One"),
        2 | 4=>println!("Number is Two"),
        5=>println!("Number is Five"),
        _=>println!("Number is Not Recongnizible")
    }
}
