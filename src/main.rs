struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

// Here we have flexibility for future development in this code, because we have to change data type in only `Item` type and change struct name in `impl Iterator for <new_struct_name>`. we have to decide and write at once and dont need to touch other lines of code.
impl Iterator for Counter {
    type Item = u32; // `type` keyword is for creating alias for the data types, that means where we write `Item` in our code which means we write `u32`. this is required in `Iterator` case 

    fn next(&mut self) -> Option<Self::Item> {
        // Here `Self` denotes the `Counter` struct. but we cannot write `Counter` directly here because we dealing with the traits, and this is required to sepcify like this.
        if self.count < 5 {
            self.count = self.count + 1;
            return Some(self.count);
        } else {
            return None;
        }
    }
}

fn main() {
    let count = Counter::new();

    for val in count {
        println!("Count: {}", val)
    }
}
