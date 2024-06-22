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

fn custom_lifetime<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.bar;
}

fn multiple_lifetimes<'a, 'b>(fooa: &'a Foo, foob: &'b Foo) -> &'b i32 {
    println!("{} {}", fooa.bar, foob.bar);
    return &foob.bar;
}

struct Bar<'a> {
    bar: &'a i32,
}

fn ownership() {
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

    // custom lifetime
    let bar = custom_lifetime(&foo);

    // foo gets copied to parameter
    copy_to_parameter(foo);

    // multiple lifetimes
    {
        let fooa = Foo { bar: 10 };
        let foob = Foo { bar: 20 };
        let bar = multiple_lifetimes(&fooa, &foob);
        // fooa gets dropped
        print!("{}", foob.bar);
        // bar gets dropped
        // foob gets dropped
    }

    // reference to number
    {
        let number = 32;
        let number_ref = &number;

        // copy value of number to x
        let mut x = *number_ref;
        x = 10;
    }

    // static lifetimes
    {
        // static variables are compile-time memory resource
        static SECRET: &str = "a compile-time static string";

        // static lifetimes however, are static runtime memory resources
        let foo: &'static Foo = &Foo { bar: 10 };
        // foo never drops

        // string literals are 'static by default
        let msg: &'static str = "a lifetimed static string";
        let msg = "all string literals are 'static timed by default actually";

        // lifetime inside data types
        let x = 45;
        let bar = Bar { bar: &x };
    }
}