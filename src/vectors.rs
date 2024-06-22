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
