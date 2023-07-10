use std::io;

fn main() {
    loop {
        println!("Input temp in Fahrenheit");

        let mut temp_f = String::new();

        io::stdin()
            .read_line(&mut temp_f)
            .expect("Failed to read input");

        let temp_f: u32 = match temp_f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must be a number");
                continue;
            }
        };

        let temp_c = convert_farenheit_to_celsius(&temp_f);

        println!("{temp_f} Fahrenheit is {temp_c} in Celsius");
        break;
    }
}

fn convert_farenheit_to_celsius(temp_f: &u32) -> u32 {
    (temp_f - 32) * 5 / 9
}
