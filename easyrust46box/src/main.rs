// Box is a very convenient type in Rust. When you use a Box, 
// you can put a type on the heap instead of the stack. 
// To make a new Box, just use Box::new() and put the item inside.


// & is used for str, because compiler doesn't know the size of a str: it can be any length
// but the & reference is always the same length, so the compiler can use it.
// Box is similar, also you can use * on a Box to get the value, just like with &

// This is why Box is called a "smart pointer", 
// because it is like a & reference (a kind of pointer) but can do more things.

fn just_takes_a_variable<T>(item: T) {} // Takes anything and drops it.

fn main() {
    let my_number = 1; // This is an i32
    just_takes_a_variable(my_number);
    just_takes_a_variable(my_number); // using this function twice, no problem

    let my_box = Box::new(1); // This is a Box<i32>
    just_takes_a_variable(my_box.clone()); // without .clone(), the second function would make an error
    just_takes_a_variable(my_box); // Because Box is not Copy

    let my_box2 = Box::new(1); // This is a Box<i32>
    let an_integer = *my_box2; // This is an i32
    println!("{:?}", my_box2);
    println!("{:?}", an_integer);

    // You can calso use a Box to create structs with the same struct inside. (recursive)
    // means that inside struct A is maybe another struct A.
    
}
