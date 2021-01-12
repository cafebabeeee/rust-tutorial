use std::{usize, vec};
use std::iter::FromIterator;

//  inner iterotor
trait InIterator<T: Copy> {
    fn each<F: Fn(T) -> T>(&mut self,f: F);
}

impl<T: Copy> InIterator<T> for Vec<T> {
    fn each<F: Fn(T) -> T>(&mut self,f: F) {
        let mut i = 0;
        while i < self.len() {
            self[i] = f(self[i]);
            i += 1;
        }
    }
}

pub fn inner_iterator() {
    let mut vec = vec![1, 2, 3, 4];
    vec.each(|e| e * e);
    println!("{:?}", vec);
}

// outer iterator
pub fn out_iterator() {
    let vec: Vec<i32> = vec![1,2,3,4,5,];
    // equals for loop  
    let mut items= vec.iter();
    loop {
        match items.next() {
            Some(i) => println!("{}", i),
            None => break,
        }
    }

    let mut counter  = Counter{count: 0};
    counter.next();
    println!("{}, {:?}", counter.count, counter.size_hint());
    counter.next();
    println!("{}", counter.count);
    counter.next();
    println!("{}", counter.count);
    counter.next();
    println!("{}", counter.count);
}

struct Counter {
    count: usize,
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

// Iter: iter()  &self
// IntoIter: into_iter() self
// IterMut: iter_mut() &mut sef

// impl FromIterator
#[derive(Debug)]
struct MyVec (Vec<i32>);

impl MyVec {
    fn new() -> MyVec {
        MyVec(Vec::new())
    }

    fn add(&mut self, value:i32) {
        self.0.push(value);
    }
}

impl FromIterator<i32> for MyVec {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut vec = MyVec::new();

        for e in iter {
            vec.add(e);
        }
        vec
    }
}

// custome iteator adapter
#[derive(Debug, Clone)]
#[must_use = "reason"]
pub struct Step<T> {
    iter: T,
    skip: usize
}
impl<T> Iterator for Step<T> 
where T:Iterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.iter.next();
        if self.skip > 0 {
            self.iter.nth(self.skip - 1);
        }
        element
    }
}

pub trait ExtIteator: Iterator {
    fn step(self, n: usize) -> Step<Self>
    where Self:Sized + Iterator,
    {
        Step {
            iter: self,
            skip: n - 1,
        }
    }
}

impl<T: ?Sized + Iterator> ExtIteator for T {}
