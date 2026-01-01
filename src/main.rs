fn main() {
    let num = 11;

    fn is_even(num: u8) -> bool{
        // we can do this in most easiest way like this only in one line
        return num % 2 == 0;
        // if num % 2 == 0 {
        //     return true;
        // } 
        // return false;
    }

    // `is_even()` function calling in `match` function
    match num {
        x if is_even(x) => println!("Even"),
        x if !is_even(x) => println!("Odd"),
        _=>println!("Number Is Not Recognizible")
    }
}
