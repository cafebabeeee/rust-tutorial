
fn main() {
    
}

fn error_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    // cannot borrow `s` as mutable more than once at a time
    println!("{}, {}", r1, r2);
}

fn problem_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}

fn without_problem_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{}, {}", r1, r2);

    let r3 = &mut s; // no PROBLEM

    println!("{}", r3);
}