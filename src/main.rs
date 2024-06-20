#![allow(dead_code)] // this line prevents compiler warnings

fn main() {
    // variables
    let x = 1;
    let x = 1u32;
    let x = 1;
    let x: i32 = 1;
    let mut x = 0;
    x = 1;

    // data types
    let type_bool: bool = true;
    let type_u8: u8 = 1;
    let type_u16: u16 = 1;
    let type_u32: u32 = 1;
    let type_u64: u64 = 1;
    let type_u128: u128 = 1;

    let type_i8: u8 = 1;
    let type_i16: u16 = 1;
    let type_i32: u32 = 1;
    let type_i64: u64 = 1;
    let type_i128: u128 = 1;

    let type_f32: f32 = 1.0f32;
    let type_f64: f64 = 1.0;

    let type_usize: usize = 1;
    let type_isize: isize = 1;

    let type_char: char = '1';

    let type_tuple: (u32, f32) = (1, 1.0f32);

    let type_array: [u32; 3] = [1, 1, 1];

    let type_str: &str = "1";

    // casting
    let x = 1.2345;
    let x = x as u32;
}

// constants
const PI: f32 = 3.14159;
const GOLDEN_RATIO: f32 = 1.618_033_988_75;

// functions
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

fn flow() {
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

// structs
struct Character {
    name: String,
    arms: u32,
    legs: u32,
    weapon: String,
    health: u32,
    stamina: u32,
    mana: u32,
}

struct float2(f32, f32);
struct float3(f32, f32, f32);
struct NoDataStruct;

fn structs() {
    let s = String::from("Hello world"); // static methods use ::
    let s_len = s.len(); // instance methods use .

    let loc1 = float2(1.0, 2.0);
    let loc2 = float3(1.0, 2.0, 3.0);
}

// enums
enum Species {
    Human,
    Crab,
    Octopus,
    Fish,
    Clam,
}

enum PoisonType {
    Acid,
    Poison,
    Panic,
}

enum Size {
    Small,
    Medium,
    Big,
}

enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None,
}

struct SeaCreature {
    species: Species,
    names: String,
    arms: u32,
    legs: u32,
    weapon: Weapon,
}

fn enums() {
    let Ferris = SeaCreature {
        species: Species::Crab,
        names: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(32, Size::Small),
    };

    match Ferris.species {
        Species::Crab => {
            println!("{} is a crab", Ferris.names);
            match Ferris.weapon {
                Weapon::Claw(num_claws, size_claws) => {
                    let size_desc = match size_claws {
                        Size::Small => "small",
                        Size::Medium => "medium",
                        Size::Big => "big",
                    };
                }
                _ => {}
            }
        }
        Species::Octopus => println!("{} is an octopus", Ferris.names),
        Species::Fish => println!("{} is a fish", Ferris.names),
        Species::Clam => println!("{} is a clam", Ferris.names),
        _ => println!("{} is a human", Ferris.names),
    }
}

// generics
struct Something<T> {
    item: Option<T>,
}

fn generics() -> Result<(), String> {
    let something_u32 = Something::<i32> { item: Some(1) };
    let something_string = Something::<String> {
        item: Some(String::from("hello")),
    };
    let something_something_f32 = Something::<Something<f32>> {
        item: Some(Something { item: Some(1.0f32) }),
    };
    let something_inferred = Something { item: Some(1) };

    if something_u32.item.is_some() {
        println!("there is something in something_u32");
    } else if something_u32.item.is_none() {
        println!("there is nothing in something_u32");
    }

    match something_u32.item {
        Some(v) => println!("there is {v} in something_u32"),
        None => {
            println!("there is nothing in something_u32");
            return Err(String::from("Error: something_u32 is None"));
        }
    }

    return Ok(());
}

fn func_returns_something_or_fail(i: i32) -> Result<i32, String> {
    if i == 0 {
        Ok(32)
    } else {
        Err(String::from("Error: i == 0"))
    }
}

fn error_handling_question_mark() -> Result<(), String> {
    // equivalent code - used when error is acceptable
    let res = func_returns_something_or_fail(0)?;
    let res = match func_returns_something_or_fail(0) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    // equivalent code - used when error is not acceptable
    let res = func_returns_something_or_fail(0).unwrap();
    let res = match func_returns_something_or_fail(0) {
        Ok(v) => v,
        Err(e) => panic!("Error generated by rust {e}"),
    };
    let res = match Some(String::from("some object")) {
        Some(v) => v,
        None => return panic!("some null error message"),
    };

    println!("found {res}");
    Ok(())
}

fn vectors() {
    // calling Vec::new()
    let mut vec = Vec::<i32>::new();
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // calling vec! macro
    let mut vec = vec![1, 2, 3];
    let mut vec = vec![1, 2, 3, 4, 5];
    let mut vec = vec![String::from("Hello"), String::from("World")];

    // looping
    for i in vec.iter() {
        println!("{i}");
    }
}

// ownership
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

// text

fn text() {
    // raw strings like these are always &'static str - utf-8 encoded
    let msg: &'static str = "Hello World";
    let msg = "Ferris says:\t\"Hello\"\n";

    // strings are multi-line by default
    let msg = "This
    is a
    string
    with
    newlines
    but not
    this one\
    ";

    // raw string literals usgin r#" text "#
    let msg = r#"
        <div class="advice">
            Raw strings are useful for some situations.
        </div>
    "#;

    // include text from local files
    let hello_txt = include_str!("hello.txt");

    // string slices
    {
        let a = "hi 🦀"; // has 7 bytes - 1h 1i 1/space 4/crab
        let aLen = a.len(); // equals 7 - size in bytes, not in graphemes
        let first_word = &a[0..2]; // [>= 0 < 2]
        let second_word = &a[3..7]; // [>= 3 < 7] - jumps space

        // Rust does not accept slices of invalid unicode characters
        let half_crab = &a[3..5]; // FAILS
    }

    // chars
    {
        let chars = "hi 🦀".chars().collect::<Vec<char>>();
        let charsLen = chars.len(); // equals 4 - size in graphemes
        let crab = chars[3] as u32; // equals 🦀 since chars are 4 bytes
    }

    // Strings
    let mut text = String::from("hello");
    // theses are not str
    // unlike string literals, Strings are heap allocated
    text.push_str(" world");
    text = text + "!";

    // text as parameters
    {
        fn say_it_loud(str: &str) {
            println!("{}!!!!", str.to_string().to_uppercase());
        }

        say_it_loud("asdasd sa das");
        say_it_loud(stringify!("jajajaj", " ", 3, 4, 5));
        say_it_loud(&String::from("aopkpapaokapk"));
    }

    // building strings
    {
        let text = ["a", "b", "c"].concat();
        let text = ["a", "b", "c"].join(",");
        let text = format!("{}-{}-{}", "a", "b", "c");
        let a = 'a';
        let b = 'b';
        let c = 'c';
        let text = format!("{a}-{b}-{c}");
    }

    // parse text
    fn parse_text() -> Result<(), std::num::ParseIntError> {
        let a = 42;
        let a_string = a.to_string();
        let b = a_string.parse::<i32>()?;

        Ok(())
    }
}
