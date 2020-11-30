// 值类型：Value 
//      数据直接存储在栈中，作为右值(在值表达式中)执行赋值操作时，会自动复制一份值的新副本。
// 引用类型：Reference
//      数据存放在堆中，而栈上只保存指向堆中数据的地址
// 值语义(Value Semantic): 按位复制，与原始对象无关
// 引用语义(Reference Semantic): 指针语语义，一般指将数据存放在堆中，
// 通过栈内存中的指针管理堆中数据，并且引用语义禁止按位复制(栈复制)
// 浅复制就是指栈复制，而深复制就是指栈堆数据一起复制
// eg: 智能指针 Box＜T＞ 封装了原生指针，是典型的引用类型。
// Box＜T＞ 无法实现 Copy，意味着它被 Rust 标记为了引用语义，禁止实现按位复制.

// rust中由Copy trait 来区别值语义和引用语义。同时引入新的定义Move(移动)语义、Copy(复制)语义
// fn main() {
//  let x = Box::new(5i32);
//  let y = x; // move semantic
//  println!("This X is {:?}", x);
//}

// ownership:
// owner: 1: 控制资源 2: 出借所有权 3: 转移所有权
// 枚举、结构体无法自动实现Copy trait,数组、元组、Option可以

// let: 绑定，并非传统意义的变量申明。而是一种绑定予以，赋予所有权。
// 绑定具有时间属性(lifetime)、空间属性(scope)
// {}、match、循环、if let、while let会创建词法作用域:
    // let a = Some("hello".to_string());
    // if let Some(s) = a { // a move to s
    //     println!("{:?}", s);
    // }
    // // println!("{:?}", a); error
    // println!("Hello, world!");
    // 闭包会创建词法作用域: 闭包会捕获环境变量 1： 对于复制语义类型，以(&T)进行捕获 2：对于移动语义类型。以move进行捕获
    // 3: 对可变绑定，如果闭包包含修改操作则以(&mut)进行捕获
    // let x = "hello".to_string();
    // let join = |i: &str| {x + i}; // 定义闭包：|| {}
    // println!("{}",join(", world!"));
    // println!("{:?}", x); // error x move to i
// lifetime: 

// mutable immutable: 共享可变状态是万恶之源

fn ownership() {
    let mut s = String::from("hello");

    let (result, len) = calculate_length(s);

    println!("result={}, length={}.", result, len);

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    string_copy();

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,but i32 is Copy, so it’s okay to still
    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.


fn string_copy() {
    let s1 = String::from("hello");

    // shadow clone
    let s2 = s1; // s1 value moved here

    // deep copy
    let s3 = s2.clone();

    println!("The string s1 is , s2 is {}", s2);

    println!("s2 = {}, s3 = {}", s2, s3);
}

fn stack_only_data_copy() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length) // return two result
}

pub fn dangle() -> &String {
    let s = String::from("hello");

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

pub fn no_dangle() -> String {
    let s = String::from("hello");

    s
}