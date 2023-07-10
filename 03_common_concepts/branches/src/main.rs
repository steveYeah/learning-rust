fn main() {
    let number = 8;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("Condition was false");
    }

    //if number {
    //    println!("This will not happen");
    //}

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    let condition = true;
    let my_number = if condition { 5 } else { 6 };
    //let my_bad = if condition { 5 } else { "six" };

    println!("My number is {my_number}");
}
