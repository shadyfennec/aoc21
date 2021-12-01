use proc_macro::TokenStream;

mod parse;
pub use parse::parse;

mod analyze;
pub use analyze::analyze;

mod codegen;
pub use codegen::codegen;

pub fn days_impl(args: TokenStream) -> TokenStream {
    let ast = parse(args.into());
    let model = analyze(ast);
    codegen(model).into()
}
