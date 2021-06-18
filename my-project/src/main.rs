
// use std::io::Result as IoResult;
use std::fmt::Result;
use std::collections::HashMap;
use std::io::{self, Write};

// use std::{cmp::Ordering, io};


use rand::Rng;


fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    println!("Map is: {:?}", map);

    let secret_number = rand::thread_rng().gen_range(1..101);

}

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }