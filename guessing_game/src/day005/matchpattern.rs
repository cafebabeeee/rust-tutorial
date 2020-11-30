
fn main() {
    let coin: Coin = Coin :: Cc;

    let result: u8 = value_in_cents(coin);

    println!("{}", result);

    let mut count = 0;

    let coin_two: Coin = Coin :: Dime;
    if let Coin :: Quarter(state) = coin_two {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("{}", count);
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Cc,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin :: Penny => {
            println!("Lucky penny!");
            1
        },
        Coin :: Nickel => 5,
        Coin :: Dime => 10,
        Coin :: Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        _ => 0, // default return values
    }
}