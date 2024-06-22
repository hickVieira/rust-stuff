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
}
