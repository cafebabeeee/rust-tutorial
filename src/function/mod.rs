pub mod closure;
pub mod constfn;
pub mod functions;
pub mod higherorderfunction;
pub mod higherrankedlifetime;
pub mod iterator;

#[cfg(test)]
mod function_test {
    use super::{closure, constfn, higherrankedlifetime, iterator};
    use itertools::Itertools;
    use std::time::SystemTime;

    #[test]
    fn closure() {
        closure::closure();
        closure::escape_closure();
    }
    #[test]
    fn test_const_fn() {
        println!("{}", constfn::GA);
    }

    #[test]
    fn fn_box() {
        let f = closure::return_closure();
        println!("{}", f(4));
        let fn_once = closure::return_fn_once();
        println!("{}", fn_once(4));
        //println!("{}", fn_once(4));
    }

    #[test]
    fn higher_ranked_lifetime() {
        higherrankedlifetime::foo(Box::new(&32u32));
        higherrankedlifetime::pick_test();
    }

    #[test]
    fn inner_iterator() {
        iterator::inner_iterator();
    }

    #[test]
    fn outer_iterator() {
        let now = SystemTime::now();

        println!("{:?}", now);
        iterator::out_iterator();
    }

    use iterator::ExtIteator;
    #[test]
    fn apply_step() {
        let arr = [1, 2, 3, 4, 5, 6];
        // let sum = arr.iter().step(2).fold(0, |acc,x| acc + x);
        // println!("sum is {}", sum);

        let positions = arr.iter().positions(|e| e % 2 == 0);
        for i in positions {
            println!("{:?}", i);
        }
    }
}
