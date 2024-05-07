// There is an even shorter way to deal with Result (and Option), 
// shorter than match and even shorter than if let. 
// It is called the "question mark operator", and is just ?. 
// After a function that returns a result, you can add ?, This will:
// - return what is inside the Result if it is Ok
// - pass the error back if is Err

// We don't need to write std::result::Result 
// because Result is always "in scope" (in scope = ready to use). 

use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    // We aren't working with things like files yet, so the ? operator doesn't look too useful yet.
    // let parsed_number = input.parse::<u16>()?.to_string().parse::<u32>()?.to_string().parse::<i32>?;
    // Add a ? each time to check and pass it on
    Ok(parsed_number)

    // So using ? when panic and unwrap are good
}

fn prints_three_things(vector: &Vec<i32>) {
    if vector.len() != 3 {
        panic!("my_vec must always have three times");
    }
    println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn get_fourth(input: &Vec<i32>) -> i32 {
    // let fourth = input.get(3).unwrap();
    // unwrap() = expect(), and it's still panic but with message
    // let fourth = input.get(3).expect("Input vector needs at least 4 items");
    let fourth = input.get(3).unwrap_or(&0);
    *fourth
}

// fn try_two_unwrap(input: Vec<Option<i32>>) {
//     println!("Index 0 is: {}", input[0].unwrap());
//     println!("INdex 1 is: {}", input[1].unwrap());
// }
fn main() {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }

    

    let my_vec = vec![8, 9, 10];
    // add more than 3 to see panic
    // let my_vec = vec![8, 9, 10, 10, 55, 99];
    // prints_three_things(&my_vec);

    let fourth = get_fourth(&my_vec);
    println!("{}", fourth);

    // let vector = vec![None, Some(1000)]; // This vector has a None, so it will panic
    // try_two_unwrap(vector);
}