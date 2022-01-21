pub mod borrow;

#[cfg(test)]
mod tests {
    use super::borrow;

    #[test]
    fn compute() {
        let a = 15;
        let mut b = 3;
        borrow::compute(&a, &mut b);
        println!(" b value is {}", b)
    }

    #[test]
    fn borrow_fun() {
        let string = String::from("hello,rust");

        let len = borrow::calculate_length(&string);

        println!("The length is {}.", len);

        //borrow::borrow::change(&mut string);
    }

    #[test]
    fn replace() {
        let s = "abc?d";
        let mut chars = s.chars().collect::<Vec<char>>();
        for i in 0..=s.len() - 1 {
            let mut words = ('a'..='z').into_iter();
            if chars[i] == '?' {
                // imutable borrow
                let left = if i == 0 { None } else { Some(chars[i - 1]) };
                let right = if i == s.len() - 1 {
                    None
                } else {
                    Some(chars[i + 1])
                };
                // mutable borrow
                chars[i] = words
                    .find(|&w| Some(w) != left && Some(w) != right)
                    .unwrap();
            }
        }
        let s = chars.iter().collect::<String>();
        println!("{}", s);
    }
    // ...
    // fn retuen_str<'a>() -> &'a str{
    //     let mut s = "Rust".to_string();
    //     for i in 0..3 {
    //         s.push_str("good");
    //     }
    //     // returns a value referencing data owned by the current function
    //     &s[..]
    // }
    //...
    // late bound early bound
    // &'static vs &'a : 'static extends 'a
    // longset short
    fn longest<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'a str {
        if a.len() > b.len() {
            a
        } else {
            b
        }
    }
    fn foo<'a>() {} // late bound
    fn poo<'a: 'a>() {} // early bound
    #[test]
    fn early_boound() {
        // cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
        // let f = foo::<'static> as fn();
        let f = foo as fn();
        let p = poo::<'static> as fn();
        println!("{:p} {:p}", &f, &p);
        println!("{}", f == p);
    }
    struct S;

    #[test]
    fn auiz_13() {
        let [x, y] = &mut [S, S];
        let eq = x as *mut S == y as *mut S;
        println!("{}", eq as u8);
    }
    use std::rc::Rc;
    struct A;
    fn p<X>(x: X) {
        match std::mem::size_of::<X>() {
            0 => println!("0"),
            _ => println!("1"),
        }
    }

    #[test]
    fn test_mem() {
        let a = &A;
        p(a);
        p(a.clone());

        let b = &();
        p(b);
        p(b.clone());

        let c = Rc::new(());
        p(Rc::clone(&c));
        p(c.clone());
    }
}
