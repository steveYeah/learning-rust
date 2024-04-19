use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // Can't do this as Rust sees that the closure only needs a reference so just tries to borrow
    // it. However Rust can not tell if v will be dropped while the thread is still using it, so
    // there for this doesn't compile
    //let handle = thread::spawn(|| println!("Here's a vector {:?}", v));
    //
    //To solve we move the value into the closure so it takes ownership of it
    let handle = thread::spawn(move || println!("Here's a vector {:?}", v));

    // this is an example of how the value could have been outlived by the thread. Neither version
    // will compile however. The version without the move will throw an error saying it can't tell
    // if the thread will out live the value. The version with the move will error that we are
    // trying to manipulate a value that we moved out of scope
    //drop(v);

    handle.join().unwrap();
}
