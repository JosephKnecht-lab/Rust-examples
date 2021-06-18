
use std::io::Result as IoResult;
use std::fmt::Result;
use std::collections::HashMap;


fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("Map is: {:?}", map);
}

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}