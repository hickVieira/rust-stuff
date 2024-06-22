struct Foo<'a> {
    bar: &'a i32,
}

fn single_lifetime<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.bar;
}

fn multiple_lifetimes<'a, 'b>(fooa: &'a Foo, foob: &'b Foo) -> &'b i32 {
    println!("{} {}", fooa.bar, foob.bar);
    return &foob.bar;
}

pub fn run() {
    // lifetime annotations
    let bar = 10;
    let fooa = Foo { bar: &bar };
    let foob = Foo { bar: &bar };

    let c = single_lifetime(&fooa);
    let m = multiple_lifetimes(&fooa, &foob);
    // fooa gets dropped
    print!("{}", foob.bar);
    // bar gets dropped
    // foob gets dropped

    // static variables are compile-time memory resource
    static SECRET: &str = "a compile-time static string";

    // static lifetimes however, are static runtime memory resources
    let foo: &'static i32 = &5;
    // foo never drops

    // string literals are 'static by default
    let msg: &'static str = "a lifetimed static string";
    let msg = "all string literals are 'static timed by default actually";

    // lifetime inside data types
    let x = 45;
    let bar = Foo { bar: &x };
}
