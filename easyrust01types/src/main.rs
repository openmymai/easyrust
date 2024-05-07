fn main() {
    // Rust has simple types that are called primitive types (primitive = very basic)

    let first_letter = 'A';
    let space = ' ';
    let other_language_char = 'Ꮔ';
    let cat_face = '😺';

    // "Cast u8 as char" means "pretend u8 is a char"
    
    // let my_number = 100;
    // println!("{}", my_number as char);
    // this print error only `u8` can be cast as `char`, not `i32`

    let my_numeber = 100;
    println!("{}", my_numeber as u8 as char);

    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());

    let slice = "Hello!";
    println!("Slice is {} bytes.", slice.len());
    let slice2 = "안녕!"; // Korean for "hi"
    println!("Slice2 is {} bytes.", slice2.len());

    println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count());
    println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count());
}