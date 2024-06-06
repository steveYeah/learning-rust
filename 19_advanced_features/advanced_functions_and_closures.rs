fn main() {
    //// Function Pointers ////
    //
    // fn type (lower case f) is a function pointer
    // using functions pointers you can pass a function to another function

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // The first parameter is of type fn (so a function pointer) that accepts a function that accepts an
    // i32 and returns an i32
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // the fn type implements all fo the Fn closure traits (Fn, FnMut, FnOnce)
    // it's best practice to make your function generic with a trait bound on one of the above
    // traits so that you can accept both closures and functions as arguments

    // An example of using both using map() on vector from the standard library
    // These show the closure version and the function version. They do the same thing
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // The name of each enum variant that we define also becomes and initializer function
    // You can use these as function pointers that implements the closure traits, so you can use
    // them as arguments for methods that take closures
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);

    //// Return Closures ////
    // Closures are defined by traits, and you can not return something using a trait as a type.
    // With other return values you could just use the type, but you cannot do that with closures
    // as they have a concreat type

    // this raises an compile error as the the size is unknown at compile time
    //fn returns_closure() -> dyn Fn(i32) -> i32 {
    //    |x| x + 1
    //}

    // We have see this before. We can use a trait object to to return a type where the size is
    // only known at runtime
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
