// functio definite
// snake_case、Raw Identifier
fn snake_case() {}
fn r#fn() {}
// 函数屏蔽：
// 变量可以多次声明绑定，之前变量被屏蔽(variable shadow),但是同名函数无法多次定义
// 默认的函数定义只在当前作用域有效，会屏蔽作用域外的同名函数
fn f() {
    print!("1");
}
#[allow(unused_variables)]
fn main() {
    f(); // 2
    {
        f(); // 3
        fn f() {
            print!("3");
        }
    }
    f(); // 2
    foo(5);
    fn f() {
        print!("2");
    }
    let s = S { i: 32 };
    fm(s); // s move to fm:s
           // println!("s={:?}", s)
    let (a, b) = addsub(56, 12);
}

// 函数参数模式匹配： 函数参数等价于隐式的let绑定，而let绑定本身是一个模式匹配行为
#[derive(Debug)]
struct S {
    i: i32,
}

fn fm(ref _s: S) {
    println!("{:p}", _s);
}

fn foo(_: i32) { // 忽略参数
                 //ToDo
}

// 函数必须且只能有一个返回值，即便没有显式返回值其实也相当于返回了单元值(). 使用元组可以返回多个值
fn addsub(x: usize, y: usize) -> (usize, usize) {
    (x + y, x - y)
}

// generical function
use std::ops::Mul;
fn square<T: Mul<Output = T>>(x: T, y: T) -> T {
    x * y
}
// turbofish 操作符：
// let a = square::<u32>(12, 2);
// Method Function
// Method 来自与OOP，表示某个实例对象的行为。
#[derive(Debug)]
struct User {
    name: &'static str,
    avatar_url: &'static str,
}

impl User {
    fn show(&self) {
        println!("My name is {}", self.name);
        println!("My avatar_url is {}", self.avatar_url);
    }
}

fn show_user() {
    let user = User {
        name: "emonchen",
        avatar_url: "wwww.cafebabe.com",
    };
    User::show(&user);
    user.show();
}
