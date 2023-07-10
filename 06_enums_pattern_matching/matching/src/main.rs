#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!!!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // match is exhaustive so if
        // we do not cover all the enum values the program
        // will not compile
        //None => None,
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn check_number(x: u8) -> u8 {
    // the "other" item is a catch all
    match x {
        3 => {
            println!("Three is the magic number!");
            3
        }
        7 => {
            println!("Lucky for some!");
            7
        }
        other => other,
        //Would raise warning as unreachable
        //9 => 9,
        //
        // the below uses _ which is a catch all
        // but doesn't bind the value to anything
        // for use in the code.
        // The () is the tuple type, meaning nothing is returned
        // so do nothing if none of the above matches match
        //
        // In this code this would not compile as
        // we expect to return a u8, but you ge the idea
        //
        // _ => ()
    }
}

fn main() {
    let penny_value = value_in_cents(Coin::Penny);
    println!("A Penny is worth: {penny_value}");

    let dime_value = value_in_cents(Coin::Dime);
    println!("A Dime is worth: {dime_value}");

    let quarter_value = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("A Quarter is worth: {quarter_value}");

    let mut count = 0;
    let coin = Coin::Dime;

    // this...
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    }

    // is the same as this.
    // This is the "if let" syntax. It's more concise at the cost of loosing exhaustive checking
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    // Option matchins
    let a = plus_one(Some(5));
    let b = plus_one(None);

    // Catch alls for matches
    let num_1 = check_number(5);
    println!("You rolled a {num_1}");

    let num_2 = check_number(3);
    println!("You rolled a {num_2}");

    let num_3 = check_number(7);
    println!("You rolled a {num_3}");
}
