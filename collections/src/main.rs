#[derive(Debug)]

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector v is {:?}",v);

    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    ];  

    println!("Row of Enums: {:?}", row);

    reading_vector();
    push_into_vector();
    itterate();
    itterate_add();
    intro_strings();

}

fn reading_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    println!("The third element is {}", &v[2]);

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);



    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn push_into_vector(){
    let mut v = vec![1, 2, 3, 4, 5];

    //let first = &v[0];
    let first = v[0]; //this fixes the reference borrow. Now it compiles


    v.push(6);  //error because it borrows the reference &v at first as immutable

    println!("The first element is: {}", first);
}

fn itterate(){
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn itterate_add(){
    let mut v = vec![11,22,33];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}

fn intro_strings(){
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    println!("String s:{}",s);

}