//高阶函数是指以函数作为参数或者返回值的函数，函数是一等公民
type MathOp = fn(i32, i32) -> i32;

fn math(op: MathOp, a: i32, b: i32) -> i32 {
    op(a, b)
}

// 函数指针
fn hello() {}
#[allow(unused_variables)]
fn one_arg(x: i32) {}

#[allow(unused_variables)]
fn main() {
    let fn_ptr1: fn(i32) = one_arg;
    let fn_ptr: fn() = hello;
    //println!("{:p}", fn_ptr);
    let other_fn = fn_ptr;
}
