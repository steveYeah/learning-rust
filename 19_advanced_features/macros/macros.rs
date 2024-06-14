fn main() {
    //// Macros ////
    // Macros in Rust enable metaprogramming
    // they are executed and resolved at compile time before the test of the code in compiled
    // Macros take Rust code as an argument and return Rust code
    // The cond that is returned replaces the code that was used to envoke the macro.
    //
    // There are:
    // Declarative Macros
    // and three types of procedual Macros
    //     - Custom #[derive] macros that are added to structs and enums
    //     - Attribute like macros that define custome attributes usable on any item
    //     Funciton like macros that look like function calls but operate on the tokens specidfied
    //     as their argument

    //// Declarative Macros with marco_rules! ////
    // These are the most common types of marcos

    // vec! is an example of a marco_rules macro
    // vec is replaced with the code that creates the vector and adds the values we specify to it
    // the number of arguments can vary, it could 3 as it is here or, 0 or a thousand.
    // Because of this we cannot use a fucntion to do this
    let v: Vec<u32> = steve_vec![1, 2, 3];
    println!("{:?}", v);

    // How it works
    #[macro_export]
    macro_rules! steve_vec {
        // These work like match expressions
        // but the pattern to match checks the source code sent to it and matches based on how that
        // code is structured
        //
        // Here the outter brackets define the whole pattern we are checking
        // inside we define a pattern
        // the pattern is an expression, maybe with a trailing comma ","
        // the inner brackets, starting with a $, are used to define a variable in this case x
        // the * denotes that this pattern could occur 0 or more times
        //
        // When this pattern matches to creates some code that is then returned
        ( $( $x:expr ),* ) => {
            // This is the code that is returned
            // it creates a new vector
            // then it pushes the captured expression into the vector
            // and it will do this for every instance of the matched expression (which we stated
            // could happen 0 or more times)
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    // Before the code is fully compiles the above use of steve_vec will be replaced with code that
    // looks like this
    let v: Vec<u32> = {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);

        temp_vec
    };

    //// Procedural Macros for Generating Code from Attributes ////
    // Procedural macros act more like a function
    // they take code in, operate on that code and then return the resulting code
    // This is different to the declarative macros above that match on patterns and produce code
    // that replaces the calling code
    //
    // There are 3 types of procedural macros
    // custom derive
    // Attribute-like
    // Function-like
    //
    // Procedural macros must reside in their own crate, with a special crate type, at least for
    // now

    // How to define a procedual macro
    //use proc_macro;
    //#[some_attribute] // This is a placeholder for this example and would be replaced by the type
    //                  // we want to use
    //pub fn some_name(input: TokenStream) -> TokenStream {}
    //
    //See crates for an example of the derive macros

    //// Attribute-like macros ////
    // Allows you to define your own attributes tha do things you want
    // Derive is an example but these work only on enums and structs
    // attribute like macros work on more things, such as functions

    // An example of this would be the use of routes in a web frame work
    // #[route(GET, "/")]
    // fn index() {}
    //
    // route is a new attribute that can be implemented to do what ever you want
    // the owner of the web frame work might define the signature of route something like
    //
    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream
    //
    // here we take 2 parameters, the first is the content of the attribute "GET. "/""
    // the second is the body of the item the attribute is attached too
    // in this case the index() function and the body of that function
    //
    // These are implemented in the same way we implemented the derive macro (own crate and all)

    //// Function-like macros ////
    // These look like functions and look the same as macro_rules macros
    // however, they work the same as the other Procedual macros so don't use the simple switch
    // statments, instead they take and return a TokenStream
    //
    // A possible example would be an sql macro that look any SQL statment and validated it for
    // correctness
    // let sql = sql!(SELECT * FROM posts WHERE id=1);
    //
    // and could be defined like
    // #[proc_macro]
    // pub fu sql(input: TokenStream) -> TokenStream {..}
}
