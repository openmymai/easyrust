fn main() {
    // for _ in 0..10 {
    //     std::thread::spawn(|| {
    //         println!("I am printing something");
    //     });
    // }
    // for _ in 0..1_000_000 {
    //     let _x = 9;
    // }

    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });
        handle.join(); // Wait for the threads to finish
    }

    // Now we will learn about the three types of closures. The three types are:

    // - FnOnce: takes the whole value
    // - FnMut: takes a mutable reference
    // - Fn: takes a regular reference


    // Without move keyword
    // It is a long message, but helpful: it says to use the `move` keyword. 
    // The problem is that we can do anything to my_string while the thread is using it, 
    // but it doesn't own it. That would be unsafe.

    let my_string = String::from("Can I go inside the thread");

    let handle_move = std::thread::spawn(move || {
        println!("{}", my_string);
    });

    handle_move.join().unwrap();
}
