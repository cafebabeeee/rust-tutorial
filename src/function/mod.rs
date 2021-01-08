pub mod functions;
pub mod higherorderfunction;
pub mod closure;
pub mod lifetime;

#[cfg(test)]
mod function_test{
    use super::{closure, lifetime};
    #[test]
    fn closure() {
        closure::closure();
        closure::escape_closure();
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
        lifetime::foo(Box::new(&32u32));
        lifetime::pick_test();
    }
}