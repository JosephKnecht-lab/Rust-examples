use std::collections::HashMap;


fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);


    println!("Scores: {:?}",scores);
    println!("Score: {:?}",score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    create();
    ownership();
    entry_value();

}

fn create(){

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![100, 500];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

        println!("Scores: {:?}",scores);

}

fn ownership(){

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}

fn entry_value(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}