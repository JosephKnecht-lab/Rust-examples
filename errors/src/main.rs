use std::fs::File;
use std::io::ErrorKind;


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
}

fn unwrap_test() {
    let f = File::open("hello.txt").unwrap();
    println!("f is: {:?}",f);
}

fn expect_test(){
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("f is: {:?}",f);
}