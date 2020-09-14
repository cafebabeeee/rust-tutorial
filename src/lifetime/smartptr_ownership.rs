// smart ptr 与ownership
// 除了引用之外，rust提供了具有引用(move)语义的智能指针
// 智能指针具有资源的所有权，而reference是所有权的借用
fn eg_1() {
    let x = Box::new("hello");
    let y = x;
    // println!("{}", x); error x move to y
}
// 智能指针解引用：
fn eg_2() {
    let a = Box::new("hello");
    let b = Box::new("Rust".to_string());
    let c = *a;
    let d= *b;
    println!("a= {:?}", a);
    // println!("b= {:?}", b); // borrow of movef value 'b'
}

// 共享所有权Rc<T>和Weak<T>
// Rc<T>可以共享所有权给多个变量，共享一个所有权时，计数+1，计数为0，即所有共享变量
    //都离开作用域时，该值才会被析构，Rc<T>属于单线程计数指针，非线程安全。不允许传递共享给别的线程
use std::rc::Rc;
fn eg_rc(){
   let x = Rc::new(45u32);
   let y1 = x.clone();
	let y2 = x.clone();
	println!("{:?}", Rc::strong_count(&x));
	let w = Rc::downgrade(&x);
	println!("{:?}", Rc::weak_count(&x));
	let y3 = &*x;
	println!("{}", 100 - *x);
}

// 内部可变性 Cell<T> 和 RefCell<T>:
// rust中可变不可变主要针对变量而言，对于结构体，可变不可变智能对其实例进行设置。无非分别制定单个成员是否可变
// Cell<T> RefCell<T>提供内部可变性(Interior Mutability)的容器，而非是智能指针
// 内部可变性容器是对 Struct 的封装
use std::cell::Cell;
use std::cell::RefCell;
struct Foo{
    x: u32,
    y: Cell<u32>
}
fn main (){
    let foo = Foo{x: 1, y: Cell::new(3)};
    assert_eq!(1, foo.x);
    assert_eq!(3, foo.y.get());
    foo.y.set(5);

    let x = RefCell::new(vec![1,2,3,4,5]);
    println!("{:?}", x.borrow());
    let mut mut_v = x.borrow_mut();
    mut_v.push(5);
    println!("{:?}", x.borrow());
    // x.borrow_mut(); 违违反运行时借用检查
}

// 对于实现Copy的类型，可以直接get()、set(),但是没有实现Copy的类型可以使用get_mut()获取可变借用
// Cell<T>使用set、get直接操作包裹值；RefCell<T>使用borrow、borrow_mut返回Ref<T>、RefMut<T>来操作包裹的值
// Cell<T> 无运行时开销，RefCell<T>执行运行时检查，可能会panic!

// Copy On Write Cow<T>:
// Cow<T>是枚举体的智能指针，包含Borrowed、Owned表示所有权的借用和拥有
// 以不可变的方式访问借用内容，以及再需要可变借用或所有权的时候克隆一份数据