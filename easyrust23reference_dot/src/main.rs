struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        println!("Are {} and {} equal? {}", self.number, other_number, self.number == other_number);
    }
}

fn main() {
    let my_number = 9;
    let reference = &my_number;
    println!("{}", my_number == *reference);

    // But when you use a method, Rust will dereference for you. 
    // The . in a method is called the dot operator, 
    // and it does dereferencing for free.

    let item = Item {
        number: 8,
    };
    let reference_number = &item.number; // reference number type is &u8
    println!("{}", *reference_number == 8);
    
    // But with the dot operator, we don't need *

    let reference_item = &item;
    println!("{}", reference_item.number == 8);

    let reference_item_two = &reference_item;

    item.compare_number(8);
    reference_item.compare_number(8);
    reference_item_two.compare_number(8);
}
