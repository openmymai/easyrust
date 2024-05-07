fn main() {
    // For mutable reference, you write &mut instead of &
    let mut my_number = 8; // i32
    let num_ref = &mut my_number; // &mut i32
    // add 10 to my_number, but you can't write num_ref += 10
    // because num_ref is not i32, it's &i32
    // use *, means "I don't want the reference, I want value."
    *num_ref += 10;
    println!("{}", my_number);
    
    // one * is the opposite of &. Also, one * erases one &.
    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Second_number = triple_reference? {}", second_number == ***triple_reference);
    // one * erases one &

    // using & is called "referencing", 
    // using * is called "dereferencing".

    // Rust Mutable and Immutable Rule
    // 
    // Rule 1: If you have only immutable references, 
    //     you can have as many as you want. 
    //     1 is fine, 3 is fine, 1000 is fine. No problem.
    // Rule 2: If you have a mutable reference, you can only have one. 
    //     Also, you can't have an immutable reference 
    //     and a mutable reference together.


    // Problem here !!!!
    // let mut number = 10;
    // let number_ref = &number; // immutable borrow
    // let number_change = &mut number; // mutable borrow
    // *number_change += 10;
    // println!("{}", number_ref); // immutable borrow use

    // Here's no problem, compiler smart
    let mut number = 10;
    let number_change = &mut number; // create a mutable reference
    *number_change += 10; // use mutable reference to add 10
    let number_ref = &number; // create an immutable reference
    println!("{}", number_ref); // print the immutable reference
    // Earlier in Rust this kind of code actually generated an error, 
    // but the compiler is smarter now. It can understand not just what we type, 
    // but how we use everything.

    // Shadowing
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country);
}
