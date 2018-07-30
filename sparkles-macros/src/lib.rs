#![feature(
    async_await,
    await_macro,
    futures_api,
    pin,
    use_extern_macros
)]
#![feature(rust_2018_preview)]
#![warn(rust_2018_idioms)]

use proc_macro::TokenStream;
use quote::{
    multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token, quote_spanned,
};
use std::cell::RefCell;
use syn::*;

thread_local! {
    static ROUTES: RefCell<Vec<String>> = RefCell::new(Vec::new());
    static NOT_FOUND_ROUTE: RefCell<Option<String>> = RefCell::new(None);
}

#[proc_macro_attribute]
pub fn server(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // let input2: DeriveInput = syn::parse(input).unwrap();

    // println!("server input: {:?}\n", input);

    input
}

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let ItemFn {
        ident, block, decl, ..
    } = match syn::parse(function.clone()).expect("failed to parse tokens as a function") {
        Item::Fn(item) => item,
        _ => panic!("#[route] can only be applied to functions"),
    };

    ROUTES.with(|f| {
        f.borrow_mut().push(ident.to_string());
    });

    let inputs = decl.inputs;
    let output = decl.output;

    // syn doesn't know how to parse async functions yet, so for now, we don't write async
    // and this re-constructs the function with the async in front, and without the attribute
    let tokens = quote! {
        async fn #ident (#inputs) #output #block
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn not_found(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let ItemFn {
        ident, block, decl, ..
    } = match syn::parse(function.clone()).expect("failed to parse tokens as a function") {
        Item::Fn(item) => item,
        _ => panic!("#[route] can only be applied to functions"),
    };

    NOT_FOUND_ROUTE.with(|f| {
        *f.borrow_mut() = Some(ident.to_string());
    });

    let inputs = decl.inputs;
    let output = decl.output;

    // syn doesn't know how to parse async functions yet, so for now, we don't write async
    // and this re-constructs the function with the async in front, and without the attribute
    let tokens = quote! {
        async fn #ident (#inputs) #output #block
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn serve(_attr: TokenStream, function: TokenStream) -> TokenStream {
    // let input2: DeriveInput = syn::parse(input).unwrap();

    //    println!("serve input: {:?}\n", input);
    ROUTES.with(|f| {
        println!("here's the routes i know about: ");

        for route in f.borrow().iter() {
            println!("* {}", route);
        }
    });

    NOT_FOUND_ROUTE.with(|f| {
        println!("not found route: ");

        match &*f.borrow() {
            Some(route) => println!("* {}", route),
            _ => {}
        }
    });

    // we need to generate this:
    /*
    fn main() {
        let host = "127.0.0.1"; // THEIR CODE
        let port = "7878"; // THEIR CODE

        let server = Server;  // THEIR CODE

        let simple_server = simple_server::Server::new(move |request, response| {
            println!("Request received. {} {}", request.method(), request.uri());

            // futures::executor::block_on(route(&self, request, response))
            futures::executor::block_on(async {
                match (request.method(), request.uri().path()) {
                    (&Method::GET, "/hello") => {
                        await!(server.hello(response))
                    }
                    (_, _) => {
                        await!(server.four_oh_four(response))
                    }
                }
            })
        });

        simple_server.listen(host, port);
    }
    */

    let ItemFn {
        ident, block, decl, ..
    } = match syn::parse(function.clone()).expect("failed to parse tokens as a function") {
        Item::Fn(item) => item,
        _ => panic!("#[serve] can only be applied to functions"),
    };

    let statements = block.stmts;

    let tokens = quote! {
        fn main() {
            #(#statements)*

            let simple_server = simple_server::Server::new(move |request, response| {
                println!("Request received. {} {}", request.method(), request.uri());

                futures::executor::block_on(async {
                    match (request.method(), request.uri().path()) {
                        (&Method::GET, "/hello") => {
                            await!(server.hello(response))
                        }
                        (_, _) => {
                            await!(server.four_oh_four(response))
                        }
                    }
                })
            });

            simple_server.listen(host, port);
        }
    };

    tokens.into()
}
