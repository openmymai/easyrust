// Rc means "reference counter".
// You know that in Rust, every variable can only have one owner. 
// That is why this doesn't work:

// fn takes_a_string(input: String) {
//     println!("It is: {}", input)
// }

// fn also_take_a_string(input: String) {
//     println!("It is: {}", input)
// }

// fn main() {
//     let user_name = String::from("User MacUserson");

//     // takes_a_string(user_name); // -> suggest to use .clone() here
//     // also_take_a_string(user_name);
// }


use std::rc::Rc;

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}

fn main() {
    let calgary = City {
        name: "Calgary".to_string(),
        population: 1_200_000,
        // pretend that this string is very very long
        city_history: Rc::new("Calgary began as a fort called Fort Calgary that...".to_string()),
    };
    let cananda_cities = CityData {
        names: vec![calgary.name],
        histories: vec![calgary.city_history.clone()], // clone to increase the count
    };
    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));
    let new_owner = calgary.city_history.clone();
}