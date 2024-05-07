// impl Trait is similar to generics. You remember that generics use a type T (or any other name) 
// which then gets decided when the program compiles.

// fn gives_higher_i32(one: i32, two: i32) {
//     let higher = if one > two { one } else { two };
//     println!("{} is higher", higher);
// }

// fn main() {
//     gives_higher_i32(8, 10);
// }


// But this only takes i32, so now we will make it generic. 
// We need to compare and we need to print with {}, 
// so our type T needs PartialOrd and Display. Remember, 
// this means "only take types that already have PartialOrd and Display".

use std::fmt::Display;

fn gives_higher_i32<T: PartialOrd + Display>(one: T, two: T) {
    let higher = if one > two { one } else { two };
    println!("{} is higher.", higher);
}

// Now let's look at impl Trait, 
// which is similar. Instead of a type T, 
// we can bring in a type impl Trait. 
// Then it will take in a type that implements that trait.
// It is almost the same:
fn prints_it(input: impl Into<String> + std::fmt::Display) {
    println!("You can print many things, including {}", input);
}

// So we can do the same thing to return a closure. 
// To return a closure, use impl and then the closure signature. 
// Once you return it, you can use it just like a function.

fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number| {
            number *= 2;
            println!("Doubling number. Now it is {}", number);
            number
        },
        "triple" => |mut number| {
            number *= 40;
            println!("Tripling number. Now it is {}", number);
            number
        },
        _ => |number| {
            println!("Sorry, it's the same: {}.", number);
            number
        },
    }
}

enum TimeOfDay {
    Dawn,
    Day,
    Sunset,
    Night,
}

fn change_fear(input: TimeOfDay) -> impl FnMut(f64) -> f64 {
    use TimeOfDay::*;

    match input {
        Dawn => |x| {
            println!("The morning sun has vanquished the horrible ngiht. You no longer feel afraid.");
            println!("Your fear is now {}", x * 0.5);
            x * 0.5
        },
        Day => |x| {
            println!("What a nice day. Maybe put your feet up and rest a bit.");
            println!("Your fear is now {}", x * 0.2);
            x * 0.2
        },
        Sunset => |x| {
            println!("The sun is almost down! This is no good.");
            println!("Your fear is now {}", x * 1.4);
            x * 1.4
        },
        Night => |x| {
            println!("What a horrible night to have a curse.");
            println!("Your fear is now {}", x * 5.0);
            x * 5.0
        },
    }
}

fn main() {
    gives_higher_i32(8, 10);

    // impl trait
    let name = "Tuon";
    let string_name = String::from("Tuon");
    prints_it(name);
    prints_it(string_name);

    // return closure

    let my_number = 10;
    // Make three closure
    let mut doubles = returns_a_closure("double");
    let mut triples = returns_a_closure("triple");
    let mut quadruples = returns_a_closure("quadruple");

    doubles(my_number);
    triples(my_number);
    quadruples(my_number);

    // Time of Day
    use TimeOfDay::*;
    let mut character_fear = 10.0;

    let mut daytime = change_fear(Day);
    let mut sunset = change_fear(Sunset);
    let mut night = change_fear(Night);
    let mut morning = change_fear(Dawn);

    character_fear = daytime(character_fear);

    character_fear = sunset(character_fear);
    character_fear = night(character_fear);
    character_fear = morning(character_fear);
}