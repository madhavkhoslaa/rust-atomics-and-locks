use std::thread;
fn main() {
    let mut numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            numbers.push(4);
            println!("{:?}", numbers);
        });
        // s.spawn(|| {
        //     numbers.push(4);
        //     println!("{:?}", numbers);
        // });
    });
    println!("Print outside the thread {:?}", numbers);
}
/*
Thinking out loud 1: Why did we not need a join here ?
Thinking out loud 2: Why were we not able to do 2 mutations ?
Thinking out loud 3: What is s ? How does it have the spawn method ?
*/
