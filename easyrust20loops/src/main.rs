fn main() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("The counter is now: {}", counter);
        if counter == 5 {
            break;
        }
    }

    let mut count = 0;
    let mut count2 = 0;
    println!("Now entering the first loop.");

    'first_loop: loop {
        count += 1;
        println!("The counter is now: {}", count);
        if count > 9 {
            println!("Now entering the second loop.");
            'second_loop: loop {
                println!("The second counter is now: {}", count2);
                count2 += 1;
                if count2 == 3 {
                    break 'first_loop;
                }
            }
        }
    }

    let mut while_count = 0;
    while while_count < 5 {
        while_count += 1;
        println!("The counter is now: {}", while_count);
    }

    for number in 0..3 {
        println!("The number is: {}", number);
    }
    for number in 0..=3 {
        println!("The next number is: {}", number);
    }

    for _ in 0..3 {
        println!("Printing the same thing three times");
    }

    let mut loop_count = 5;
    let my_number = loop {
        loop_count += 1;
        if loop_count % 53 == 3 {
            break loop_count;
        }
    };
    println!("{}", my_number);

    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);

    match_colours(first);
    match_colours(second);
    match_colours(third);
}

fn match_colours(rbg: (i32, i32, i32)) {
    println!("Comparing a colour with {} red, {} blue, and {} green:", rbg.0, rbg.1, rbg.2);
    let new_vec = vec![(rbg.0, "red"), (rbg.1, "blue"), (rbg.2, "green")];
    let mut all_have_at_least_10 = true;
    for item in new_vec {
        if item.0 < 10 {
            all_have_at_least_10 = false;
            println!("Not much {}.", item.1)
        }
    }
    if all_have_at_least_10 {
        println!("Each colour has at least 10.")
    }

    println!();
}
