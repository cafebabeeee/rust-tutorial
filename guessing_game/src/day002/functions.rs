
//
fn main() {

    println!("begin invoke another function ");
    another_function(124, 200);

    satament();

    let x = five();

    println!("The value of x is: {}", x);
}

fn another_function(x: i8, y: u8) {
    println!("Another function.function parameter is {}, {}", x, y);
}

// statement and expression
fn satament() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // this is a statement
}

fn plus_one(x: i32) -> i32 {
    x + 1 // this is a statement
}