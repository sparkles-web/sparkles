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
use proc_macro2::{self, TokenTree, Literal};
use proc_macro2::TokenStream as TokenStream2;
use quote::{
    multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token, quote_spanned,
};
use std::cell::RefCell;
use syn::*;

#[derive(Debug)]
struct Route {
    name: String,
    method: String,
    path: Literal,
}

thread_local! {
    static ROUTES: RefCell<Vec<Route>> = RefCell::new(Vec::new());
    static NOT_FOUND_ROUTE: RefCell<Option<String>> = RefCell::new(None);
}

#[proc_macro_attribute]
pub fn server(_attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, function: TokenStream) -> TokenStream {
    let ItemFn {
        ident, block, decl, ..
    } = match syn::parse(function.clone()).expect("failed to parse tokens as a function") {
        Item::Fn(item) => item,
        _ => panic!("#[route] can only be applied to functions"),
    };

    let attr = TokenStream2::from(attr);
    let tokens: Vec<TokenTree> = attr.into_iter().collect();

    let method = match &tokens[0] {
        TokenTree::Ident(ident) => {
            let ident = ident.to_string();

            match &*ident {
                "GET" => ident,
                _ => panic!("the {} method is not yet supported", ident),
            }
        },
        _ => panic!("the first argument must be an identifier")
    };

    let path = match &tokens[2] {
        TokenTree::Literal(l) => l.clone(),
        _ => panic!("the second argument must be a string literal")
    };

    let route = Route {
        name: ident.to_string(),
        method,
        path,
    };

    ROUTES.with(|f| {
        f.borrow_mut().push(route);
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
    let ItemFn {
        block, ..
    } = match syn::parse(function.clone()).expect("failed to parse tokens as a function") {
        Item::Fn(item) => item,
        _ => panic!("#[serve] can only be applied to functions"),
    };

    let statements = block.stmts;

    let mut routes = Vec::new();

    ROUTES.with(|f| {
        for route in f.borrow().iter() {
            let name = &route.name;
            let method = &route.method;
            let path = &route.path;

            let method = Ident::new(method, proc_macro2::Span::call_site());
            let name = Ident::new(name, proc_macro2::Span::call_site());

            routes.push(quote! {
                (&Method::#method, #path) => {
                    await!(server.#name(response))
                }
            });
        }
    });

    // rustc is not smart enough to realize that this is initalized once
    // or maybe it's smarter than me, who knows.
    let mut not_found_route = quote! {};

    NOT_FOUND_ROUTE.with(|f| {
        let f = f.borrow();

        match &*f {
            Some(route) => {
                let route = Ident::new(route, proc_macro2::Span::call_site());

                not_found_route = quote! {
                    (_, _) => {
                        await!(server.#route(response))
                    }
                };
            }
            None => {
                not_found_route = quote! {
                    (_, _) => {
                        let mut response = sparkles::simple_server::Builder::new();
                        response.status(StatusCode::NOT_FOUND);
                        Ok(response.body("<h1>404</h1><p>Not found!<p>".as_bytes().to_vec())?)
                    }
                };
            }
        }
    });

    let tokens = quote! {
        fn main() {
            #(#statements)*

            let simple_server = sparkles::simple_server::Server::new(move |request, response| {
                println!("Request received. {} {}", request.method(), request.uri());

                sparkles::futures::executor::block_on(async {
                    match (request.method(), request.uri().path()) {
                        #(#routes)*
                        #not_found_route
                    }
                })
            });

            simple_server.listen(host, port);
        }
    };

    tokens.into()
}
