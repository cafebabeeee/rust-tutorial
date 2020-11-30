
fn main() {
    let mut s = String::from("hello");

    let (result, len) = calculate_length(s);

    println!("result={}, length={}.", result, len);

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    string_copy();

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,but i32 is Copy, so it’s okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.


fn string_copy() {
    let s1 = String::from("hello");

    // shadow clone
    let s2 = s1; // s1 value moved here

    // deep copy
    let s3 = s2.clone();

    println!("The string s1 is , s2 is {}", s2);

    println!("s2 = {}, s3 = {}", s2, s3);
}

fn stack_only_data_copy() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) // return two result
}