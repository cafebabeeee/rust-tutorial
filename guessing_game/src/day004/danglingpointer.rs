
// The Rules of References
// Let’s recap 概括 what we’ve discussed about references:

// 1.At any given time, you can have either one mutable reference 
//     or any number of immutable references.
// 2.References must always be valid.

fn main() {
    let reference_to_nothing = dangle();
    let reference_to_string = no_dangle();

    println!("{}, {}.",reference_to_nothing, reference_to_string);
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}