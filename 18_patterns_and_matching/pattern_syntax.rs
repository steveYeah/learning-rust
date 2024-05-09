fn main() {
    // Matching Literals
    // matching concreate values
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("four"),
    }

    // Matching Named Variables
    // "Matched, y = 5"
    // "at the end: x = Some(5), y = 10"
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y, = {y}"),
        _ => println!("Default case, x ={:?}", x),
    }

    // Multiple Patterns
    // using OR
    let x = 1;

    match x {
        1 | 2 => println!("one or two"), // | is the or operator
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values
    // using the ..= operator
    //
    // Only works for numeric or char values as rust had to determine if the range is empty
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //Destructuring to break apart values

    // Destructuring Structs
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    // a and b are assigned to the values in x and y within the point
    assert_eq!(0, a);
    assert_eq!(7, b);

    //let (x, y) = p; This no work as you have to assign the values to a Point, otherwise the types
    //don't match. In this case a tuple is not a Point

    // Because this is common there is a shorthand
    // this uses the same name for the values in the Point struct. Because these are the same Rust
    // patten matches them with the values in Point and assigns them to like-named values x to x
    // for example
    let Point { x, y } = p; // a and b are assigned to the values in x and y within the point
                            // Order of x and y don't seem to matter either

    assert_eq!(0, x);
    assert_eq!(7, y);

    // You can also destructure with literals..
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"), // match when y is 0. Then shadow x
        Point { x: 0, y } => println!("On the y axis at {y}"), // match when x is 0. Then shadow y
        Point { x, y } => println!("On neither axis at ({x}, {y}"),
    }

    // Destructuring Enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the colour to red {r}, green {g}, and blue {b}",)
        }
    }

    // Nested Enums
    enum Colour {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColour(Colour),
    }

    let msg = Message2::ChangeColour(Colour::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColour(Colour::Rgb(r, g, b)) => {
            println!("Change the colour to red {r}, green {g}, and blue {b}",)
        }
        Message2::ChangeColour(Colour::Hsv(h, s, v)) => {
            println!("Change the colour to hue {h}, saturation {s}, value {v}",)
        }
        _ => (),
    }

    // Destructuring Struct and Tuples

    // Destructuring a mix of tuples and structs
    // feet, inches, x and y are assigned
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring Values in a Pattern with _

    // Functional parameters
    // This can be usuful when implementing a trait. You have to match the signiture but you might
    // not need all of the parameters
    fn foo(_: i32, y: i32) {
        println!("This code only used the y parameter: {}", y);
    }

    foo(3, 4);

    // ignoring parts of a value with _

    // This example controls a setting. When a value is set you cannot override it, but you can
    // unset it
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwirte an existing cusomized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    // you can also match some using _ to ignore the ones you don't care about
    let numbers = (1, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("some numbers: {first}, {third}, {fifth}")
        }
    }

    // Ignoring an unused variable by starting its name with _
    let _x = 5; // will not raise not used warning
    let y = 10; // will raise not used warning

    // using an underscore before a var name means the value still binds to the variable
    // just using _ means it will not bind
    // we care about this as we can pass ownership of a value in a var if it is bound to it

    // binding
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    //println!("{:?}", s); // this will raise an error at compile time as s was moved into _s in
    //the if statement
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s); // this is ok as s was not moved into _ in the if statement as Rust
                         // doesn't bind to _

    // Ignoring Remaing Parts of a Value with ..
    struct ThreeDeePoint {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = ThreeDeePoint { x: 0, y: 0, z: 0 };

    match origin {
        ThreeDeePoint { x, .. } => println!("x is {}", x),
    }

    // .. will expand to cover any number of values
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // it needs to be clear what values you are using .. for
    // if it is ambiguous then it the code will not complie

    //match numbers {
    //    (.., second, ...) => {
    //        println!("Some numbers: {}:", second);
    //    }
    //}

    // Match Guards

    // You can use an additional conditions on a match arm. These condtions can use the new
    // variable created in the match.
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The Number {} is even", x), // Note using x in the guard
        Some(x) => println!("The Number {} is odd", x),
        None => (),
    }

    // Match Guards apply to all patterns in the same arm
    let x = 4;
    let y = false;

    // This prints "no" as y is false. The guard applies to all patterns, not jus the last one
    // the precedence is (4 | 5 | 6) if y
    // and NOT 4 | 5 | (6 if y)
    match x {
        4 | 5 | 6 if y => println!("Yes"),
        _ => println!("no"),
    }

    // @ Bindings
    // These allow you to both test and bind a value in one pattern
    enum Message3 {
        Hello { id: i32 },
    }

    let msg = Message3::Hello { id: 5 };
    match msg {
        // Binds the value in id to id_variable at the same time as testing the range
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        //  Test the range but the value is not bound to a variable the arm can use, as not using
        //  an @ binding
        Message3::Hello { id: 10..=12 } => println!("Found an id in a another range"), // Tests the
        // can use the variable, but it is not tested in the pattern
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
