fn main() {
    let mut x = 5;
    println!("The number is {}", x);
    x = 7;
    println!("The number is {}", x);

    // define a const，must annotate the type,
    // underscores can be inserted in numeric literals to improve readability
    const MAX_POINTS: u32 = 100_000;
    // new variable shadows the previous variable
    let x = 8;
    println!("The number is {}", x);

    // you can change the data type with same name variable
    let spaces = "revious variable";
    let spaces = spaces.len();

    // i8 can store numbers from -(2^7) to 2^7 - 1
    // u8 can store numbers from 0 to 2^8 - 1
    let y: i8 = 150; // literal out of range for `i8`
    println!("The number y is {}", y)
}