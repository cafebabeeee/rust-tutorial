// Closure通常指词法闭包，是一个持有外部环境的函数。外部环境指针闭包定义是所处词法作用域
// 在函数式编程范式中称为自由变量，指并不是在闭包内定义的变量。将自由变量和自身绑定的函数就是闭包

// 返回闭包;
// fn counter(x: i32) -> Box<Fn(i32) -> i32> {
//     Box::new(move |n: i32| n + x)
// }
// Fn : trait
fn counter(x: i32) -> impl Fn(i32) -> i32 {
    move |n: i32| n + x
}

// 闭包特性：1: 延迟执行 2: buhuo环境变量
// 基本语法：|a: i32, b: i32| -> i32 { a + b};
// 闭包函数没有参数只有捕获的自由变量时，管道符里的参数也可以省略
// let add = || a + b;

// 闭包的实现：
// 查看闭包类型：
// fn main ()  {
//     let c1: ()= | | { println!("I'm a closure!"); };
// }
// mismatched types
// expected `()`, found closure
// note: expected unit type `()`
//            found closure `[closure@src\function\closure.rs:21:17: 21:52]`
// [closure@src\function\closure.rs:21:17: 21:52] 是一个由编译器制造的临时存在的闭包实例类型

// 闭包只是一种语法糖，闭包和普通函数的区别就是 闭包可以捕获环境中的自由变量
// 实现闭包：
// 增加trait将函数调用变为可重载的操作符。eg: 将a （b, c, d）这种函数调用变成如下形式
// Fn::call( &a, ( b, c, d ) ); 会对方法接收者进行不可变租借
// FnMut::call_mut( &mut a, ( b, c, d ) ); 会对方法接收者进行可变租借
// FnOnce::call_once( a, ( b, c, d ) ); 会转移方法接收者的所有权

// #![feature(fn_traits, unboxed_closures)]
// 显式指定闭包类型
///
/// ```
/// implemention closure
/// #![feature(unboxed_closures, fn_traits)]
/// struct Closure {
///     env_var: u32,
/// }
///
/// impl FnOnce<()> for Closure {
///     type Output = u32;
///     extern "rust-call" fn call_once(self, args: ()) -> u32 {
///         println!("call closure FnOnce.");
///         self.env_var + 2
///    }
/// }
///
/// impl FnMut<()> for Closure {
///     //type Output = u32;
///     extern "rust-call" fn call_mut(&mut self, args: ()) -> u32 {
///         println!("call closure FnMut.");
///         self.env_var + 2
///     }
/// }
///
/// impl Fn<()> for Closure {
///     //type Output = u32;
///     extern "rust-call" fn call(&self, args: ()) -> u32 {
///         println!("call closure Fn.");
///         self.env_var + 2
///     }
/// }
///
/// fn call_it<F: Fn() -> u32>(f: &F) -> u32{
///     f()
/// }
///
/// fn call_mut<F: FnMut() -> u32>(f: &mut F) -> u32{
///     f()
/// }
///
/// fn call_once<F: FnOnce() -> u32>(f: F) -> u32{
///     f()
/// }
///
/// fn main () {
///     let env_var = 1;
///     let mut c = Closure {env_var: env_var};
///    
///    c();
///     c.call(());
///     c.call_mut(());
///     c.call_once(());
///     let mut c = Closure {env_var: env_var};
///     println!("{}", env_var);
///     {
///         assert_eq!(3, call_it(&c));
///     }
///     {
///         assert_eq!(3, call_mut(&mut c));
///     }
///    {
///         assert_eq!(3, call_once(c));
///    }
/// }
/// ```
// auto implememt Fn() for Copy 
#[allow(unused_variables)]
pub fn closure() {
    let env_var = 1;
    let c: Box<dyn Fn() -> i32> = Box::new(|| env_var + 2);
    fn add(a: i32, b: i32) -> i32 {
        a + b
    };
    println!("{}", c());
    c();
    println!("{}", c());
    add(1, 4); 
    c();
    println!("{}", c());
    let add: Box<dyn Fn(i32, i32) -> i32> =Box::new(|a: i32, b: i32| -> i32 {a + b});
}