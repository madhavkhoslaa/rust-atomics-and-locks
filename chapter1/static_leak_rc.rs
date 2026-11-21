use std::sync::Arc;
use std::thread;
fn main() {
    static X: [i32; 4] = [1, 2, 3, 4];
    // dbg!(&X);
    let Y = Box::new([1, 2, 3, 3]);
    let Z = &*Box::leak(Y);
    let A = Arc::new(vec![1, 1, 1]);
    let handle1 = thread::spawn({
        let A = A.clone();
        move || {
            dbg!(&X);
            dbg!(&Z);
            dbg!(A);
        }
    });
    let handle2 = thread::spawn({
        let A = A.clone();
        move || {
            dbg!(&X);
            dbg!(&Z);
            dbg!(A);
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
// Thinking Out Loud 1:
// How does shadowing work ? what is the arg to threadspawn method that allows use to shadow
// but when I try to shadow under the move block what makes it A move in the new thread ?
// Problems with ARC
// They do not give mutable access to memory
