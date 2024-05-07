fn main() {
    prints_number(56);
}

fn prints_number(input: i32) {
    assert_eq!(input % 2, 0);   // number must be equal.
                                // if number % 2 is not 0, it panics
    println!("The number is not odd. It is {}", input);
}
