// 引用(reference)与借用(borrow):
// 引用是rust提供的一种指针语义，指针是指向某块内存的地址，而引用可以看作某块内存的别名(Alias)
// 引用分为可变引用(&)与不可变引用(&mut)
// 引用&x 也为借用Borrowing, 通过&操作符完成所有权的借用，借用个不会发生所有权转移
// 借用后，所有者：1: 不可变借用期间, 所有者无法修改资源，可以访问，无法再次出借可变借用，可以出借不可变
//               2: 可变借用期间，所有者不能访问资源，也不可以再次出借所有权
// 引用离开其作用域时便归还所有权

// borrow rule:
// 1: 借用生命周期不能大于所有者生命周期
// 2: 可变借用不能有别名(Alias)
// 3: 不可变借用不能再次出借为可变借用
// *: 对移动语义类型解引用会转移所有权
pub fn compute(input: &u32, output: &mut u32) {
    if *input > 10 {
        *output = 1;
    }
    if *input > 5 {
        *output *= 2;
    }
}
pub fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn error_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    // cannot borrow `s` as mutable more than once at a time
    println!("{}, {}", r1, r2);
}

pub fn problem_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

pub fn without_problem_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{}, {}", r1, r2);

    let r3 = &mut s; // no PROBLEM

    println!("{}", r3);
}
