// Dangling Reference

fn main() {
    let reference_to_nothing = create_string_reference();
}

fn create_string_reference()->&String {
    let s: String = String::from("Hello");
    return &s;
}

// It will give an error because `create_string_reference()` finishesh its block on execution, means it returns nothing while we try to borrow `s` string

// read more on error message