use std::thread::{self, JoinHandle};
// fn main() {
//     let numbers: Vec<u8> = vec![1, 2, 3, 4];
//     let handler1 = thread::spawn(move || {
//         // Thinking Out Loud 1: Why does the compiler tell me to add the move keyword ? Why cannot I
//         // use the owned value by myself ?
//         println!("{:?}", numbers);
//     });
//     handler1.join().unwrap();
// ! cd ~/.local/src/helix-steel && cargo xtask steel
//     //We cannot use dbg on numbers because the value got moved dbg!(numbers);
// }

// Thinking Out Loud 2.
// What if we send references to numbers ?

// use std::thread::{self, JoinHandle};
// fn main() {
//     let numbers: Vec<u8> = vec![1, 2, 3, 4];
//     let handler1 = thread::spawn(|| {
//         println!("{:?}", &numbers);
//     });
//     handler1.join().unwrap();
// }

/*
We failed on this ? Why ?
spawn method requires static trait to be on the arg that is passed inside it
which tells that the lifetime of the value will be greater than that of the thread
this guarentees that the value will always be valid and not be freed in the heap
this provides the threads memory saftey over heap owned values
*/

fn main() {
    let numbers: Vec<u8> = vec![1, 2, 3, 4];
    let numbers_clone = numbers.clone();
    let handler1 = thread::spawn(|| {
        // Thinking Out Loud 3: Why do we need to still add the move keyword ?
        // What makes the compiler give the error
        // note: function requires argument type to outlive `'static`
        //   --> part2.rs:37:20
        //    |
        // 37 |       let handler1 = thread::spawn(|| {
        //    |  ____________________^
        // 38 | |         println!("{:?}", numbers_clone);
        // 39 | |     });
        //    | |______^
        // help: to force the closure to take ownership of `numbers_clone` (and any other referenced variables), use the `move` keyword
        //    |
        // 37 |     let handler1 = thread::spawn(move || {
        //    |                                  ++++        println!("{:?}", numbers_clone);
    });
    handler1.join().unwrap();
}
