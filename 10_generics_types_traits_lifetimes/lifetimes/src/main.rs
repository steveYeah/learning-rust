use std::fmt::Display;
//&i32        // a reference
//&'a i32     // a reference with an explicit lifetime
//&'a mut i32 // a mutable reference with an explicit lifetime

// Lifetime parameters tell the complier how the lifetime of each reference relate to each other.
// Here the return reference will be valid as long as the shortest lived parameter (x or y)
// This is then checked by the compiler
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// as we always return x, we only need to set a lifetime generic for x
// as only x is related to the return
fn longest_is_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// No need for a lifetime here due to the Elision rules
// Rust can work out that the return is related to the parameter
// as there is only one parameter. The return has to be realated to to a parameter as
// it is a reference.
// This just makes things a bit easier to write, otherwise we would always have to specify
// lifetime parameters
fn longest_is_x_one_param(x: &str) -> &str {
    // All the rules:
    // 1. each param is auto assinged a lifetime
    // 2. if there is only one input lifetime the output lifetime is set to that (this case)
    // 3. if one of the input parameters is self (this is a method) then the output parmater is set
    //    to the lifetime of self
    x
}

// Using generics together (lifetime and trait bounds)
fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, announce: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", announce);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);

    // Showing how the lifetimes are checked - this is OK
    // as string1 has the longest scope. string 2 has the shortest,
    // so that will match the scope of the return value, which is does
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), &string2.as_str());
        println!("The longest string is {}", result);
    }

    // Showing how lifetimes are checked. - this is not OK
    // as string2 has the shortest lifetime, result matches that lifetime.
    // We then try to use result beyond that lifetime (when results has
    // gone out of scope). the complier suggests that bad_string2 is
    // the reason for result having the short lifetime,

    let bad_string1 = String::from("long string is long");
    let out_of_scope_result;

    {
        let bad_string2 = String::from("xyz");
        out_of_scope_result = longest(bad_string1.as_str(), &bad_string2.as_str());
    }

    // This stops it from compiling as it's used out of scope
    //println!("The longest string is {}", out_of_scope_result);

    // Structs with borrowed values
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // String literals have static lifetimes as they are stored in the binary
    // and last as long as the program.
    // The Rust compiler sometimes suggests using static lifetimes,
    // but they are not usually the answer
    let s: &'static str = "I have static lifetime!";
}

// Structs that holds a borrowed value
struct ImportantExcerpt<'a> {
    part: &'a str,
}
