// We have seen traits before: Debug, Copy, Clone are all traits. 
// To give a type a trait, you have to implement it. 
// Because Debug and the others are so common, 
// we have attributes that automatically do it. 
// That's what happens when you write #[derive(Debug)]: 
// you are automatically implementing Debug.

// #[derive(Debug)]
// struct MyStruct {
//     number: usize,
// }

// So when you create a trait, you must think: 
// "Which functions should I write? 
// And which functions should the user write?"

struct Animal {
    name: String,
}

trait Dog {
    // fn bark(&self) {
    //     println!("Woof woof!");
    // }
    // fn run(&self) {
    //     println!("The dog is running!");
    // }
    fn bark(&self); // bark() says it needs a &self and return nothing
    fn run(&self); // run() says it needs a &self and return nothing
                    // so now we have to write them ourselves.
}

impl Dog for Animal {
    fn bark(&self) {
        println!("{}, stop barking", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is a cat who is {} years old.", self.name, self.age)
    }
}

fn print_cats(pet: String) {
    println!("{}", pet);
}

use std::fmt;

struct Position {
    longitude: f32,
    latitude: f32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.longitude, self.latitude)
    }
}

// Monster

use std::fmt::Debug; // So we don't have to write std::fmt::Debug every time now

struct Monster {
    health: i32,
}

// struct Wizard {}
// struct Ranger {}
#[derive(Debug)] // Now Wizard has Debug
struct Wizard {
    health: i32, // Now Wizard has health
}
#[derive(Debug)] // So does Ranger
struct Ranger {
    health: i32, // So does Ranger
}

trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl FightClose for Ranger {} // Each type get FightClose
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {} // but only Ranger gets FightFromDistance
impl Magic for Wizard {} // and only Wizard gets Magic

// trait FightClose {
// trait FightClose: std::fmt::Debug { // Now a type needs Debug to use FightClose
//     fn attack_with_sword(&self, opponent: &mut Monster) {
//         opponent.health -= 10;
//         println!(
//             "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
//             // opponent.health
//             opponent.health, &self
//         );
//     }
//     fn attack_with_hand(&self, opponent: &mut Monster) {
//         opponent.health -= 2;
//         println!(
//             "You attack with your hand. Your opponent now has {} health left. You are now at: {:?}",
//             // opponent.health
//             opponent.health, &self
//         );
//     }
// }

// impl FightClose for Wizard {}
// impl FightClose for Ranger {}

// trait FightFromDistance {
// trait FightFromDistance: std::fmt::Debug {
//     fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
//         if distance < 10 {
//             opponent.health -= 10;
//             println!(
//                 "You attack with your bow. Your opponent now has {} health left. You are now at: {:?}",
//                 // opponent.health
//                 opponent.health, self
//             );
//         }
//     }
//     fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
//         if distance < 3 {
//             opponent.health -= 4;
//         }
//         println!(
//             "You attack with your rock. Your opponent now has {} health left. You are now at: {:?}",
//             // opponent.health
//             opponent.health, self
//         )
//     }
// }
// impl FightFromDistance for Ranger {}


fn attack_with_bow<T: FightFromDistance + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You attack with your bow. Your opponent now has {} health left. You are now at: {:?}",
            // opponent.health
            opponent.health, character
        );
    }
}
fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
    opponent.health -= 10;
    println!(
        "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
        // opponent.health
        opponent.health, character
    );
}
fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "You raise your hands and cast a fireball! Your opponent now has {} health left. You are now at: {:?}",
            opponent.health, character
        );
    }
}

use std::fmt::Display;

fn print_vec<T: Display>(input: &Vec<T>) {
    for item in input {
        print!("{}", item);
    }
    println!();
}

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}

#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}

impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("{:?} has a population of {:?}.", city.name, city.population);
        }
    }
}

use std::convert::From;

struct EvenOddVec(Vec<Vec<i32>>);

impl From<Vec<i32>> for EvenOddVec {
    fn from(input: Vec<i32>) -> Self {
        let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]];

        for item in input {
            if item % 2 == 0 {
                even_odd_vec[0].push(item);
            } else {
                even_odd_vec[1].push(item);
            }
        }
        Self(even_odd_vec)
    }
}

// fn print_it<T: AsRef<str> + Display>(input: T) {
//     println!("{}", input)
// }

fn print_it<T>(input: T)
where
    T: AsRef<str> + Debug + Display,
{
    println!("{}", input)
}
fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();

    let mr_mantle = Cat {
        name: "Reggie Mantle".to_string(),
        age: 4,
    };

    // println!("Mr. Mantle is a {:?}", mr_mantle);
    // with #[derive(Debug)] it prints
    // Mr. Mantle is a Cat { name: "Reggie Mantle", age: 4 }
    println!("{}", mr_mantle);

    print_cats(mr_mantle.to_string());
    println!("Mr. Mantle's String is {} letters long.", mr_mantle.to_string().chars().count()); // Turn him into chars and count them

    // let radagast = Wizard {};
    // let aragorn = Ranger {};

    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };

    let mut uruk_hai = Monster { health: 40 };

    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragorn, &mut uruk_hai, 8);
    fireball(&radagast, &mut uruk_hai, 8);


    // From traits
    let array_vec = Vec::from([8, 9, 10]);
    print_vec(&array_vec);

    let str_vec = Vec::from("What kind of vec will I be?");
    print_vec(&str_vec);

    let string_vec = Vec::from("What kind of vec will a String be?".to_string());
    print_vec(&string_vec);

    // Country
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let findland = Country::from(finland_cities);

    findland.print_cities();

    // Odd Even
    let bunch_of_numbers = vec![8, 7, -1, 3, 222, 9787, -47, 77, 0, 55, 7, 8];
    let new_vec = EvenOddVec::from(bunch_of_numbers);

    println!("Even numbers: {:?}\nOdd numbers: {:?}", new_vec.0[0], new_vec.0[1]);

    // AsRef
    print_it("Please print me");
    print_it("Also, please print me".to_string());
    // print_it(7); <- this will not print (because T:AsRef<str>)
}
