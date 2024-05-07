// Box is very useful for returning traits. 
// You know that you can write traits in generic functions.


// This only takes something with Display, 
// so it can't accept our struct DoesntImplementDisplay.
// But it can take in a lot of others like String.

use core::num;
use std::fmt::Display;

struct DoesntImplementDisplay {}

// You also saw that we can use impl Trait to return other traits, or closures.
// Box can be use in similar way.

fn displays_it<T: Display>(input: T) {
    println!("{}", input);
}

#[allow(dead_code)] // Tell the compiler to be quiet
use std::mem::size_of;

trait JustATrait {} // We will implement this on everything

enum EnumOfNumbers {
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}
impl JustATrait for EnumOfNumbers {}

struct StructOfNumbers {
    an_i8: i8,
    another_i8: i8,
    one_more_i8: i8,
}
impl JustATrait for StructOfNumbers {}

enum EnumOfOtherTypes {
    I8(i8),
    AnotherI8(i8),
    Collection(Vec<String>),
}
impl JustATrait for EnumOfOtherTypes {}

struct StructOfOtherTYpes {
    an_i8: i8,
    another_i8: i8,
    a_collection: Vec<String>,
}
impl JustATrait for StructOfOtherTYpes {}

struct ArrayAndI8 {
    array: [i8; 1000],
    an_i8: i8,
    in_u8: u8,
}
impl JustATrait for ArrayAndI8 {}


use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne {}

impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!")
    }
}

#[derive(Debug)]
struct ErrorTwo;

impl Error for ErrorTwo {}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

// Make a function that just returns a String or an error
fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> { // With Box<dyn Error> you can return anything that has the Error trait
    match input {
        0 => Err(Box::new(ErrorOne)), // Don't forget to put it in a box
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me".to_string()), // This is the success type
    }
}
fn main() {
    println!(
        "{}, {}, {}, {}, {}",
        size_of::<EnumOfNumbers>(),
        size_of::<StructOfNumbers>(),
        size_of::<EnumOfOtherTypes>(),
        size_of::<StructOfOtherTYpes>(),
        size_of::<ArrayAndI8>(),
    );

    let vec_of_u8s = vec![0_u8, 1, 80]; // Three number to try out

    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }
}

// fn returns_just_a_trait() -> JustATrait { // doesn't have a size known at compile time
//     let some_enum = EnumOfNumbers::I8(8);
//     some_enum
// }

// Here we also add the keyword dyn. dyn is a word that shows you 
// that you are talking about a trait, not a struct or anything else.

// And now it works, because on the stack is just a Box and we know the size of Box.
// fn returns_just_a_trait() -> Box<dyn JustATrait> {
//     let some_enum = EnumOfNumbers::I8(8);
//     Box::new(some_enum)
// }
