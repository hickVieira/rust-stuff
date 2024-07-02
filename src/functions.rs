fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    return (b, a);
}

fn nothing1() {
    return ();
}

fn nothing2() {}

trait MyTrait {
    fn MyMethod(&self) -> &String;
}

fn generic_function<T>(trait_object: T)
where
    T: MyTrait,
{
    println!("This is a generic function {}", trait_object.MyMethod());
}

fn generic_function_shorthand1<T: MyTrait>(trait_object: T) {
    println!(
        "This is a generic function shorthand {}",
        trait_object.MyMethod()
    );
}

fn generic_function_shorthand2(trait_object: impl MyTrait) {
    println!(
        "This is a generic function shorthand {}",
        trait_object.MyMethod()
    );
}

pub fn run() {
    let x = add(1, 2);
    let y = subtract(2, 1);
    let z = swap(1, 2);
    let a = nothing1();
    let b = nothing2();
}
