use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // when the main thread ends, all other thread are terminated as well
    // Calling join on the join handle blocks the main thread until the spawned thread completes
    // this way it woll always run to completion. Comment this out and run without to see the main
    // thread terminate the spawned thread early
    handle.join().unwrap();
}
