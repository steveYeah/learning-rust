use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// All implementaions get the new function as T is generic
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only types that implement Display and PartialOrd get cmp_display
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("X is larger!: {}", self.x);
        } else {
            println!("Y is larger!: {}", self.y);
        }
    }
}
