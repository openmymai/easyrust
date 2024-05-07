// A lifetime means "how long the variable lives".
// You only need to think about lifetimes with references. 
// This is because references can't live longer than the object they come from.

// not work function
// fn returns_reference() -> &str {
//     let my_string = String::from("I am a string");
//     &my_string // ⚠️
        // &my_string can't exist without my_string
// }

fn return_str() -> &'static str {
    let my_string = String::from("I am a string");
    "I am a str" // return &str with a lifetime of static
}

// That's because we returned a &str with a lifetime of static. 
// Meanwhile, my_string can only be returned as a String: 
// we can't return a reference to it because it is going to die in the next line.

// So now fn returns_str() -> &'static str tells Rust: "don't worry, 
// we will only return a string literal". 
// String literals live for the whole program, so Rust is happy.

#[derive(Debug)]
struct City {
    name: &'static str,
    date_founded: u32,
}

#[derive(Debug)]
struct JapanCity<'a> { // with lifetime 'a
    name: &'a str, // and name also has lifetime 'a
    date_founded: u32,
}

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points.", self.name, self.hit_points)
    }
}
fn main() {
    let my_str = return_str();
    println!("{}", my_str);


    let my_city = City {
        name: "Ichinomiya",
        date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    // Japan City
    let city_name = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

    let my_japan_city = JapanCity {
        name: &city_name[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_japan_city.name, my_japan_city.date_founded);

    // Adventurer
    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
}
