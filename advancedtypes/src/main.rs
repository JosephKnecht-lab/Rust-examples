use std::fmt;
use std::io::Error;

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//     fn flush(&mut self) -> Result<(), Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
// }

type Result<T> = std::result::Result<T, std::io::Error>;  //the two traits are equvalant, since we create a new alias for Result T 


pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}


fn main() {
    
    type Kilometers = i32;  //type alliases Kilometers is i32 now

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;  //merge types into a single type Thunk

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    
    }


}

//function that never returns
fn bar() -> ! {
    // --snip--
}
