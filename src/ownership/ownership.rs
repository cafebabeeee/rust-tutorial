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