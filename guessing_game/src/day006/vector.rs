
fn main() {

    let mut vec: Vec<i32> = Vec :: new();

    {
        let mut vec = vec![1, 2, 3];
        vec.push(4);
    } // vec goes out of the scope and is freed there

    vec.push(4);

    println!("{:?}", vec);

    let mut vec = vec![1, 2, 3, 4, 5, 6];

    let third: &i32 = &vec[2];

    println!("The third element is {}.", third);

    match vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut vec {
        *i += 10;
        println!("{}", i);
    }

    // using enum to store mutiple Types
    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12)];

    println!("{:?}", row);
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}