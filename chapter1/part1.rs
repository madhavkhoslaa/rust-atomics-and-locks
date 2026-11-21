use std::thread::{self, JoinHandle};
fn main() {
    let handler1: JoinHandle<()> = thread::spawn(f);
    let handler2: JoinHandle<()> = thread::spawn(f);
    match handler1.join() {
        Ok(()) => {
            println!("Hello from okay")
        }
        Err(err) => {
            println!("{:?}", err);
        }
    };
    // handler1.join();
    println!("Middle"); // Thinking Out Loud 1: I print this in between but why is the output not ordered ?
    handler2.join().unwrap();
}

fn f() {
    // let x = thread::spawn(f);  -- Note: I tested to spawn threads to see the limit of the OS(Need to dig deeper on this; Parked for now; ADHD fix). It failed around 130k
    let id = thread::current().id();
    println!("Hello from the other thread, {:?}", id);
    // x.join().unwrap();
}

/*
Thinking Out Loud 1:
The output is not ordered because:
1. The start times are around the same time
2. The threads are logging and the order is not deterministic because of OS sched
3. If I were to retern something from the thread the output will be ordered; But the execution
of the threads will not be in order
*/
