use std::fmt::{Debug, Display};
use std::cmp::PartialOrd;

// fn return_number(number: i32) -> i32 {
//     println!("Here is your number.");
//     number
// }

// Generics means "maybe one type, maybe another type".

fn return_number<T>(number: T) -> T {
    println!("Here is your number.");
    number
}

fn return_number_mytype<MyType>(number: MyType) -> MyType {
    println!("Here is your number from MyType.");
    number
}

// You will remember that some types in Rust are Copy, some are Clone, 
// some are Display, some are Debug, and so on. 
// With Debug, we can print with {:?}. 
// So now you can see that we have a problem if we want to print T

// fn print_number<T>(number: T) {
//     println!("Here is your number: {}", number);
// }

// T doesn't implement Debug. So do we implement Debug for T?
// No, because we don't know what T is.
// But we can tell the function: "Don't worry, 
// because any type T for this function will have Debug"
fn print_number<T: Debug>(number: T) {
    println!("Here is your number: {:?}", number);
}

#[derive(Debug)]
struct Animal {
    name: String,
    age: u8,
}

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {:?}", item);
}

// Sometimes we need more than one type in a generic function.

fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

// using where

fn compare_and_display_where<T, U>(statement: T, num_1: U, num_2: U) 
where
    T: Display,
    U: Display + PartialOrd,
{
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}

// If you have one type T and another type T, they must be the same.
// If you have one type T and another type U, they can be different. -
// But they can also be the same.

fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
    println!("I have two things to say: {} and {}", statement_1, statement_2);
}

fn main() {
    let number = return_number(5);
    let number_mytype = return_number_mytype(5);

    print_number(5);

    let charlie = Animal {
        name: "Charlie".to_string(),
        age: 1,
    };

    let number_item = 55;
    print_item(charlie);
    print_item(number_item);

    compare_and_display("Listen up!", 9, 8);
    
    compare_and_display_where("Listen up!", 9, 8);

    say_two("Hello there!", String::from("I hate sand.")); // T->&str, U->String
    say_two(String::from("Where is Padme?"), String::from("Is she all right?")); // T&U are String
}