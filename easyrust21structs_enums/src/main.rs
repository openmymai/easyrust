#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

impl Animal {
    fn new() -> Self {
        // Self means Animal, it can be Animal instead of Self.
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn change_to_dog(&mut self) {
        println!("Changing animal to dog!");
        self.animal_type = AnimalType::Dog;
    }

    fn change_to_cat(&mut self) {
        println!("Changing to cat!");
        self.animal_type = AnimalType::Cat;
    }

    fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
}

// Remember that Self (the type Self) and self (the variable self) are abbreviations. 
// (abbreviation = short way to write)
// So in our code, Self = Animal. 
// Also, fn change_to_dog(&mut self) means fn change_to_dog(&mut Animal).

enum Mood {
    Good,
    Bad,
    Sleepy,
}

impl Mood {
    fn check(&self) {
        match self {
            Mood::Good => println!("Feeling good!"),
            Mood::Bad => println!("Eh, not feeling so good"),
            Mood::Sleepy => println!("Need sleep NOW"),
        }
    }
}

fn main() {
    // This is where you can start to give your structs and enums some real power. 
    // To call functions on a struct or an enum, use an impl block. 
    // These functions are called methods. There are two kinds of methods in an impl block.
    //
    // Methods: these take self (or &self or &mut self). 
    // Regular methods use a . (a period). 
    // .clone() is an example of a regular method.
    //
    // Associated functions (known as "static" methods in some languages): 
    // these do not take self. Associated means "related to". 
    // They are written differently, using ::. 
    // String::from() is an associated function, and so is Vec::new(). 
    // You see associated functions most often used to create new variables.

    // For a new struct or enum, 
    // you need to give it Debug if you want to use {:?} to print

    let mut new_animal = Animal::new();

    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();

    // Mood
    let my_mood = Mood::Sleepy;
    my_mood.check();
}
