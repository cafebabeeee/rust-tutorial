// use std::marker::Send;
// use std::marker::Sync;

// send: 可以在线程间安全的转移所有权
// sync: 可以在线程间安全的传递不可变借用
// 线程之间安全地共享变量 Arc＜T＞是Rc＜T＞的线程安全版本
use std::thread;
pub fn move_str() {
    let mut str = String::from("hello");
    // for _ in 0..3 {
    //     thread::spawn( || {
    //         str.push('h');
    //     });
    // }
    thread::spawn(move || {
        str.push('h');
    });
}
