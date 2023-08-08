use std::fmt::Display;

pub trait Summary {
    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    // Only define the method signature
    fn summarize_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.summarize_author(),
            self.location
        )
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct BlogPost {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for BlogPost {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// With this syntax each parmeter can be different types (as long as they implements Summary). To
// forcc the same type then youm must use Trait bound syntax
pub fn notify(item: &impl Summary) -> &impl Summary {
    println!("Breaking news! {}", item.summarize());

    item
}

// Multiple traits can be listed
pub fn notify_multiple(item: &(impl Summary + Display)) {}

//Trait bound syntax version of above. Can enforce same type for each parameter the same as all
//Generic functions
pub fn notify_two<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// specify multiple traits
pub fn notify_two_multiple<T: Summary + Display>(item: &T) {}

// or with where
pub fn notify_two_multiple_where<T, U>(t: &T, u: &U)
where
    T: Summary,
    U: Display + Summary,
{
}

// Not allowed. The type of the return must be the same even when using a trait as return
//pub fn get_article(switch: bool) -> impl Summary {
//    if switch {
//        Tweet {
//            username: String::from("horse_ebooks"),
//            content: String::from("of course, as you probably already know, people"),
//            reply: false,
//            retweet: false,
//        }
//    } else {
//        NewsArticle {
//            headline: String::from("Penguins win the Stanley Cup Championship!"),
//            location: String::from("Pittsburgh, PA, USA"),
//            author: String::from("Iceburgh"),
//            content: String::from(
//                "The Pittsburgh Penguins once again are the best /
//            team in the NHL.",
//            ),
//        }
//    }
//}

fn main() {
    // Using a trait
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best /
            team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    // Using a default implementation of a trait
    let blog_post = BlogPost {
        title: String::from("My first blog post"),
        author: String::from("steveYeah"),
        content: String::from("This is my first blog post"),
    };

    println!("1 new blog post: {}", blog_post.summarize());

    // Using traits are parameters
    notify(&blog_post);
    notify_two(&blog_post);
}
