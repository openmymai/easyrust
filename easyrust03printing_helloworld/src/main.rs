// fn means function,
// main is the function that starts the program
// () means that we didn't give the function any variables to start.
// {} is called a code block. This is the space where code lives.
fn main() {
    println!("Hello, world!");

    println!("Hello, world number {}!", 8);

    println!("Hello, world number {}!", number());

    multiply(8, 9);
    let some_number = 10;
    let some_other_number = 2;
    multiply(some_number, some_other_number);

    // {
    //     let my_number = 8;
    // }
    // println!("Hello, number {}", my_number);
    // there is no my_number and println!() can't find it

    let my_number = {
        let second_number = 8;
        second_number + 9 // it's work just like a function
    }
    println!("My number is {}", my_number);
}

fn number() -> i32 {
    8
}

fn multiply(number_one: i32, number_two: i32) {
    let result = number_one * number_two;
    println!("{} times {} is {}", number_one, number_two, result);
}

// If it had a ;, it would not return anything (it would return a ())
// Rust will not compile this if it has a ;, 
// because the return is i32 and ; returns (), not i32:
// fn number() -> i32 {
//     8; // error
// }

