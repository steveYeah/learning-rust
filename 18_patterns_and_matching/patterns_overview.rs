fn main() {
    let x: Option<u32> = Some(3);

    //a simple match expression
    match x {
        Some(i) => Some(i + 1),
        None => None,
    };

    // a deeper if let expression
    let favorite_colour: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(colour) = favorite_colour {
        println!("Using your favorite colour, {colour}, as the background")
    } else if is_tuesday {
        println!("Tuesday is green day!")
    // Ok(age) here is a shaddow of age. This works in the same way as it does in the match
    // statement
    // We can only use the shaddow in the following scope (inside the {})
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background colour");
        } else {
            println!("Using orange as the background colour");
        }
    } else {
        println!("Using blue as the background colour");
    }

    // While let. loop while the pattern continues to match
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // pop() returns an option. When there is a value it returns some(val) and when empty it
    // returns None
    // When None is returned the pattern no longer matches and the loop ends
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops
    let v = vec!['a', 'b', 'c'];

    // everything after the for is a pattern
    // we use enumerate here so that a tuple is produced
    // the tuple matches the patter ( here the pattern is (idx, val)) so the values are assigned
    for (idx, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, idx);
    }

    // let statements
    // these are also patterns!
    // let PATTERN = EXPRESSION
    // if the pattern matches the expression then the values are assigned
    let x = 5;
    let (x, y, z) = (1, 2, 3);

    // function parameters work in the same way
    // here the value of &(3,5) matches the pattern of &(x,y) so x and y are assigned as expected
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current locations: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // refutable and irrefutable patterns
    // refutable patterns are ones that might not match
    // irrefutable patterns are ones that will always match

    // let statments are irrefutable, they must always match
    let x = 5;

    // The while and for patterns above are good examples of refutable patterns. We expect them not
    // to match as that is how we stop itterating

    // if you attempt to use an irrefutable pattern in a let (or in other places irrefutable
    // patterns are expected) you will get a compile error
    // in this case it will error because the patter for None is not covered!
    let some_val = Some(6);
    //let Some(some_other_val) = some_val;

    // to handle this case we change to an if let as above so we can handle the cases where the
    // pattern doesn't match
    if let Some(some_other_val) = some_val {
        println!("some other valu is {}", some_other_val);
    };

    // If you do the opposite and handle something as if it was refutable but was in fact
    // irrefutable then the compiler will just issue a warning
    if let x = 5 {
        println!("x is {}", 5);
    };

    // match arms must use refutable patterns, accept for the last (catch all) arm. Other wise the
    // first irrefutable pattern hit will always match and all lower arms are dead code
}
