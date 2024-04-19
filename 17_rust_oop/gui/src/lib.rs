pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // this is instead of implementing for a single type
    // Using the trait object we can have multiple different types in components
    // Just using trait bounds would limit us to a single type for each instance of Screen
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a Button");
    }
}
