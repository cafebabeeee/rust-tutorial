mod borrow;
mod function;
mod lifetime;
mod ownership;
mod guess_number;
mod variable;

fn main() {

}

#[cfg(test)]
mod test{
    use crate::*;

    #[test]
    fn guess() {
        guess_number::guess::guss();
    }

    #[test]
    fn compute() {
        let a = 15;
        let mut b = 3;
        borrow::borrow::compute(&a, &mut b);
        println!(" b value is {}", b)
    }

    #[test]
    fn borrow() {
        let string = String::from("hello,rust");

        let len = borrow::borrow::calculate_length(&string);

        println!("The length is {}.", len);

        //borrow::borrow::change(&mut string);
    }

    #[test]
    fn dangling() {
        let reference_to_nothing = ownership::ownership::dangle();
        let reference_to_string = ownership::ownership::no_dangle();

        println!("{}, {}.",reference_to_nothing, reference_to_string);
    }

}
