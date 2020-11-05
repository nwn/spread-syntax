use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Expr, parse::Parser, punctuated::Punctuated, Token};
use quote::quote;

#[proc_macro]
pub fn array(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let x = input.clone();
    let exprs = parser.parse(x).unwrap();
    let elements = process_elements(exprs);

    let mut output = quote! {
        let mut array = [0; 7];
        let mut idx = 0;
        let mut base = 0;
    };

    for elem in elements {
        match elem {
            ArrayElem::Atom(expr) => output = quote! {
                #output
                array[idx] = #expr;
                idx += 1;
                base += 1;
            },
            ArrayElem::Expand(expr) => output = quote! {
                #output
                let chunk = #expr;
                let chunk_len = chunk.len();
                while idx < base + chunk_len {
                    array[idx] = chunk[idx - base];
                    idx += 1;
                }
                base += chunk_len;
            }
        }
    }

    output = quote!(
        {
            #output
            array
        }
    );

    output.into()
}

fn process_elements(exprs: impl IntoIterator<Item = Expr>) -> Vec<ArrayElem> {
    use syn::{ExprRange, RangeLimits};
    exprs.into_iter().map(|expr| match expr {
        Expr::Range(ExprRange {
            attrs: _,
            from: None,
            limits: RangeLimits::HalfOpen(_),
            to: Some(embed)
        }) => ArrayElem::Expand(embed),
        _ => ArrayElem::Atom(expr),
    }).collect()
}

#[derive(Clone)]
enum ArrayElem {
    Atom(Expr),
    Expand(Box<Expr>),
}
