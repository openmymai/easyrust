fn main() {
    // 1.  The stack is very fast, but the heap is not so fast.
    // 2.  Rust needs to know the size of a variable at compile time.
    // 3.  But some types don't know the size at compile time. 
    //     But the stack needs to know the exact size. 
    //     So what do you do? First you put the data in the heap, 
    //     because the heap can have any size of data. 
    //     And then to find it a pointer goes on the stack. 
    //     This is fine because we always know the size of a pointer. 
    //     So then the computer first goes to the stack, 
    //     reads the pointer, and follows it to the heap where the data is.

    // Pointers sound complicated, but they are easy. 
    // Pointers are like a table of contents in a book.
    // MY BOOK

    // TABLE OF CONTENTS

    // Chapter                        Page
    // Chapter 1: My life              1
    // Chapter 2: My cat               15
    // Chapter 3: My job               23
    // Chapter 4: My family            30
    // Chapter 5: Future plans         43

    let my_number = 15; // make a variable, this is a i32
    let single_reference = &my_number; // make a reference, this is &i32
    let double_reference = &single_reference; // this is &&i32
    let five_references = &&&&&my_number; // this is a &&&&&i32
}
