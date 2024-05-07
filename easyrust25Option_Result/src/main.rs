// You use Option when you have a value that might exist, or might not exist. 
// When a value exists it is Some(value) and when it doesn't it's just None.

// fn take_fifth(value: Vec<i32>) -> i32 {
//     value[4]
// }

// Panic means that the program stops before the problem happens. 

use core::num;

// So now we will change the return type from i32 to Option<i32>. 
fn take_fifth(value: &Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

// use .unwrap() to get value, but .unwrap() None will panic
fn handle_option(my_option: &Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {}!", number),
            None => println!("Found a None!"),
        }
    }
}

// Compare Option and Result signature

// enum Option<T> {
//     None,
//     Some(T),
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Result

fn give_result(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(())
    } else {
        return Err(())
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five".to_string()),
    }
}

fn main() {
    // let new_vec = vec![1, 2];
    // let index = take_fifth(new_vec);
    // When we run the code, it panics.
    // index out of bound

    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    // println!("{:?}, {:?}", take_fifth(new_vec), take_fifth(bigger_vec));

    let mut option_vec = Vec::new();
    option_vec.push(take_fifth(&new_vec));
    option_vec.push(take_fifth(&bigger_vec));

    handle_option(&option_vec);

    // using .is_some() and .is_none() method, no need handle_option
    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
        let inside_number = take_fifth(&vec);
        if inside_number.is_some() {
            println!("We got: {}", inside_number.unwrap()); // now it's safe using .unwrap()
        } else {
            println!("We got nothing");
        }
    }

    // Result
    if give_result(5).is_ok() {
        println!("It's okay, guys")
    } else {
        println!("It's an error, guys")
    }

    let mut result_vec = Vec::new();

    for number in 2..7 {
        result_vec.push(check_if_five(number));
    }

    println!("{:?}", result_vec);

    // Using a match with Option and Result sometimes requires a lot of code. 
    // For example, the .get() method returns an Option on a Vec.
    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(0);
    let get_two = my_vec.get(10);
    println!("{:?}", get_one);
    println!("{:?}", get_two);

    // range 0 to 10 
    for index in 0..10 {
        match my_vec.get(index) {
            Some(number) => println!("The number is: {}", number),
            None => {}
        }
    }

    // it's good but we don't do anything for "None"
    // we can make the code smaller using "if let"
    // "if let" = do something if it matches, and don't do anything if it doesn't
    for index in 0..10 {
        if let Some(number) = my_vec.get(index) {
            println!("The number is: {}", number);
        }
    }

    // "while let" is like a  while loop for "if let"
    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the city of {}:", city[0]);
        while let Some(information) = city.pop() {
            if let Ok(number) = information.parse::<i32>() {
                println!("The number is: {}", number);
            }
        }
    }
}
