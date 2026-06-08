use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, Token, Type, parse::Parser, parse_macro_input};

/// #[with_progress(expr)]
///
/// 在函数体开头注入 `let __ctx = crate::progress::ProgressContext::new(expr);`
/// 函数体可通过 `__ctx` 使用进度条上下文，函数返回时自动 finish 总进度条。
#[proc_macro_attribute]
pub fn with_progress(args: TokenStream, input: TokenStream) -> TokenStream {
    let parser = |input: syn::parse::ParseStream| -> syn::Result<(Expr, Type)> {
        let total: Expr = input.parse()?;
        input.parse::<Token![,]>()?;
        let context_type: Type = input.parse()?;
        Ok((total, context_type))
    };
    let (total_expr, context_type) = parser.parse(args).unwrap_or_else(|e| {
        panic!(
            "Expected `#[with_progress(total_expr, ContextType)]`, got error: {}",
            e
        )
    });
    let input_fn = parse_macro_input!(input as ItemFn);

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let body = &input_fn.block;

    let output = quote! {
        #vis #sig {
            let __ctx = <#context_type as crate::progress::ProgressExt>::new(#total_expr);
            #body
        }
    };

    output.into()
}
