mod borrow;
mod function;
mod lifetime;
mod ownership;

fn main() {
    let a = 15;
    let mut b = 3;
    borrow::borrow::compute(&a, &mut b);
    println!(" b value is {}", b)
}
