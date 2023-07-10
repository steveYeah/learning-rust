use std::collections::HashMap;

fn main() {
    let mut values = vec![
        9, 3, 0, 9, 1, 4, 1, 6, 1, 8, 7, 1, 5, 1, 4, 9, 9, 9, 9, 9, 9, 9, 9,
    ];
    println!("Starting with {:?}", values);

    // Sort vector
    values.sort();
    println!("Sorted {:?}", values);

    let mut median = 0;

    // Find median
    if values.len() % 2 == 0 {
        let x = (values.len() + 1) / 2;
        median = values[x];
    } else {
        let x = values.len() / 2;
        let x1 = x + 1;

        let v = values[x];
        median = (v + values[x1]) / 2;
    };

    // Find mode
    let mut index = HashMap::new();

    for item in &values {
        *index.entry(item).or_insert(0) += 1;
    }
    println!("Created index: {:?}", index);

    let mut mode = 0;
    let mut top_occurances = 0;
    for (key, value) in index {
        if value > top_occurances {
            top_occurances = value;
            mode = *key;
        }
    }

    // print results
    println!("The median is {median}");
    println!("The mode is {mode}");
}
