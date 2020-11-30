
fn main() {
    let string = String::from("hello,rust");

    let len = calculate_length(&string);

    println!("The length is {}.", len);

    change(&mut string);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}