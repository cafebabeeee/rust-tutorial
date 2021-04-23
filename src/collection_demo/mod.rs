mod hashmap;
mod string;
mod vector;
#[cfg(test)] 
mod tests{
    use std::{collections::VecDeque, string, vec};


    #[test]
    ///```
    ///impl Add<&str> for String {
    ///    type Output = String;
    ///
    ///    #[inline]
    ///    fn add(mut self, other: &str) -> String {
    ///        self.push_str(other);
    ///        self
    ///    }
    ///}
    ///```
    fn string_add() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // note s1 has been moved here and can no longer be used
        let s3 = s1 + &s2; 
        println!("{}", s3)
    }

    #[test]
    fn string() {
        let string = String::new();
        assert_eq!("", string);
        let string = String::from("hello rust");
        assert_eq!("hello rust", string);
        let string = String::with_capacity(20);
        assert_eq!("", string);
        let str = "the tao of rust";

        let string: String  = str.chars().filter(|c| !c.is_whitespace()).collect();
        assert_eq!("thetaoofrust", string);
        let m = string.matches("tao");
        let string = str.to_owned();
        assert_eq!("the tao of rust", string);
        let string = str.to_string();
        let str = &str[11..15];
        println!("{}", str);
    }

    #[test]
    fn string_ops() {
        let str = "the tao";
        println!("{:?}", str.as_bytes());
        println!("{:?}", str.chars());
        let v: Vec<&str> = "1abc2abc3".matches(char::is_numeric).collect();
        assert_eq!(v, ["1", "2", "3"]);
    }

    #[test]
    fn vec_prealloc() {
        let mut vec = Vec::with_capacity(10);
        for i in 0..10 {
            vec.push(i);
        }

        println!("{}", vec.capacity());

        vec.clear();
        println!("{}", vec.capacity());
        vec.shrink_to_fit();
        println!("{}", vec.capacity());
        for i in 0..10 {
            vec.push(i);
            // output 4 4 4 4 8 8 8 8 16 16
            print!(" {}", vec.capacity());
        }
    }

    #[test]
    fn zero_size() {
        let mut vec = Vec::new();
        vec.push(());
        // ouput 18446744073709551615
        println!("{}", vec.capacity());
    }
}