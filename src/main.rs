fn main() {
    // let mut arr1: [u8; 5];  // Array Declaration

    let mut arr1;
    arr1 = [1, 2, 3, 4, 5];

    println!("arr1[0] = {}", arr1[0]);
    println!("arr1 = {:?}", arr1);

    arr1[2] = 30;  // Array Index No. 2 value will be changed to 30
    println!("arr1 = {:?}", arr1); 

    // Array Length
    println!("Array Length is: {}", arr1.len());

}