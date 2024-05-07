fn main() {
    // When you declare a variable with let, 
    // it is immutable (cannot be changed).

    // This will not work
    // let my_number = 9;
    // my_number = 10;

    let mut my_number = 8;
    my_number = 10;

    let mut my_variable = 8; // it is now an i32. That can't be changed
    // my_variable = "Hello, world!"; // error here

    // Shadowing
    // Shadowing means using let to declare a new variable 
    // with the same name as another variable.
    let my_no = 8;
    println!("{}", my_no);
    let my_no = 9.2;
    println!("{}", my_no);

    let my_scope_number = 8;
    println!("{}", my_scope_number);
    {
        let my_scope_number = 9.2;
        println!("{}", my_scope_number)
    }
    println!("{}", my_scope_number);

    // times_two
    let final_number = {
        let y = 10;
        let x = 9;
        let x = times_two(x);
        let x = x + y;
        x 
    };
    println!("The number is now: {}",final_number)

    // Pretending we are using Rust without shadowing
    // let final_number = {
    //     let y = 10;
    //     let x = 9; // x starts at 9
    //     let x_twice = times_two(x); // second name for x
    //     let x_twice_and_y = x_twice + y; // third name for x!
    //     x_twice_and_y // too bad we didn't have shadowing - we could have just used x
    // };
    // println!("The number is now: {}", final_number)
}

fn times_two(number: i32) -> i32 {
    number * 2
}