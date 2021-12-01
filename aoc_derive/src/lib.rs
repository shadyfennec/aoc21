use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod days;

#[proc_macro]
#[proc_macro_error]
pub fn days(args: TokenStream) -> TokenStream {
    days::days_impl(args)
}
