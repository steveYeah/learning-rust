use std::collections::HashMap;

fn main() {
    //Creating a new hash map
    let mut scores = HashMap::new();

    // adding values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing
    let team_name = String::from("Blue");

    // get() returns Option<&V>. By calling copied() we get Option<i32> (as that is the type of the
    // values in this hash.
    // unwrap_or() returns the i32 (unwarps it from Option), or returns 0 if it is None
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Looping through keys and values
    for (key, value) in &scores {
        println!("Key is {key}, Value is {value}");
    }

    // The hash map owns it's keys and values
    let key = String::from("key");
    let value = String::from("value");

    let mut map = HashMap::new();
    map.insert(key, value);

    //println!("Key is {key}, Value is {value}"); // Compile error as the key and value were moved
    // into the HashMap

    //Updating
    //
    // Standard rule is to override
    let mut map = HashMap::new();
    map.insert(String::from("Key"), 10);
    map.insert(String::from("Key"), 50);

    println!("map is {:?}", map); // Key is 50

    // Only add if not present
    map.entry(String::from("Another Key")).or_insert(10);
    map.entry(String::from("Key")).or_insert(20);
    println!("map is {:?}", map); // Key is 50, Another Key is 10
}
