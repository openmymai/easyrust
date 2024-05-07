fn main() {
    // References are very useful for functions. 
    // The rule in Rust on values is: a value can only have one owner.
    let country = String::from("Austria");
    // print_country(country); // after print, String is now dead.
    // print_country(country); // error here
    // let country = print_country2(country); // use let to get the String back
    // print_country2(country);

    // This says "you can look at it, but I will keep it".
    print_country3(&country);
    print_country3(&country);

    let mut country = String::from("Austria");
    add_hungary(&mut country);

    // Conclusion

    // - fn function_name(variable: String) takes a String and owns it.
    // - fn function_name(variable: &String) borrows a String and can look at it
    // - fn function_name(variable: &mut String) borrows a String and can change it
}

fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn print_country2(country_name: String) -> String {
    println!("{}", country_name);
    country_name
}

// much better way to fix by adding &
fn print_country3(country_name: &String) {
    println!("{}", country_name);
}

fn add_hungary(country_name: &mut String) {
    country_name.push_str("-Hungary");
    println!("Now it says: {}", country_name);
}