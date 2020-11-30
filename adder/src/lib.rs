#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        println!("{}", "CAFEBABE");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn other() {
        panic!("Make this test fail");
    }

    #[test]
    fn add_two_test() {
        let result = add_two(5);
        println!("{}", result);
    }

    #[test]
    fn greeting_test() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
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

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
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