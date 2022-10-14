//  MOD.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 23:04:18
//  Last edited:
//    14 Oct 2022, 11:14:01
//  Auto updated?
//    Yes
// 
//  Description:
//!   The parser module takes the tokens from the scanner and generates an
//!   AST with it (defined in `ast.rs`).
// 

// Declare the submodules
pub mod settings;
pub mod pattern;
pub mod action;
pub mod rule;
pub mod areas;
pub mod parser;

// Pull stuff into the global namespace
pub use parser::{parse, Error};

// Useful macros
macro_rules! tag {
    (Token::$var:ident, $($val:expr),*) => {
        nom::bytes::complete::tag(crate::tokens::Token::$var($($val),*, crate::spec::TextRange::None))
    };
}
pub(crate) use tag;
