//  MOD.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 23:04:18
//  Last edited:
//    16 Oct 2022, 15:15:38
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
    (Token::$var:ident) => {
        move |tokens: crate::tokens::TokenList| -> nom::IResult<crate::tokens::TokenList, crate::tokens::Token, crate::errors::ParseError> {
            use nom::InputTake;

            // Create a vanilla, to-be-comparsed token
            let expected: crate::tokens::Token = crate::tokens::Token::$var(crate::spec::TextRange::None);

            // Attempt to get the given token from the list
            if tokens.is_empty() { return Err(nom::Err::Error(crate::errors::ParseError::EofError{ expected })); }
            let (token, res): (crate::tokens::TokenList, crate::tokens::TokenList) = tokens.take_split(1);

            // Make sure if they are the same, then return
            if std::mem::discriminant(&token[0]) != std::mem::discriminant(&expected) { return Err(nom::Err::Error(crate::errors::ParseError::UnexpectedTokenError{ got: tokens[0].clone(), expected })); }
            Ok((res, token[0]))
        }
    };
    (Token::$var:ident, $($val:expr),+) => {
        nom::bytes::complete::tag(crate::tokens::Token::$var($($val),+, crate::spec::TextRange::None))
    };
}
pub(crate) use tag;
