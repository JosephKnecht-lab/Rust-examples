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

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Plus one values: {:?}", six);


    let some_u8_value = 7;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    
    
}