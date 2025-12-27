fn main() {
    let num:u8 = 23;
    println!("The Number is: {}", num);
}

// Notes 

// Q. why u8?
// A. u8 is an unsigned 8-bit integer type in Rust, which means it can hold values from 0 to 255. 
//    It is often used when you want to save memory and you know that the values will not exceed this range. 
//    In this case, since 23 is within the range of u8, it is a suitable choice.

// Q. why not i8?
// A. i8 is a signed 8-bit integer type in Rust, which means it can hold values from -128 to 127. 
//    If you are certain that the number will always be non-negative, using u8 can be more efficient in terms of memory usage.

// Q. what is the difference between u8 and i8?
// A. The main difference between u8 and i8 is that u8 is an unsigned type, meaning it can only represent non-negative values (0 to 255), 
//    while i8 is a signed type, meaning it can represent both negative and positive values (-128 to 127). 
//    This affects the range of values they can hold and how they are used in calculations and data storage.

// Q. when to use u8 over i8?
// A. You should use u8 over i8 when you are certain that the values you are working with will always be non-negative. 
//    This can help save memory and improve performance in certain situations. 
//    For example, when dealing with byte data, color values (0-255), or any other scenario where negative values are not applicable.

// Q. when to use i8 over u8?
// A. You should use i8 over u8 when you need to represent both negative and positive values. 
//    This is important in scenarios where the data can fall below zero, such

// Q. what if we does not specify u8 or i8?
// A. If you do not specify u8 or i8, Rust will use the default integer type, which is i32. 
//    This means that the variable will be able to hold values from -2,147,483,648 to 2,147,483,647. 
//    While this provides a larger range, it also uses more memory (4 bytes) compared to u8 or i8 (1 byte each). 
//    Therefore, specifying the appropriate integer type can help optimize memory usage and performance based on your specific needs. as temperature readings, financial calculations, or any other scenario where negative values are meaningful.

// Q. what is the default integer type in Rust?
// A. The default integer type in Rust is i32, which is a signed 32-bit integer. This means it can hold values from -2,147,483,648 to 2,147,483,647.

// Q. why i32 is the default integer type in Rust?
// A. i32 is chosen as the default integer type in Rust because it provides a good balance between range and memory usage for most applications. 
//    It can handle a wide variety of common use cases without consuming too much memory, making it a practical default choice for general-purpose programming. as temperature readings, financial calculations, or any other scenario where negative values are meaningful.