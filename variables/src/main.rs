fn main() {

    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    shadowing();
    spaces();
}

fn shadowing(){
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x); 
}

fn spaces(){
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces); 

}