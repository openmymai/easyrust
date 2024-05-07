fn main() {
    let my_number = 8;
    dbg!(my_number);

    let mut my_no = dbg!(9);
    dbg!(my_no += 10);

    let new_vec = dbg!(vec![8, 9, 10]);

    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>());

    dbg!(double_vec);

    // inspect
    let new_inspect_vec = vec![8, 9, 10];

    let double_inspect_vec = new_inspect_vec
        .iter()
        .inspect(|first_item| println!("The item is: {}", first_item))
        .map(|x| x * 2)
        .inspect(|next_item| println!("Then it is: {}", next_item))
        .collect::<Vec<i32>>();
}
