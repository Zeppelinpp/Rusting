use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, Pat, parse_macro_input};

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let sig = &input.sig;
    let vis = &input.vis;
    let block = &input.block;
    let attrs = &input.attrs;

    let green = "\x1b[38;2;166;226;46m";
    let reset = "\x1b[0m";

    let arg_logs = sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let name = &pat_ident.ident;
                let name_str = name.to_string();
                if name_str == "query" || name_str == "ignore_case" {
                    let arg_fmt =
                        format!("{}[{}]{} = {{:?}}", green, name_str.to_uppercase(), reset);
                    return Some(quote! {
                        println!(#arg_fmt, #name);
                    });
                }
            }
        }
        None
    });

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            #(#arg_logs)*
            #block
        }
    };
    expanded.into()
}
