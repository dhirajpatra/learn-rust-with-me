use std::collections::HashMap;
use std::fmt;
use std::io::Result as IoResult;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    function1().unwrap();
    function2().unwrap();
}

// A formatting-related function
fn function1() -> fmt::Result {
    Ok(())
}

// An IO-related function
fn function2() -> IoResult<()> {
    Ok(())
}
