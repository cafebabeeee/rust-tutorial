// lifetime:
// 值的lifetime与词法作用域有关，借用会在函数之间传递，必然会跨越多个词法作用域
// fn eg_1()  {
//     let x;
//     {
//         let a = 5;
//         x = &a;  error
//     }
//     println!("x is {}", x)
// }

// 显式生命周期参数：
// 生命周期参数不能改变任何引用的的生命周期长度，只是用于编译时借用检查
// fn eg<'a, T>(input: &'a T) -> output: &'a T
// 禁止在无参函数方法中返回引用，从函数返回一个引用，其生命周期参数比如与输入参数生命周期参数匹配
fn eg_2<'a>(a: &'a str, b: &'a str) -> &'a str {
    let result = String::from("hello, lifetime");

    // result.as_str() error
    a
}

fn longest_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// 'b: 'a：表示返回引用的lifetime必须不大于'b
fn longest<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// static lifetime: 'static: 存活于整个程序运行期间，所有的字符串字面量都有 'static 生命周期，类型为 &'static str
// lifetime 省略规则：
// · 每个输入位置上省略的生命周期都将成为一个不同的生命周期参数。
// · 如果只有一个输入生命周期的位置（不管是否忽略），则该生命周期都将分配给输出生命周期。
// · 如果存在多个输入生命周期的位置，但是其中包含着&self 或&mut self，则 self 的生命周期都将分配给输出生命周期。

// Trait 对象的lifetime
// · trait 对象的生命周期默认是＇static。
// · 如果实现 trait 的类型包含&＇a X 或&＇a mut X，则默认生命周期就是＇a。
// · 如果实现 trait 的类型只有 T：＇a，则默认生命周期就是＇a。
// · 如果实现 trait 的类型包含多个类似 T：＇a 的从句，则生命周期需要指明
trait Foo<'a> {}
struct FooImpl<'a> {
    name: &'a str,
}

impl<'a> Foo<'a> for FooImpl<'a> {}

fn fool<'a>(s: &'a str) -> Box<Foo<'a> + 'a> {
    Box::new(FooImpl { name: s })
}
