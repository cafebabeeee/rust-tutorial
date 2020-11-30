#[warn(unused_mut)]
fn main() {
 
    let mut data = "initial content";

    let s = data.to_string();

    let s1 = "initial content".to_string();

    let s2 = String::from("initial contents");

    println!("{}-{}", s, s1);


    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{}", s); // s is Зд
}

fn str_concat() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}