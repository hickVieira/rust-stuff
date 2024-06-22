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
