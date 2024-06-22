struct Foo {
    bar: i32,
}

fn copy_to_parameter(foo: Foo) {
    println!("{}", foo.bar);
}

fn mutable_ref(foo: &mut Foo) {
    foo.bar = 10;
    println!("{}", foo.bar);
}

fn ref_to_ref(foo: &Foo) -> &i32 {
    return &foo.bar;
}

pub fn run() {
    // binding instance of foo
    let mut foo = Foo { bar: 10 };

    // modify bar by ref
    let x = &mut foo.bar;
    // deref
    *x = 10;
    // x is dropped here - allowing us to mut ref it again
    let y = &mut foo.bar;

    // ref to foo
    let foo_ref = &foo;

    // foo gets referenced to parameter
    mutable_ref(&mut foo);
    mutable_ref(&mut foo);
    mutable_ref(&mut foo);

    // ref of ref
    let bar_ref = ref_to_ref(&foo);

    // foo gets copied to parameter
    copy_to_parameter(foo);
}