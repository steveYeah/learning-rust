fn main() {
    // Creating
    //
    // With type as no items given (and not pushed later)
    let v: Vec<i32> = Vec::new();

    // vec macro creates new vector with items
    let v = vec![1, 2, 3];

    // New with pushed so no need for type hints
    let mut v = Vec::new();

    v.push(1);
    v.push(2);

    //Accessing
    //
    // With index
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {third}");

    // With get
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // The main difference between the 2
    let v = vec![1, 2, 3, 4, 5];
    let no_there = v.get(100);
    let no_there = &v[100]; // Panics as out of bounds

    // Borrow check applies to the vector, not the elements
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // Immutable borrow

    //v.push(6); // Compile error as this is a mutable borrow and we have an immutable borrow above
    println!("The first element is {first}");

    // Iterating
    //
    // for loop
    v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{i}");
    }

    // Only immutable refences used here, so the borrow checker is happy
    // If we tried to edit (push or pop from v) then the BC would complain
    for i in &mut v {
        *i += 50;
    }

    //Using Enum to hold items of diffent types
    //
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // This works as all possible types are known and decleared to the compiler so it knows what
    // types you have to check for (what matches must be covered) to handle items
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
} // When the vectors go out of scope, all of its elements also go out of scope
