use std::collections::HashMap;


fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}",scores);

    create();

}

fn create(){

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![100, 500];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

        println!("Scores: {:?}",scores);


}