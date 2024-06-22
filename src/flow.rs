pub fn run() {
    // branching
    let x = 10;
    if x < 10 {
        println!("less than 10");
    } else if x == 10 {
        println!("equal to 10");
    } else {
        print!("greater than 10");
    }

    // loops
    let mut x = 0;
    loop {
        x += 1;
        if x < 10 {
            break;
        }
    }

    let mut x = 0;
    while x < 10 {
        x += 1;
    }

    for i in 0..5 {
        println!("{}", i);
    }

    for i in 0..=5 {
        println!("{}", i);
    }

    // match
    let x = 0i32;
    match x {
        0 => {
            println!("found zero");
        }
        1 | 2 => {
            println!("one or two");
        }
        3..=5 => {
            println!("between 3 and 5");
        }
        matched @ 6..=10 => {
            println!("matched number == {}", matched);
        }
        _ => {
            println!("no match");
        }
    }

    // value from block/expressions
    let x = {
        let a = 1;
        let b = 1;
        a + b
    };

    let x = if x < 0 { 1 } else { 0 };

    let mut x = 0;
    x = loop {
        x += 1;
        if x < 100 {
            break x;
        }
    };

    let x = match x {
        1..=10 => "1 to 10",
        _ => "no match",
    };
}
