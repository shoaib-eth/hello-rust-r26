fn main() {
    let vrr: Vec<&str> = vec!["Hello", "World", "Rustians"]; // Here, `vrr` is the owner of all the array elements
    write_vrr(vrr); // ownership transfered
    
    // println!("vrr = {:?}", vrr);  // Compile time error
}

fn write_vrr(vrr2: Vec<&str>) { // `vrr2` is the current owner of `vrr`
    println!("vrr2 = {:?}", vrr2);
}

// Important Notes 📝

// 1. Ownership is exist in vector

/* Onwnership Tranfer */
// vrr  - >  Ownership Transfered through write_vrr()  ->  vrr2