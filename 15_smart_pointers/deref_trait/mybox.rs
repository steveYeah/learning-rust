use std::ops::Deref;

// Basically a named tuple that holds one (generic) value
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// We can now dereference MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // defefs as we have the Deref trait
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));

    // This works because of deref coersion
    // Rust calls deref on m (*m) to get the String
    // The String doesn't match the parameter so it calls defer of the result of *m (in this case a
    // String
    // String implements deref, calling deref on String (*String) returns a string slice
    // This matches the parameter so it stops there
    // This is all done at compile time, so no cost at runtime
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
