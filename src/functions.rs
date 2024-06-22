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

pub fn run()
{
    let x = add(1, 2);
    let y = subtract(2, 1);
    let z = swap(1, 2);
    let a = nothing1();
    let b = nothing2();
}