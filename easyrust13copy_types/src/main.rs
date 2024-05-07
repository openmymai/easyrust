fn main() {
    // Some types in Rust are very simple. They are called copy types. 
    // These simple types are all on the stack, and the compiler knows their size. 
    // That means that they are very easy to copy, so the compiler always copies 
    // when you send it to a function. It always copies because they are so small 
    // and easy that there is no reason not to copy. 
    // So you don't need to worry about ownership for these types.
    
    // These simple types include: 
    // integers, floats, booleans, and char
    // See it from the document, Trait implementations
    // - is copied when you send it to a function(Copy)
    // - can use {} to print(Display)
    // - can use {:?} to print(Debug)

    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words ");
        get_length(my_string.clone());
    }

    let mut my_string = String::new();
    for _ in 0..50 {
        my_string.push_str("Here are some more words ");
        get_length_ref(&my_string);
    }

    // variable without values
    let my_number;
    {
        let number = {
            57
        };
        my_number = loop_then_return(number);
    }
    println!("{}", my_number)
}

fn get_length(input: String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

fn get_length_ref(input: &String) {
    println!("It's {} words long.", input.split_whitespace().count());
}

fn loop_then_return(mut counter: i32) -> i32 {
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}