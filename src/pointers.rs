struct Foo {
    bar: i32,
}

pub fn run() {
    // allocate
    let mut foo = Foo { bar: 10 };

    // mutable reference
    let bar = &mut foo;

    // change bar
    bar.bar = 20;

    // or change value by dereferencing bar directly
    let bar = &mut foo.bar;
    *bar = 30;

    println!("{}", *bar);
    println!("{}", foo.bar);

    // reference to number
    let number = 32;
    let number_ref = &number;

    // copy value of number to x
    let mut x = *number_ref;
    x = 10;

    // raw pointers
    let mut num: i32 = 10;
    let memloc_const = &num as *const i32 as usize;
    let memloc_mut = &mut num as *mut i32;

    // dereferencing
    let num: i32 = 10;
    let num_ref_ref_ref: &&&i32 = &&&num;
    let num_ref_ref: &i32 = **num_ref_ref_ref;
    let num: i32 = *num_ref_ref;

    // auto dereference using .
    let foo = Foo { bar: 10 };
    let foo_ref_ref_ref = &&&foo;
    let bar = foo_ref_ref_ref.bar;
    let bar = (***foo_ref_ref_ref).bar;
}
