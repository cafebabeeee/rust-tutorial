mod borrow;
mod function;
mod lifetime;
mod ownership;
mod guess_number;
mod variable;
mod error;
mod mio;
mod struct_and_enum;
mod trait_demo;
mod collection_demo;

fn main() {

}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test{
    use crate::*;
    use borrow::borrow;
    use guess_number::guess;
    use ownership::ownership;

    #[test]
    fn guess() {
        guess::guss();
    }

    #[test]
    fn compute() {
        let a = 15;
        let mut b = 3;
        borrow::compute(&a, &mut b);
        println!(" b value is {}", b)
    }

    #[test]
    fn borrow() {
        let string = String::from("hello,rust");

        let len = borrow::calculate_length(&string);

        println!("The length is {}.", len);

        //borrow::borrow::change(&mut string);
    }

    #[test]
    fn dangling() {
        let reference_to_nothing = ownership::dangle();
        let reference_to_string = ownership::no_dangle();

        println!("{}, {}.",reference_to_nothing, reference_to_string);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn guess_new_test() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn it_works_with_result() -> Result<(), String> {
        if 2 + 2 == 3 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}
