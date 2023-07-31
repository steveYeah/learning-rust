pub trait Summary {
    // Only define the method signature
    //fn summarize(&self) -> String;

    // Default implementation
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

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
}
