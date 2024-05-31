fn main() {
    //// Associated types ////

    // these are playce orders for types within the traits and they can be defined when
    // implementing the trait

    // Iterator uses Associated type to denote the type that we will iterate over
    pub trait Iterator {
        type Item; // Associated type

        fn next(&mut self) -> Option<Self::Item>; // Using the Associated Type here in the Option
    }

    // Implementing that trait
    struct Counter(u32);

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            Some(self.0)
        }
    }

    // Using trait bounds is different than using Generics
    // If we used Generics to define the Iterator trait then we would be able to implement the
    // trait on Counter multiple time, each one with a different type
    // this would mean that each time we called next on counter we would have to annotate which
    // Iterator we wanted to use (i32, u32, str,...) everytime!
    // Using Associated Types we can only implement the trait once, meaning that we guarantee that
    // next will only be used with the one type and we do not need to annotate the type when using
    // next
    // Because of this Associated Types act as part of the traits contract

    //// Default Generic Parameters and Operator Overloading ////

    // Here is an example of overlading the + operator for a type
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    // Here we do not specify the type of Rhs and use the default.
    // See below for details
    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // The add trait looks a bit like this
    // Rhs (right hand side) is the Default Type Parameter for Add.
    // In this case it states that by default we are expecting rhs to be the same type as the type
    // that implemented this trait

    //trait Add<Rhs = Self> {
    //    type Output;

    //    fn add(self, rhs: Rhs) -> Self::Output;
    //}

    // Here is an example of not using the Default Type Parameter for Add but instead specify the type we are
    // adding
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        // Above we specify that Rhs is Meters instead of the default which is "use the same type
        // as the type implementing the trait" which in this case would be Millimeters
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // Default Type Parameters are mostly used to
    // * extend types without breaking existing code
    // * To allow customisation for cases that most people will not use
    //  The Add trait is an example of that latter, Most implemetations of Add will be adding 2
    //  like types

    //// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name ////

    // You can implement many different traits on a struct, even if the functions / methods have
    // the same name in each trait, or even on the struct itself
    // The only thing you will have to do is state which one you are calling when you call it

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("Fall and just miss the ground!");
        }
    }

    // Here is how to use the fully Qualified name to specify which fly method we want to use
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly(); // compiler defaults to using the method that is directly implemented on the
                  // type

    // Associated functions work differently to associated methods. Methods have a self
    // Parameter, so the compiler uses that parmater to work out what type you are calling the
    // method for. Functions don't have that, so we need to be more explict when calling
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // This will use the implementation on the Dog directly as that is the default behaviour
    // this will print Spot
    // How to use the Animal implementation for a Dog?
    println!("A baby dog is called a {}", Dog::baby_name());

    // This fails as Tust cannot work out which implementation of the trait to use!
    // This is because there is no self parameter for the compiler to use to work it out
    //println!("A baby dog is called a {}", Animal::baby_name());

    // To get this to work, we have to use the fully Qualified Syntax. This technically could be
    // used all the time but that would make the language very verbose

    // This prints puppy
    // The fully Qualified name specifies that we want to use the baby_name function as implemented
    // for Animal on a Dog
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // fully Qualified syntax is defined as follows
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    // for functions there is not receiver
    // you only need to use fully qualified syntax if there is not enough information for the
    // compiler to work out what it is you are trying to do

    //// Using Supertrait to Require One Trait's Functionality Within Another Trait ////

    // ther fmt::Display is the Supertrait in this case
    // it means that whatever implements OutlinePrint must also implement Display
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string(); // implemented as part of Display. This is why we need
                                           // to have Display
            let len = output.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    // no code block as we are using the default implementation
    impl OutlinePrint for Point {}

    // Without the following you will get a compiler error saying that Display is not implemented
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({} ,{})", self.x, self.y)
        }
    }

    let test_point = Point { x: 1, y: 2 };
    test_point.outline_print();

    //// Using the Newtype Pattern to implement External Traits on External Types ////

    // The Orpahn rule states you are only allowed to implement a trait on a type if either of the
    // two are local to the crate. This prevents other peoples code from breaking yours and visa
    // versa. Without this rule two creates could implement the same trait for the same type and
    // Rust wouldn't know which implementation to use

    // You can get around this using the Newtype pattern, where you wrap a type in another type
    // the following example allows us to implement Display on Vec<T>

    // This is the Newtype pattern
    struct Wrapper(Vec<String>);

    // we then implement the trait for the type that we own, but it is only a wrapper to a type we
    // do not own
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", ")) // Using self.0 to access the vector
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // Newtypes only surface the methods from the types that are wrapped if you implement them!
    // You can implement Deref to get around this, but it will surface everything
    // If you want to suface some, but not all methods then you have to implement them
    // individually!
}
