// A type alias means "giving a new name to another type". 
// Type aliases are very easy. Usually you use them when you have a very long type 
// and don't want to write it every time. 
// It is also good when you want to give a type a better name 
// that is easy to remember. Here are two examples of type aliases.

use std::iter::{Take, Skip};
use std::slice::Iter;

type File = String;

enum MapDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn give_direction(direction: &MapDirection) {
    use MapDirection::*;
    let m = "You are heading";

    match direction {
        North => println!("{} north.", m),
        NorthEast => println!("{} northeast.", m),
        // Better than MapDirection::North => println!....
    }
}

enum FileState {
    CannotAccessFile,
    FileOpenedAndReady,
    NoSuchFileExists,
    SimilarFileNameInNextDirectory,
}

fn give_filestate(input: &FileState) {
    use FileState::{
        CannotAccessFile as NoAccess,
        FileOpenedAndReady as Good,
        NoSuchFileExists as NoFile,
        SimilarFileNameInNextDirectory as OtherDirectory
    }

    match input {
        NoAccess => println!("Can't access file."),
        Good => println!("Here is your file."),
        NoFile => println!("Sorry, there is no file by that name."),
        OtherDirectory => println!("Please check the other directory."),
    }
}


fn return<'a>(input: &'a Vec<char>) -> Take<Skip<Iter<'a, char>>> {
    input.iter().skip(4).take(5)
}

fn main() {
    let my_file = File::from("I am file contents");
    let my_string = String::from("I am file contents");
    println!("{}", my_file == my_string);
}
