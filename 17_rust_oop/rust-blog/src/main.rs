use rust_blog::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("I just had my lunch");
    //assert_eq!("", post.content());

    post.add_text(", it was well good!");

    let post = post.request_review();
    //assert_eq!("", post.content());

    let post = post.reject();
    let post = post.request_review();

    //first approval
    let post = post.approve();
    //second
    let post = post.approve();

    assert_eq!("I just had my lunch, it was well good!", post.content());
}
