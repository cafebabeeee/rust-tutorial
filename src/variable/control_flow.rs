// 
 fn flow() {

    if_condition();
    loop_demo();
    range_with_rev();
 }

 fn range_with_rev() {

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
 }

 fn if_condition() {

    let number = 100;
    if number >= 10 {
        println!("The number is more than 10!");
    } else {
        println!("The number is less than 10!");
    }

 }

 fn condition() {
    let condition = true;

    let number = if condition {
        5
    } else {
        // "six" // The if and else arms have value types that are incompatible
        6
    };

    println!("The value of number is: {}", number);
 }

 // repeation with loop
fn loop_demo() {
    loop{
        println!("Hello,Rust!");
        break;
    }
}

fn loop_with_result() {
    let mut count = 0;

    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // add the value after breank expression 
        }
    };

    println!("The loop result is {}", result);
}

fn condition_loop_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_with_for() {
    let array = [1, 2, 3, 4, 5, 6];

    for element in array.iter() {
        println!("The value is {}", element);
    } 
}