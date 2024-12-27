#![allow(dead_code, unreachable_patterns, unused_imports)]

use ast::create_ast;
use parse::get_tokens;
pub mod parse;
pub mod tests;
pub mod ast;
pub mod funcs;
pub mod racket_error;
fn main() {
    println!("Hello World!");
    println!("{:#?}",create_ast(get_tokens("((x 2))")));
}
