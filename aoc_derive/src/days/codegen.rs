use super::analyze::Model;
use proc_macro2::{Span, TokenStream};
use syn::Ident;

use quote::quote;

fn ident<S>(s: S) -> Ident
where
    S: AsRef<str>,
{
    Ident::new(s.as_ref(), Span::call_site())
}

fn gen_get(model: &Model) -> TokenStream {
    let days_struct = (1..=*model)
        .map(|i| ident(format!("Day{}", i)))
        .collect::<Vec<_>>();

    let days = (1..=*model).collect::<Vec<_>>();

    quote! {
        pub fn get_day(day: usize) -> eyre::Result<Box<dyn AocDay + Send + Sync>> {
            let error: eyre::Result<Box<dyn AocDay + Send + Sync>> = match day {
                #(#days => Ok(Box::new(#days_struct::default())),)*
                x if (1..=25).contains(&x) => Err(eyre::eyre!(AocError::UnimplementedDay)),
                x => Err(eyre::eyre!(AocError::NonExistentDay)).suggestion("AoC runs from the 1st to the 25th; try using one of these days."),
            };

            error.wrap_err_with(|| format!("Failed to build AoC day {}", day))
        }
    }
}

pub fn codegen(model: Model) -> TokenStream {
    let days_mod = (1..=model)
        .map(|i| ident(format!("day{}", i)))
        .collect::<Vec<_>>();

    let get = gen_get(&model);

    quote! {
        #(mod #days_mod; )*
        #(pub use #days_mod::*;)*

        #get
    }
}
