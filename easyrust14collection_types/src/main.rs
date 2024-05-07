fn main() {
    let array1 = ["One", "Two"]; // this one is type [&str; 2]
    let array2 = ["One", "Two", "Five"]; // this one is type [&str: 3], different type

    let seasons = ["Spring", "Summer", "Autumn", "Winter"];
    let seasons2 = ["Spring", "Summer", "Fall", "Autumn", "Winter"];
    // seasons.ddd(); // error method not found in [&str; 4]

    let my_array = ["a"; 10]; // 10 a
    println!("{:?}", my_array); 

    let my_number = [0, 10, -20];
    println!("{}", my_number[1]);

    let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
     
    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let everything = &array_of_ten[..];

    println!("Three to five: {:?},\nstart at two: {:?},\nend at five: {:?},\neverything: {:?}", three_to_five, start_at_two, end_at_five, everything);
}
