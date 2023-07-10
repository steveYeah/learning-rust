fn main() {
    let gifts = [
        (1, "A partridge in a pear tree"),
        (2, "Two turtle doves"),
        (3, "Three French hens"),
        (4, "Four calling birds"),
        (5, "Five gold rings"),
        (6, "Six geese a-laying"),
        (7, "Seven swans a-swimming"),
        (8, "Eight maids a-milking"),
        (9, "Nine ladies dancing"),
        (10, "Ten lords a-leaping"),
        (11, "Eleven pipers piping"),
        (12, "Twelve drummers drumming"),
    ];

    for (day, gift) in gifts {
        let day_suffix = if day == 1 {
            "st"
        } else if day == 2 {
            "nd"
        } else if day == 3 {
            "rd"
        } else {
            "th"
        };

        println!("On the {day}{day_suffix} day of Christmas my true love sent to me");
        println!("{gift}");

        for idx in (0..day - 1).rev() {
            let prev_gift = gifts[idx].1;

            if idx != 0 {
                println!("{prev_gift},");
            } else {
                println!("And {prev_gift}");
            }
        }
        println!(" ");
    }
}
