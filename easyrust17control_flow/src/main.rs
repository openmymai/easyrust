fn main() {
    let my_number = 5;
    if my_number == 7 {
        println!("It's seven");
    } else if my_number == 6 {
        println!("It's six");
    } else {
        println!("It's a different number");
    }


    let my_num = 5;
    if my_num % 2 == 1 && my_num > 0 {
        println!("It's a positive odd number");
    } else if my_num == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }

    let my_num_match: u8 = 5;
    match my_num_match {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        // without default it will ask for the remaining 3, 4, 5, ... 255 (u8)
        _ => println!("it's some other number"),
    }

    // You write match and then make a {} code block.
    // Write the pattern on the left and use a => fat arrow to say what to do when it matches.
    // Each line is called an "arm".
    // Put a comma between the arms (not a semicolon).

    let my_num_second_match = 5;
    let second_number = match my_num_second_match {
        0 => 0,
        5 => 10,
        _ => 2,
    };
    println!("second number is: {}", second_number);

    // match tuple
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's dark and unpleasant today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", "warm") => println!("It's dark but not bad"),
        _ => println!("Not sure what the weather is."),
    }

    // if inside match
    let children = 5;
    let married = true;
    match (children, married) {
        (children, married) if married == false => println!("Not married with {} children", children),
        (children, married) if children == 0 && married == true => println!("Married but no children"),
        _ => println!("Married? {}. Number of children: {}.", married, children),
    }

    // you can use _ as many times as you want in a match
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    match_colours(first);
    match_colours(second);
    match_colours(third);
}

// first also has not much green
// A match statement always stops when it finds a match, 
// and doesn't check the rest. 
// This is a good example of code that compiles well 
// but is not the code you want.

fn match_colours(rbg: (i32, i32, i32)) {
    match rbg {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, b, _) if b < 10 => println!("Not much blue"),
        (_, _, g) if g < 10 => println!("Not much green"),
        _ => println!("Each colour has at least 10"),
    }
}