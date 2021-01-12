// higher ranked lifetime

use std::{fmt::Debug, u32};

pub trait DoSth<T> {
    fn do_sth(&self, value: T);
}
 
impl<'a, T: Debug> DoSth<T> for &'a u32 {
    fn do_sth(&self, value: T) {
        println!("{:?}", value);
    }
}

pub fn foo(d: Box<dyn for<'a> DoSth<&'a u32>>){
    let s = 10;
    d.do_sth(&s);
} 

struct Pick<F> {
    data: (i32, i32),
    f: F,
}

impl<F> Pick<F> where F: Fn(&(i32, i32)) -> &i32{
    fn call<'a>(&'a self) -> &'a i32{
        (self.f)(&self.data)
    }
}

fn max(data: &(i32, i32)) -> &i32 {
    if data.0 > data.1 {
        return &data.0;
    }
    &data.1
}

pub fn pick_test() {
    let value = (3, 6);
    let elm = Pick {
        data: value,
        f: max
    };

    println!("{:?}", elm.call());
}