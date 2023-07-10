fn main() {
    // Create new blank string
    let s = String::new();

    // Create a new string from a string literal
    let s = "string".to_string();
    let s = String::from("string");

    // String are unicode so all these are valid
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");

    // Updating a string
    //
    // using push_str
    let mut s = String::from("Hello, ");
    let other_s = "there";
    s.push_str(other_s);

    // push_str takes &str, so here other_s is still accessable as it did not take ownership
    println!("other_s is {other_s}");

    // push() appends a single char
    s.push('!');

    // Concatenating using + is a different story
    let s = String::from("Hello, ");
    let other_s = String::from("there");

    let obi_greeting = s + &other_s;

    //println!("s is {s}"); // Compile error as s was moved when we used the + operator
    println!("other_s is {other_s}"); // this works as this was passed as an arg and as &str

    // Using format! marco for Concatenating strings
    let s1 = String::from("Well");
    let s2 = String::from("hello");
    let s3 = String::from("there");

    let s = format!("{s1} {s2} {s3}");

    // all of the args are &str so all are still owned here
    println!("s1 is {s1}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");

    // Indexing into strings
    // Indexing into string doesn't work as expected, as you are indexing into a byte Vector
    // of UTF8 byte codes.
    // As a result, if you try it you get a complie error
    let s = String::from("hello");
    //let c = s[0]; // It's not 'h' it's 104. Compile error to stop this

    // here is a better example from the book
    let hello = "Здравствуйте";
    //let answer = &hello[0]; // The 3 here is capital Cyrillic letter Ze and takes 2 bytes to store
    // as a result this would return 208, which means nothing on it's own

    // You can take a slice though as long as there is a char in the range
    // As there is not way of telling at compile time, if there isn't it will be a run time error

    // OK
    let test = &hello[0..4];

    // not OK
    //let test = &hello[0..1]; // run time panic

    // For more saftey, loop through the string, stating if you want chars or bytes!
    for c in hello.chars() {
        println!("c is {c}");
    }

    for b in hello.bytes() {
        println!("b is {b}");
    }
}
