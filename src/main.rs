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

#[cfg(test)]
mod test{
    use crate::*;
    use ownership::ownership;

    #[test]
    fn dangling() {
        //let reference_to_nothing = ownership::dangle();
        let reference_to_string = ownership::no_dangle();

        println!("{}", reference_to_string);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn guess_new_test() {
        let guess = Guess::new(200);
        println!("{}", guess.value);
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
