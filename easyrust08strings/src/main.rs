fn main() {
    // 1.  &str is a simple string. 
    //     When you write let my_variable = "Hello, world!", 
    //     you create a &str. A &str is very fast.
    // 2.  String is a more complicated string. 
    //     It is a bit slower, but it has more functions. 
    //***  A String is a pointer, with data on the heap. ***

    // Both &str and String are UTF-8
    let name = "서태지";
    let other_name = String::from("Adrian Fahrenheit Țepeș");
    println!("{}, {}", name, other_name);

    let emoji_name = "😂";
    println!("My name is actually {}", emoji_name);

    // str is a dynamically sized type 
    // (dynamically sized = the size can be different)
    println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>()); // std::mem::size_of::<Type>() gives you the size in bytes of a type
    println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.", std::mem::size_of_val("서태지")); // std::mem::size_of_val() gives you the size in bytes of a variable
    println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));

    // There are many ways to make a String. Here are some:

    // 1.  String::from("This is the string text"); 
    //     This is a method for String that takes text and creates a String.
    // 2.  "This is the string text".to_string(). 
    //     This is a method for &str that makes it a String.
    // 3.  The format! macro. This is like println! 
    //     except it creates a String instead of printing.
    let my_name = "Billybrobby";
    let my_country = "USA";
    let my_home = "Korea";

    let together = format!(
        "I am {} and I come from {} but I live in {}.",
        my_name, my_country, my_home
    );

    // One other way to make a String is called .into()
    // with type declaration
    let my_string: String = "Try to make this a String".into();
    println!("{}", my_string);
}
