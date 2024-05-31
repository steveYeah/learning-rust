fn main() {
    //// Using the Newtype Pattern for Types Safety and Abstraction ////

    // You can use Newtypes to enforce the use a particular value whilst giveing the type a
    // meaningful value. We was this in the advanced traits section where we used the Newtypes of
    // Meters and Millimeters
    // This makes is harder for the user to use the wrong value and also adds meaning to the value
    // by explaining what it holds in the value, in this case the unit we are using

    // Newtypes are often used to abstract implementation details, and allow you to expose a more
    // meaningful API to the user, more related to the purpose (or name of the type)

    // Another use is to abstract away the implementation details. The example given is the user of
    // a People type that wraps a HashMap<i32, String> that exposing and API that allows you to add
    // a person to the collection. The API can be implemented such that the user can supply just a
    // name and doesn't need to care that the user is stored with a user ID.

    //// Creating Type Synonyms with Type Aliases ////

    // You can Alias types much in the same way as you do in Python
    type Kilometers = i32;

    // This alias is just and i32 and not a type all in to itself. You can pass an i32 to a
    // function that accepts a Kilometers. This is not the case for Newtypes, you have to give a
    // Meter when one is expected
    //
    // As with Python you use these when you repeatedly use types with long names or that use the
    // same pattern

    // Long / complicated types being reused
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}
    // commented out so I don't need to implement
    //fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {}

    // Becomes..
    type Thunk = Box<dyn Fn() + Send + 'static>;
    fn takes_long_type_alt(f: Thunk) {}
    // commented out so I don't need to implement
    //fn returns_long_type_alt() -> Thunk {}

    // It's also common to use with Result<T, E> to reduce the repitition
    // this is an example from std::io
    use std::fmt;
    use std::io::Error;

    // This is worth noting as I imagine this would be quite confusing seeing this in a codebase
    // without knowing this pattern
    type Result<T> = std::result::Result<T, std::io::Error>;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
        // ... and so on
    }

    //// The Never Type that Never Returns ////

    // Rust has an Empty Type, refered to as the Never Type
    //fn bar() -> ! {} the Return type here is the Never Type

    // This is used quite a bit although is a bit hidden behind the scenes

    // This is a common example
    let guess = String::from("5");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        //Err(_) => "hello", This will not compile as the match arms have to all return a i32!!
        Err(_) => panic!("Whoops"), // So why does this work? Does panic! return an i32?? No! It's
                                    // return type is the never type (! - as seen above). It
                                    // never returns, so there is no type. As there is no type the
                                    // compiler can see the only type returned is an i32 so all
                                    // types returned by the arms do match, as only one arm returns
                                    // a type!
    };
    // The above is also true for "contine". Infinite loops also use the Never Type
    // as their return type as well

    //// Dynamically Sized Types and the Sizes Trait ////

    // Rust needs to know how much memory to allocate to each variable type at compile time
    // If it cannot do that then the program will not compile
    // There are ways around that though

    // Dynamically Sized Types or DSTs
    // str is a DST. Because it is Dynamically Sized you cannot use it directly for the reasons
    // mentioned above

    // This will not compile as a str doesn't have a size known at compile time
    //let s1: str = "Hello there!";

    // To get around this you use a pointer of some sort, in this case a reference (a reference to
    // a Dynamically Sized type is a Slice I think)

    // This works as the size of the Slice is known at compile time
    // A slice consists of two values; a memory location and the it's length
    let s1: &str = "Hello there!";

    // you can use str with any of the pointers mentioned in this book
    // Box<str>, Rc<str> etc..

    // Traits are also DSTs
    // This is why in chapter 17 to use Traits as trait objects you have to put them behind a
    // pointer!
    // &dyn Trait or Box<dyn Trait> or Rc<dyn Trait> and so on

    // To handle DSTs Rust uses the Sized trait
    // Rust automatically implements this trait on everything where the size is known at compile
    // time
    // Rust also uses this trait as a bound to every generic function
    // so this

    //fn generic<T>(t: T) {}

    // Actually becomes this..

    //fn generic<T: Sized>(t: T) {}

    // you can relaz the restriction by allowing DSTs to be accepted as well as non DSTs by using
    // ?Sized
    // When doing so you have to accept a reference to the type so that you can compile
    // as, of course, you have to know the size of the type at compile time and we do that by using
    // a pointer, in this case a reference
    //
    fn generic<T: ?Sized>(t: &T) {}
}
