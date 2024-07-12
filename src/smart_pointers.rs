use std::ops::{Add, Deref};

struct MyBox<T> {
    data: T,
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        return &self.data;
    }
}

pub fn run() {
    let mybox = MyBox { data: 10 };

    // derefing returns .data
    let x = *mybox;
    
    // calling method also derefs
    let y = mybox.add(1);

    println!("should print 10 : {}", x);
    println!("should print 11 : {}", y);
}
