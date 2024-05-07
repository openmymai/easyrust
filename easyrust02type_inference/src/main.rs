fn main() {
    // let small_number: u8 = 10;
    // let small_number = 10u8;
    let small_number = 10_u8;
    let big_number = 100_000_000_i32;

    // The _ does not change the number. It is only to make it easy for you to read. 
    // And it doesn't matter how many _ you use:
    let number = 0___________u8;
    let number2 = 1____6________2____4________i32;
    println!("{}, {}", number, number2);

    // float
    // let my_float= 5.;


    // let my_float: f64 = 5.0;
    // let my_other_float: f32 = 8.5;
    // let third_float = my_float + my_other_float; // error

    let my_float: f64 = 5.0;
    let my_other_float: f32 = 8.5;
    let third_float = my_float + my_other_float as f64;

    let my_second_float = 5.0;
    let my_other_second_float = 8.5;
    let third_second_float = my_second_float + my_other_second_float;

    // but now it knows that you need to add it to an f32. 
    // So it chooses f32.
    let my_third_float: f32 = 5.0;
    let my_third_other_float = 8.5;
    let third_other_third_float = my_third_float + my_third_other_float;

}
