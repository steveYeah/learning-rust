use std::thread;

fn main() {
    // moving in
    immutable_borrow();
    mutable_borrow();
    do_move();
}

fn immutable_borrow() {
    // Contains closure that captures vars as immutable borrow
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrow = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrow();
    println!("After calling closure: {:?}", list);
}

fn mutable_borrow() {
    // Contains closure that captures vars as immutable borrow
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mutably = || list.push(7);

    // Can no longer use list as the closure has a mutable reference to list
    //println!("Before calling closure: {:?}", list);
    borrow_mutably();
    // mutable borrow ends so can borrow immutablly again
    println!("After calling closure: {:?}", list);
}

fn do_move() {
    let list = vec![1, 2, 3];
    println!("Before defining clousre: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // No list anymore as it was moved into the closure
    //println!("After calling closure: {:?}", list);
}

fn func_trait_example_error() {
    // Silly example of using a closure with the wrong trait bound in a function that accepts and
    // closure
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 1,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    // Idea is to count how many times the closure gets called by adding a string to a vector
    // check the compile error for more

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });

    // The above closure moves value out (and into sort_operations. This means it can only be
    // called once (FnOnce). Sort_by_key requires that we can call the closure multiple times
    // (FnMut)
    //
    // Traits -> these are addative. The closure gets more of these depending on it's behaviour
    // FnOnce -> applies to all closures. If the closure moves a captured value out of it's body,
    // the closure gets this trait only
    //
    // FnMut -> applies if the closure doesn't move a captured value out of it's body. It mutable a
    // captured value though
    //
    // Fn -> Applies to closures that don't move, mutate or even capture a value

    println!("{:#?}", list);
}

fn func_trait_example_ok() {
    // Silly example of using a closure with the wrong trait bound in a function that accepts and
    // closure
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 1,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = 0;

    // Count how many times the closure gets called
    list.sort_by_key(|r| {
        sort_operations += 1;
        r.width
    });
    // The above closure mutates it's captured value only. That means it has the trait FnMut
    // Sort_by_key has the trait bound of FnMut so this is ok

    println!("{:#?}", list);
}
