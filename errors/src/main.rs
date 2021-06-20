use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;


fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    unwrap_test();
    expect_test();
    let r = read_username_from_file();
    println!("Result is is: {:?}",r);

}

fn unwrap_test() {
    let f = File::open("hello.txt").unwrap();
    println!("f is: {:?}",f);
}

fn expect_test(){
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("f is: {:?}",f);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}