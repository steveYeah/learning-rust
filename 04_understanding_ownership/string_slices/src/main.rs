fn main() {
    let mut s = String::from("Hello world");

    //let word = first_word(&s);
    let word = good_first_word(&s); // an imutable reference is created

    s.clear(); // A mutable refence needs to be created for clear. This will fail compulation as an
               // immutable refence exists

    println!("first word is {word}");
}

fn first_word(s: &String) -> usize {
    // this is bad as we have no connection between the string data and the idx returned by this
    // fuction. If the string chages then this idx has not meaning any more, but that is not
    // tracked anywhere
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//fn good_first_word(s: &String) -> &str {
fn good_first_word(s: &str) -> &str {
    // signature
    // using &str is better as it is more generic. We can use it for string literals (which are
    // string slices!), string slices and refernces to strings
    //
    // This verson returns a string slice, which is a reference to part of the original string
    // The string can no longer be changed without raising a compile error as a non mutable
    // reference exists on the string (until it is dropped that is)
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}
