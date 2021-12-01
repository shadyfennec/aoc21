use proc_macro_error::abort_call_site;

use super::parse::Ast;

pub type Model = usize;

pub fn analyze(ast: Ast) -> Model {
    if let Ok(n) = ast.base10_parse() {
        if (1..=25).contains(&n) {
            n
        } else {
            abort_call_site!("The day number must be between 1 and 25 included.")
        }
    } else {
        abort_call_site!("Wrong number literal somehow");
    }
}
