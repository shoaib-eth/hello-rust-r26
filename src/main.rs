fn main() {
    let vrr: Vec<&str> = vec!["Hello", "World", "Rustians"]; 
    write_vrr(&vrr); 
    
    println!("vrr = {:?}", vrr);  
}

fn write_vrr(vrr2: &Vec<&str>) { 
    println!("vrr2 = {:?}", vrr2);
}