struct Bar<'a> {
    bar: &'a i32,
}

struct Foo<'a> {
    bar: &'a Bar<'a>,
}

fn func<'a>(x: &'a mut i32) -> &'a i32 {
    *x = 25;
    return x;
}

pub fn run() {
    // lifetime annotations
    let x = 10;
    let bar = Bar { bar: &x };
    let foo = Foo { bar: &bar };
    let mut x: i32 = 10;
    let a = func(&mut x);

    println!("{}", a);

    // static lifetimes

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
    let bar = Bar { bar: &x };
}
