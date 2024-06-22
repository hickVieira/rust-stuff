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