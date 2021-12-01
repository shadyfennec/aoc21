use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;

pub type Ast = syn::LitInt;

pub fn parse(args: TokenStream) -> Ast {
    const MESSAGE: &str = "days! takes a single integer as an argument";
    if args.is_empty() {
        abort_call_site!(MESSAGE)
    } else if let Ok(ast) = syn::parse2(args) {
        ast
    } else {
        abort_call_site!(MESSAGE)
    }
}
