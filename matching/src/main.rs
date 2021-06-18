// #[derive(Debug)] // so we can inspect the state in a minute

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let myCoin = Coin::Quarter(UsState::Alaska);
    let value: u8 = value_in_cents(myCoin);
    println!("Value is : {}", value);
}