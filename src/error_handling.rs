
fn func_returns_something_or_fail(i: i32) -> Result<i32, String> {
    if i == 0 {
        Ok(32)
    } else {
        Err(String::from("Error: i == 0"))
    }
}

fn error_handling() -> Result<(), String> {
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

    // there are also `expect` and `unwrap_or`

    println!("found {res}");
    Ok(())
}
