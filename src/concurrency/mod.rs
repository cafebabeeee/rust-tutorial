pub mod mutil_thread;
pub mod sendsync;

#[cfg(test)]
mod tests {
    static mut V: i32 = 0;

    //  (1) 从内存中将V的初始值放入寄存器中。
    //  (2) 将寄存器中V的值加1。
    //  (3) 将加1后的值写入内存。
    fn unsafe_seq() -> i32 {
        unsafe {
            V += 1;
            V
        }
    }
    use std::{borrow::Borrow, thread};
    #[test]
    fn test1() {
        for _ in 0..10000 {
            thread::spawn(|| {
                unsafe_seq();
                unsafe {
                    println!("{:?},{}", thread::current().id(), &V);
                }
            });
        }
    }

    #[test]
    #[allow(unused_variables)]
    fn thread_build() {
        let t = thread::Builder::new().name("mythread".to_string());
        let t = t
            .spawn(|| {
                unsafe_seq();
                unsafe {
                    println!("{:?},{}", thread::current().id(), &V);
                    return &V;
                }
            })
            .unwrap();

        unsafe {
            println!("main thread {:?},{}", thread::current().id(), &V);
        }
        let r = t.join();
    }

    use std::cell::RefCell;
    #[test]
    fn thread_local() {
        thread_local! (static FOO: RefCell<u32> = RefCell::new(1));

        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 2;
        });

        thread::spawn(|| {
            FOO.with(|f| {
                assert_eq!(*f.borrow(), 1);
                *f.borrow_mut() = 3;
            });
            println!("{:?}", FOO.borrow());
        });
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 2);
        });
    }

    use std::time::Duration;
    #[test]
    fn park() {
        let park_thread = thread::Builder::new()
            .name("park thread".to_string())
            .spawn(|| {
                println!("parking thread");
                thread::park();
                println!("thread unparked");
            })
            .unwrap();

        thread::sleep(Duration::from_millis(10000));
        park_thread.thread().unpark();
        park_thread.join().unwrap();
    }
}
