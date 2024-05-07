// Interior mutability means having a little bit of mutability on the inside. 
// Remember how in Rust you need to use mut to change a variable? 
// There are also some ways to change them without the word mut. 
// This is because Rust has some ways to let you safely change values 
// inside of a struct that is immutable. Each one of them follows some rules 
// that make sure that changing the values is still safe.

use std::cell::Cell;

struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

// RefCell
// A RefCell is another way to change values without needing to declare mut. 
// It means "reference cell", and is like a Cell but uses references instead of copies.

use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}

// There are many methods for RefCell. 
// Two of them are .borrow() and .borrow_mut(). 
// With these methods, you can do the same thing you do with & and &mut. 

// The rules are the same:

// - Many borrows is fine,
// - one mutable borrow is fine,
// - but mutable and immutable together is not fine.


// Mutex is another way to change values without declaring mut. 
// Mutex means mutual exclusion, which means "only one at a time". 
// This is why a Mutex is safe, because it only lets one process change it at a time. 
// To do this, it uses .lock(). Lock is like locking a door from the inside. 
// You go into a room, lock the door, and now you can change things inside the room. 
// Nobody else can come in and stop you, because you locked the door.

use std::sync::Mutex;

// RwLock means "read write lock". 
// It is like a Mutex but also like a RefCell. 
// You use .write().unwrap() instead of .lock().unwrap() to change it. 
// But you can also use .read().unwrap() to get read access. 
// It is like RefCell because it follows the rules:

// - many .read() variables is okay,
// - one .write() variable is okay,
// - but more than one .write() or .read() together with .write() is not okay.

use std::sync::RwLock;
use std::mem::drop;

fn main() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5, // no change
        memory: 4_000_000,
        date_issued: 2020, // no change
        on_sale: Cell::new(true), // rather than true,
    };

    // let mut super_phone_3000 -> every field will become mutable
    super_phone_3000.on_sale.set(false);

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    println!("{:?}", user_1.active);
    user_1.active.replace(false);
    println!("{:?}", user_1.active);

    let date = 2020;
    user_1
        .active
        .replace_with(|_| if date < 2000 { true } else { false });
    println!("{:?}", user_1.active);

    // Mutex
    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();

    println!("{:?}", my_mutex);
    println!("{:?}", mutex_changer);

    *mutex_changer = 6;
    println!("{:?}", mutex_changer);

    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    println!("{:?}, {:?}", read1, read2);

    drop(read1);
    drop(read2);

    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;
    drop(write1);
    println!("{:?}", my_rwlock);
}
