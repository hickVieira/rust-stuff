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

    // raw pointers
    {
        let bytes: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

        let raw_ptr = bytes.as_ptr();
        let raw_ptr = &bytes as *const u8;

        let ptr_usize = raw_ptr as usize;
        let ptr_f32 = ptr_usize as *const f32;
        let ptr_u32 = ptr_usize as *const u32;

        let val_f32 = unsafe { *ptr_f32 };
        let val_u32 = unsafe { *ptr_u32 };

        println!("should print small float : {}", val_f32);
        println!("should print small uint : {}", val_u32);
    }
}
