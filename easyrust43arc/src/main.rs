// You remember that we used an Rc to give a variable more than one owner. 
// If we are doing the same thing in a thread, we need an Arc. 
// Arc means "atomic reference counter". 

// Atomic means that it uses the computer's processor 
// so that data only gets written once each time.

// This is important because if two threads write data at the same time, 
// you will get the wrong result.

use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {
    let thread1 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 1 is working!")
        }
    });
    let thread2 = std::thread::spawn(|| {
        for _ in 0..5 {
            println!("Thread 2 is working!")
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Exit the program");
    // Threads are working at the same time
    // This is called concurrency, which means "running together".


    // change the value of my_number
    // now we have safe clones attached to my_number, 
    // we can move them into other thread with no problem
    let my_number = Arc::new(Mutex::new(0));

    let my_number1 = Arc::clone(&my_number);
    let my_number2 = Arc::clone(&my_number);

    let thread1 = std::thread::spawn(move || {
        for _ in 0..10 {
            *my_number1.lock().unwrap() += 1;
        }
    });

    let thread2 = std::thread::spawn(move || {
        for _ in 0..10 {
            *my_number2.lock().unwrap() += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is: {:?}", my_number);
    println!("Exiting the program");

    // join 2 threads in single for loop
    let my_no = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![]; // JoinHandles will go in here

    for _ in 0..2 {
        let my_no_clone = Arc::clone(&my_no);
        let handle = std::thread::spawn(move || {
            for _ in 0..10 {
                *my_no_clone.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle); // save the handle so we can call join on it outside of the loop
        // if we don't put it in the vec, it will die here
    }
    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{:?}", my_no);


    // software engineering
    let mut handle_vec_se = vec![];
    let my_number_se = make_arc(0);

    for _ in 0..2 {
        let my_number_se_clone = new_clone(&my_number_se);
        let handle = spawn(move || {
            for _ in 0..10 {
                let mut value_inside = my_number_se_clone.lock().unwrap();
                *value_inside += 1;
            }
        });
        handle_vec_se.push(handle);
    }
    handle_vec_se.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{:?}", my_number_se);
}

fn make_arc(number: i32) -> Arc<Mutex<i32>> {
    Arc::new(Mutex::new(number))
}

fn new_clone(input: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
    Arc::clone(&input)
}