struct Point<T> {
    x: T,
    y: T,
}

// We define T after impl so it can be used for Point<T> in this definition
// This means that this fucntion will work with a point holding any type
// The use of T as the name is optional, it doesn't have to match the
// struct definition but convention is to use the same name
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// No generics here. This will only work on a Point containing f32s
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// Here we use different names for the generic types to better communicate what is happening
// Note the genreic names used in the function are different to those used in the impl block
// These do not need to be defined in the impl block as they used in the method alone
impl<X1, Y1> MixedPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixedPoint<X2, Y2>) -> MixedPoint<X1, Y2> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

//fn largest<T>(list: &[T]) -> &T {
//    // Generic function
//    let mut largest = &list[0];

//    for item in list {
//        if item > largest {
//            largest = item;
//        }
//    }

//    largest
//}

fn main() {
    // Using a generic function
    let numbers = vec![23, 50, 25, 100, 65];
    //let result = largest(&numbers);

    //println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    //let result = largest(&chars);

    //println!("The largest char is {}", result);

    //=========
    // Using generic structs
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 5.0, y: 10.0 };
    //let mixed_point = Point { x: 1, y: 2.0 }; // won't work, must be the same type

    let mixed_point = MixedPoint { x: 1, y: 2.0 };

    // Also OK if they are the same
    let mixed_int_point = MixedPoint { x: 1, y: 2 };
}
