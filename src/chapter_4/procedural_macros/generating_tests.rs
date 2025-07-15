use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Token};
use quote::quote;

struct TestCase {
    name: Ident,
    arrow: Token![=>],
    assertion: Expr,
}

impl Parse for TestCase {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(TestCase {
            name: input.parse()?,
            arrow: input.parse()?,
            assertion: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn generate_tests(input: TokenStream) -> TokenStream {
    let cases = syn::parse_macro_input!(
        input with syn::punctuated::Punctuated::<TestCase, Token![,]>::parse_terminated
    );

    let tests = cases.iter().map(|case| {
        let name = &case.name;
        let assertion = &case.assertion;
        quote! {
            #[test]
            fn #name() {
                println!("Running test: {}", stringify!(#name));
                assert!(#assertion);
            }
        }
    });

    quote! {
        #(#tests)*
    }.into()
}
