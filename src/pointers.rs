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
}
