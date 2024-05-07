// With structs, you can create your own type.
// The name of a struct should be in UpperCamelCase 
// (capital letter for each word, no spaces)

// Rust has 3 types of structs.
// 1. unit struct, unit means "doesn't have anything", just name and semicolon
struct FileDirectory;

// 2. tuple struct, or an unnamed struct, only type not field names
struct Colour(u8, u8, u8);

// 3. name struct, this is the most common struct. -> field name and types
struct SizeAndColour {
    size: u32,
    colour: Colour, // And we put it in our new named struct
}

struct Country {
    population: u32,
    capital: String,
    leader_name: String
}

fn main() {
    let my_colour = Colour(50, 0, 50);
    println!("The second part of the colour is: {}", my_colour.1);


    let size_and_colour = SizeAndColour {
        size: 150,
        colour: my_colour
    };

    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    // let kalmykia = Country {
    //     population: population,
    //     capital: capital,
    //     leader_name: leader_name,
    // };
    // don't need to write twice
    let kalmykia = Country {
        population,
        capital,
        leader_name
    };

    
}
