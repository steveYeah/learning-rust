#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    //let width = 30;
    //let height = 50;

    //Using tuples
    //let rect1 = (30, 50);

    // Using structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // using Debug
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(scale * 30),
        height: 50,
    };

    // auto uses pretty print
    dbg!(&rect2);

    // display trait
    println! {"rect1 is {:?}", rect1};

    println!(
        "The area of the rectangle is {} square pixels.",
        //area(width, height)
        //area(rect1)
        //area(&rect1)
        // Using method on struct
        rect1.area()
    );

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4: {}", rect1.can_hold(&rect4));

    let square = Rectangle::square(10);
    println!("This is a square: {:?}", square);
}

//fn area(width: u32, height: u32) -> u32 {
//fn area(dimentions: (u32, u32)) -> u32 {
fn area(rectangle: &Rectangle) -> u32 {
    //width * height
    //dimentions.0 * dimentions.1
    rectangle.width * rectangle.height
}
