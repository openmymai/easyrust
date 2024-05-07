fn main() {
    // In the same way that we have &str and String, 
    // we have arrays and vectors.
    let name1 = String::from("Windy");
    let name2 = String::from("Gomesy");

    let mut my_vec = Vec::new();

    my_vec.push(name1);
    my_vec.push(name2);

    // Instead of using .push() to make Rust decide the type
    let mut my_vec_2: Vec<String> = Vec::new();

    // create a vector with the vec! macro
    let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let everything = &vec_of_ten[..];

    println!("Three to five: {:?}
        start at two: {:?}
        end at five: {:?}
        everything: {:?}", three_to_five, start_at_two, end_at_five, everything);
    
    // Because a vec is slower than an array, we can use some methods to make it faster. 
    // A vec has a capacity, which means the space given to the vector.
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());

    let my_vec_u8: Vec<u8> = [1, 2, 3].into();

    let my_vec_underscore: Vec<_> = [9, 0, 10].into();

    // So if you think you know how many elements you need, 
    // you can use Vec::with_capacity() to make it faster.
    // Ex. let num_vec = Vec::with_capacity(8);
}
