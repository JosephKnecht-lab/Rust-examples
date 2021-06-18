
use std::io;
use std::fmt;
use std::collections::HashMap;


fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("Map is: {:?}", map);
}

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}