struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    drop_when_out_of_scope();
    force_drop();
}

fn drop_when_out_of_scope() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created");
}

fn force_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    println!("CustomSmartPointer created");
    // Cannot call the destructor directly!
    //c.drop();
    //
    //drop is from std::mem::drop and is available in the prelude

    drop(c);
    println!("CustomSmartPointer dropped before end of function");
}
