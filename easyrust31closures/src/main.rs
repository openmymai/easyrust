use std::collections::HashMap;

// Company struct
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let my_closure = || println!("This is a closure");
    my_closure();

    let my_closure_w_variable = |x: i32| println!("{}", x);

    my_closure_w_variable(5);
    my_closure_w_variable(10);

    let my_closure_w_block = || {
        let number = 7;
        let other_number = 10;
        println!("The two numbers are {} and {}", number, other_number);
    };
    my_closure_w_block();

    // - a || that doesn't enclose a variable from outside is an "anonymous function". 
    // Anonymous means "doesn't have a name". It works more like a regular function.
    // - a || that does enclose a variable from outside is a "closure". 
    // It "encloses" the variables around it to use them.

    let number_one = 6;
    let number_two = 10;
    let my_closure_enclose = || println!("{}", number_one + number_two);
    my_closure_enclose();

    let my_closure_enclose_2 = |x: i32| println!("{}", number_one + number_two + x);
    my_closure_enclose_2(5);

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.get(0).is_some() {
            &my_vec[0]
        } else {
            &0
        }
    });
    println!("{}", fourth);

    let num_vec = vec![2, 4, 6];

    let double_vec = num_vec
        .iter()
        .map(|number| number * 2)
        .collect::<Vec<i32>>();
    println!("{:?}", double_vec);


    let num_vec_for_each = vec![10, 9, 8];
    num_vec_for_each
        .iter() // iterate over it
        .enumerate() // get (index, number)
        .for_each(|(index, number)| println!("Index number {} has number {}", index, number));


    // Closure and HashMap
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];

    let number_word_hashmap = some_numbers
        .into_iter()
        .zip(some_words.into_iter())
        .collect::<HashMap<_,_>>();

    println!("For key {} we get {}.", 2, number_word_hashmap.get(&2).unwrap());

    // .enumberate() for char
    let numbers_together = "140399923481800622623218009598281";

    for (index, number) in numbers_together.char_indices() {
        match (index % 3, number) {
            (0..=1, number) => print!("{}", number),
            _ => print!("{}\t", number),
        }
    }
    println!();

    // |_|
    // Sometimes you see |_| in a closure. 
    // This means that the closure needs an argument (like x), 
    // but you don't want to use it.

    let my_vec_closure_underscore = vec![8, 9, 10];
    println!("{:?}", my_vec_closure_underscore.iter().for_each(|_| println!("We didn't use the variables at all")));

    // helpful methods
    let months = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let filtered_months = months
        .into_iter()
        .filter(|month| month.len() < 5)
        .filter(|month| month.contains("u"))
        .collect::<Vec<&str>>();

    println!("{:?}", filtered_months);

    // Company struct
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Doug Suttles"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let all_the_ceos = company_vec
        .into_iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();
    
    println!("{:?}", all_the_ceos);

    let user_input = vec!["8.9", "Nine point nine five", "8.0", "7.6", "eleventy-twelve"];

    let actual_numbers = user_input
        .into_iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();

    println!("{:?}", actual_numbers);

    // big vec
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);

    let mut counter = 0;
    let mut big_iter = big_vec.into_iter();

    loop {
        counter += 1;
        if big_iter.next() == Some(5) {
            break;
        }
    }
    println!("Final counter is: {}", counter);

    // even odd
    let even_odd = vec!["even", "odd"];
    let even_odd_vec = (0..6)
        .zip(even_odd.into_iter().cycle())
        .collect::<Vec<(i32, &str)>>();

    println!("{:?}", even_odd_vec);

    // no ending range
    let ten_chars = ('a'..).take(10).collect::<Vec<char>>();
    let skip_then_ten_chars = ('a'..).skip(1300).take(10).collect::<Vec<char>>();

    println!("{:?}", ten_chars);
    println!("{:?}", skip_then_ten_chars);

    // fold
    let some_fold_numbers = vec![9, 6, 9, 10, 11];

    println!("{}", some_fold_numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );

    // make String
    let a_string = "I don't have any dashes in me.";

    println!("{}",
        a_string
            .chars()
            .fold("-".to_string(), |mut string_so_far, next_char| {
                string_so_far.push(next_char);
                string_so_far.push('-');
                string_so_far    
            })
    );

    // chunks and windows
    let num_c_w_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    for chunk in num_c_w_vec.chunks(3) {
        println!("{:?}", chunk);
    }

    for window in num_c_w_vec.windows(3) {
        println!("{:?}", window)
    }

    // return tuple, finding word
    let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8 pm. Rule number 3: Wake up at 6 am.";
    let rule_locations = rules.match_indices("Rule").collect::<Vec<(_,_)>>();
    println!("{:?}", rule_locations);

    // peekable
    // .peekable() lets you make an iterator where you can see (peek at) the next item. 
    // It's like calling .next() (it gives an Option) except that the iterator doesn't move, 
    // so you can use it as many times as you want. You can actually think of peekable as "stoppable", 
    // because you can stop for as long as you want. Here is an example of us using .peek() three times 
    // for every item. We can use .peek() forever until we use .next() to move to the next item.

    let just_numbers = vec![1, 5, 100];
    let mut number_iter = just_numbers.iter().peekable(); // This actually creates a type of iterator called Peekable

    for _ in 0..3 {
        println!("I love the number {}", number_iter.peek().unwrap());
        println!("I really love the number {}", number_iter.peek().unwrap());
        println!("{} is such a nice number", number_iter.peek().unwrap());
        number_iter.next();
    }
}
