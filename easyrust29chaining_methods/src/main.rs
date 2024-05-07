fn main() {
    let mut new_vec_while = Vec::new();
    let mut counter = 1;

    while counter < 11 {
        new_vec_while.push(counter);
        counter += 1;
    }

    println!("new_vec_while: {:?}", new_vec_while);

    let mut new_vec_for = Vec::new();

    for i in 1..=10 {
        new_vec_for.push(i);
    }

    println!("new_vec_for: {:?}", new_vec_for);

    // functional, shorter
    let new_vec_func = (1..=10).collect::<Vec<i32>>();
    let new_new_vec_func: Vec<i32> = (1..=10).collect();

    println!("new_vec_func: {:?}, new_new_vec_func: {:?}", new_vec_func, new_new_vec_func);

    // chaining method
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();

    println!("{:?}", new_vec);

    // much esier to read
    let my_vec_10 = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_vec_from_10 = my_vec_10
        .into_iter() // "iterate" over the items (iterate = work with each item inside it). into_iter() gives us owned values, not references
        .skip(3) // skip over three items: 0, 1, and 2
        .take(4) // take the next four: 3, 4, 5, and 6
        .collect::<Vec<i32>>(); // put them in a new Vec<i32>

    println!("new_vec_from_10 {:?}", new_vec_from_10);
}
