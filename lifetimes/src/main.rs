fn main() {
    let r;

        {
            let x = 5;
            r = &x; //x doesnt live outside the scope
        }

        println!("r: {}", r);
}
