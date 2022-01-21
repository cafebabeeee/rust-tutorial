// Send Sync
// 如果类型T实现了Send: 该类型实例可以在线程间安全传递所有权
// 如果实现了Sync: 表明该类型实例在多线程并发中不会导致内存不安全，可以安全跨线程共享

// use std::thread;
// fn rg_1 (){
//     let mut data = vec![1, 2, 3];
//     for i in 0..3 {
//         // use of moved value: 'data'
//         thread::spawn(move || {
//             data[i] += 1;
//         });
//     }
// }
