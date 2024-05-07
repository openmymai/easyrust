fn main() {
    // Tuples in Rust use (). We have seen many empty tuples already, 
    // because nothing in a function actually means an empty tuple:
    //
    // fn do_something() {}
    // 
    // is actually short for:
    // fn do_something() -> () {}
    // |_> that function gets nothing (an empty tuple) 
    // and returns nothing (an empty tuple)
    
    let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
    println!(
        "Inside the tuple is: First item: {:?}
        Second item: {:?}
        Third item: {:?}
        Fourth item: {:?}
        Fifth item: {:?}
        Sixth item: {:?}",
            random_tuple.0,
            random_tuple.1,
            random_tuple.2,
            random_tuple.3,
            random_tuple.4,
            random_tuple.5,
    );

    let str_vec = vec!["one", "two", "three"];

    let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{:?}", b);

    let (_, _, variable) = (str_vec[0], str_vec[1], str_vec[2]);
    println!("{:?}", variable);
}

fn just_prints() {
    println!("I am printing"); // Adding ; means we return an empty tuple.
}
