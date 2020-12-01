
// Compound types can group multiple values into one type.
// Rust has two primitive compound types: tuples and arrays
#[allow(unused_variables)]
fn compound_type() {

    // Tuples
    // tuples hax fixed length,cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);


    // tuple is considered a single compound element
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // we can access a tuple element directly by using a period (.) 
    // followed by the index of the value we want to access
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // Arrays

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June",           "July", "August", "September", "October", "November", "December"];

    let a: [i8;5] = [1, 2, 3, 4, 5];

    let a = [3; 5]; // [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

}