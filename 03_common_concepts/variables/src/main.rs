fn main() {
    // defining varibles
    let mut x = 5;
    //let x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of my const is: {THREE_HOURS_IN_SECONDS}");

    // Shaddowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of X is: {x}");

    // tuples - with optional type hints
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b: [3; 5]; // 5 elements, all 3s
    let first = a[0];
}
