// Use a struct when you want one thing AND another thing.
// Use an enum when you want one thing OR another thing.

// So structs are for many things together, 
// while enums are for many choices together.

enum ThingsInTheSky {
    Sun,
    Stars,
}
// This is an enum because you can either see the sun, or the stars: 
// you have to choose one. These are called variants.


// You can add data to an enum too
enum ThingsInTheSkyV2 {
    Sun(String),
    Stars(String),
}

#[allow(dead_code)]
enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

#[derive(Debug)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[allow(dead_code)]
enum Star {
    BrownDwarf = 10,
    RedDwarf = 50,
    YellowStar = 100,
    RedGiant = 1000,
    DeadStar,
}

enum Number {
    U32(u32),
    I32(i32),
}

fn main() {
    let time = 8;
    let skystate = create_skystate(time); // create_skystate returns a ThingsInTheSky
    check_skystate(&skystate); // Givre it a reference so it can read the variable skystate

    let timev2 = 8;
    let skystatev2 = create_skystate_v2(timev2);
    check_skystate_v2(&skystatev2);

    let my_mood = Mood::Happy;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happyness is {}", happiness_level);

    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        print!("{:?}\t", &season);
    }
    println!();

    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedDwarf];
    for star in starvec {
        match star as u32 {
            size if size <= 80 => println!("Not the biggest star."),
            size if size >= 80 => println!("This is a good-sized star."),
            _ => println!("That star is pretty big"),
        }
    }
    println!("What about DeadStar? It's the number {}.", DeadStar as u32);

    let my_vec = vec![get_number(-800), get_number(8)];
    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value {}", number),
            Number::I32(number) => println!("It's an i32 with the value {}", number),
        }
    }
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun, // Between 6 and 18 hours we can see the sun
        _ => ThingsInTheSky::Stars, // Otherwise, we can see stars
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!")
    }
}

fn create_skystate_v2(time: i32) -> ThingsInTheSkyV2 {
    match time {
        6..=18 => ThingsInTheSkyV2::Sun(String::from("I can see the sun!")),
        _ => ThingsInTheSkyV2::Stars(String::from("I can see the stars!")),
    }
}

fn check_skystate_v2(state: &ThingsInTheSkyV2) {
    match state {
        ThingsInTheSkyV2::Sun(description) => println!("{}", description),
        ThingsInTheSkyV2::Stars(n) => println!("{}", n),
    }
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*;
    let happiness_level = match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    };

    happiness_level
}

fn get_number(input: i32) -> Number {
    let number = match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    };
    number
}