//  PATTERN.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 23:08:57
//  Last edited:
//    14 Oct 2022, 11:13:15
//  Auto updated?
//    Yes
// 
//  Description:
//!   Matches a pattern. This is quite a variable and thus complicated
//!   one.
// 

use nom::IResult;
use nom::{branch, combinator as comb, multi, sequence as seq};

pub use crate::errors::ParseError as Error;
use crate::spec::{Node, TextRange};
use crate::tokens::Token;
use crate::ast::Pattern;
use crate::parser::tag;


/***** LIBRARY *****/
/// Parses a pattern off the given list of tokens.
/// 
/// # Arguments
/// - `input`: The list of tokens.
/// 
/// # Returns
/// A Pattern if we were able to parse one.
/// 
/// # Errors
/// This function returns an error if we failed to parse a pattern.
pub fn parse<'a>(input: &'a [Token]) -> IResult<&'a [Token], Pattern, Error> {
    comb::map(
        seq::tuple((
            tag!(Token::Protocol, String::new()),
            branch::alt((
                tag!(Token::IpAddress, String::new(), String::new(), String::new(), String::new()),
                multi::separated_list1(
                    tag!(Token::Dot),
                    tag!(Token::Identifier, String::new()),
                ),
            )),
        )),
        |(): ()| {
            Pattern {
                protocol : (),
                base     : (),
                path     : (),
                port     : (),

                range : (),
            }
        }
    )(input)
}
