fn main() {
    // Rust use references to make sure that all memory access is safe.
    // We use & to create a references.
    let country = String::from("Austria");
    // "reference to a String".
    let ref_one = &country;
    let ref_two = &country;

    println!("{}", ref_one);

    // We could create three references 
    // or one hundred references to country 
    // and it would be no problem.

    let ct = return_str();
}

fn return_str() -> &str {
    //              ^ expected named lifetime parameter
    let country = String::from("Austria");
    let country_ref = &country;
    country_ref
}

// The function return_str() creates a String, 
// then it creates a reference to the String. 
// Then it tries to return the reference. 
// But the String country only lives inside the function, 
// and then it dies. Once a variable is gone, 
// the computer will clean up the memory 
// and use it for something else. 
// So after the function is over, 
// country_ref is referring to memory that is already gone, 
// and that's not okay. Rust prevents us from making a mistake with memory here.