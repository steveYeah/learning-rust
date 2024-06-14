use proc_macro::TokenStream;
use quote::quote;
use syn;

// this enables us to use this macro using
// #[derive(HelloMacro)]
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a prepresentation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // We have to panic on errors as we have to return a TokenStream to conform to the procedural
    // macro API
    // you should provide more specific errors thans this though, useing panic! or expect

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // within the ast data structure, ident is the name of struct that was passed into the macro
    // for the sake of this demo app it will be Pancakes
    let name = &ast.ident;
    // quote is used to define the code we want to return
    // name is of course switched with the value held in the name var
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // into turns the result of quote into a TokenStream
    gen.into()
}
