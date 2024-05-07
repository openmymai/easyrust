use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

struct City {
    name: String,
    population: HashMap<u32, u32>,
}

struct CityBTree {
    name: String,
    population: BTreeMap<u32, u32>,
}
fn main() {
    let mut tallinn = City {
        name: "Talinn".to_string(),
        population: HashMap::new(),
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
    }

    let mut tallinn = CityBTree {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    for (year, population) in tallinn.population {
        println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
    }

    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Cannada");
    }

    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldd"));

    // If a HashMap already has a key when you try to put it in, 
    // it will overwrite its value:

    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");

    println!("{:?}", book_hashmap.get(&1));

    book_hashmap.insert(1, "L'Allemagne Moderne");

    if book_hashmap.get(&1).is_none() { // is_none() returns a bool: true if it's None, false if it's Some
        book_hashmap.insert(1, "Le Petit Prince");
    }

    println!("{:?}", book_hashmap.get(&1));

    let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"];

    let mut book_hashmap = HashMap::new();

    for book in &book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {}? {}", book, true_or_false);
    }

    // show book number, if there are more than 1 book
    let mut book_hashmap = HashMap::new();

    for book in &book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0);
        *return_value += 1;
    }

    for (book, number) in book_hashmap {
        println!("{}, {}", book, number);
    }

    // ranting
    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();

    for item in data {
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }

    for (male_or_female, numbers) in &survey_hash {
        println!("{:?}: {:?}", male_or_female, numbers);
    }

    println!("{:?}", &survey_hash);


    // HashSet
    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11
    ];

    let mut number_hashset = HashSet::new();

    for number in &many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    println!("There are {} unique numbers, so we are missing {}.", hashset_length, 100 - hashset_length);

    let mut missing_vec = vec![];
    for number in 0..100 {
        if number_hashset.get(&number).is_none() {
            missing_vec.push(number);
        }
    }

    print!("It does not contain: ");
    for number in missing_vec {
        print!("{} ", number);
    }

    // BTreeSet
    let mut number_btreeset = BTreeSet::new();

    for number in &many_numbers {
        number_btreeset.insert(number);
    }
    for entry in number_btreeset {
        print!("{} ", entry);
    }

    // BinaryHeap
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number)
    }

    while let Some(number) = my_heap.pop() {
        println!("Popped off {}. Remaining numbers are: {:?}", number, show_remainder(&my_heap));
    }


    // VecDeque
    let mut my_vecdeque = VecDeque::new();
    let things_to_do = vec!["Send email to customer", "add new product to list", "phone Loki back"];

    for thing in things_to_do {
        my_vecdeque.push_front((thing, false));
    }
    
    done(&mut my_vecdeque);
    done(&mut my_vecdeque);

    check_remaining(&my_vecdeque);
    
    for task in my_vecdeque {
        print!("{:?} ", task);
    }
}   


// VecDeque
fn check_remaining(input: &VecDeque<(&str, bool)>) {
    for item in input {
        if item.1 == false {
            println!("You must: {}", item.0);
        }
    }
}

fn done(input: &mut VecDeque<(&str, bool)>) {
    let mut task_done = input.pop_back().unwrap();
    task_done.1 = true;
    input.push_front(task_done);
}
// VecDeque


// BinaryHeap
fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> {
    let mut remainder_vec = vec![];

    for number in input {
        remainder_vec.push(*number)
    }
    remainder_vec
}