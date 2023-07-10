fn main() {
    // Double free error control

    // String is a variable sized data type. This means it is allocated on the heap
    let s1 = String::from("hello");
    //  this expression "moves" data from s1 to s2, so s1 is no longer valid. Like shallow copy, but the original is
    // removed/ invalidated
    // let s2 = s1;

    // clone creates a deepcopy of the data on the heap
    let s2 = s1.clone();

    println!("s1 is {s1}");

    // Copying values on the heap

    // Here, the ints are on the stack as they are of a known size
    let x = 5;
    let y = x;
    // all the int types implement the copy trait that allows the value to be copied
    // You cannot implement copy if drop is implemented on the type

    println!("x: {x}, y:{y}");

    // the same as above applied to functions
    takes_ownership(s1);
    makes_copy(x);

    // takes_ownership does just that and now s1 is no longer valid in this scope
    //println!("{s1}");
    println!("{x}");

    let s1 = gives_ownership();
    let s2 = takes_and_gives_back(s1);

    // Fails as we moved s1 to s2 above
    //println!("{s1}");

    println!("{s2}");

    // Borrowing and mutable reference

    // this erros as if you have a mutable reference to something you cannot have other refereces
    // to it

    let mut s = String::from("Hello");
    //let r1 = &mut s;
    //let r2 = &mut s;

    // You can create more than one mutable reference but they cannot both be active at the same
    // time
    {
        // Seperate scope
        let r1 = &mut s;
        println!("r1: {r1}");
    }

    let r2 = &mut s;
    println!("r2: {r2}");

    // The same is true for mutable and imutable reference
    let mut s = String::from("Hello");
    let r1 = &s; // All ok
    let r2 = &s; // Still ok as we are just reading

    //let r3 = &mut s; // Not OK!
    //println!("{r1}, {r2}, {r3}");

    // you can create mutable reference once the other reference have been dropped
    let mut s = String::from("Hello");
    let r1 = &s; // All ok
    let r2 = &s; // Still ok as we are just reading

    println!("{r1}, {r2}"); // Ownership of the references is moved to println! which drops them
                            // after it completes
    let r3 = &mut s; // Now ok as it is the only reference to s
    println!("{r3}");

    // Dangling pointers

    // This raises a complie error as the string we return a reference to is dropped when the
    // function ends, so the value the reference is pointing to no longer exists
    let s = dangle();
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // here some_string is dropped (removed from memory (the heap))

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");

    some_string // gives owndership of some_string to the calling code
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string // Take owndership of some_string amd then returns it again
}

fn dangle() -> &String {
    let s = String::from("Hello");

    s
}
