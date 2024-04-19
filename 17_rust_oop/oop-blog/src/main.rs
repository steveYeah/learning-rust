use oop_blog::Post;

fn main() {
    let mut post = Post::new();

    // Text added when in draft state
    post.add_text("I just had my lunch");
    assert_eq!("", post.content());

    post.add_text(", it was well good!");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // This should not fail, but should not add the text
    post.add_text(" You can't have any!");
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    // firt approval
    post.approve();
    assert_eq!("", post.content());

    // final approval
    post.approve();
    assert_eq!("I just had my lunch, it was well good!", post.content());
}
