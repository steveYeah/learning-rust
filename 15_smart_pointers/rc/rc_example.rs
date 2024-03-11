enum List {
    // Cannot use as box owns the data
    // when we reference it in list b, we move the data into b
    // so when we try to reference a in c, it errors as it was already moved to b
    // see box_type_error
    //Cons(i32, Box<List>),

    // Rc -> reference counted smart pointer
    // Rc increase the reference count to it, so the data is not owned by a single list it is owned
    // by all the lists. As a result Drop is only called on a Rc smart pointed once there are no references to it
    // This smart pointer doesn't work across multiple threads
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // Commented out as the box varient has changed to the Rc<List> varient
    //box_type_error();

    rc_version();
}

//fn box_type_error() {
//    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//    let b = Cons(3, Box::new(a));
//    let c = Cons(4, Box::new(a));
//}

fn rc_version() {
    // Create a new cons list and store in a Rc<List>
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // Rc::clone only increases the reference count, it doesn't clone the data!
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
