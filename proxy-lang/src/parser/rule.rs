//  RULE.rs
//    by Lut99
// 
//  Created:
//    14 Oct 2022, 10:58:44
//  Last edited:
//    14 Oct 2022, 11:04:11
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines functions for parsing rules from a stream of Tokens from a
//!   source.
// 

use nom::IResult;
use nom::{combinator as comb, sequence as seq};

pub use crate::errors::ParseError as Error;
use crate::spec::{Node, TextRange};
use crate::tokens::Token;
use crate::ast::{Action, Pattern, Rule};
use crate::parser::tag;
use crate::parser::pattern;
use crate::parser::action;


/***** LIBRARY *****/
/// Parses a Rule off of the given Token stream.
/// 
/// # Arguments
/// - `input`: The stream of Tokens to parse.
/// 
/// # Returns
/// A parse Rule if there was one.
/// 
/// # Errors
/// This function errors if we failed to parse one.
pub fn parse<'a>(input: &'a [Token]) -> IResult<&'a [Token], Rule, Error> {
    comb::map(
        seq::tuple((
            pattern::parse,
            tag!(Token::Arrow),
            action::parse,
            tag!(Token::Comma),
        )),
        |(pattern, arrow, action, comma): (Pattern, &'a [Token], Action, &'a [Token])| {
            let range: TextRange = TextRange::new(pattern.start(), comma[0].end());
            Rule {
                lhs : pattern,
                rhs : action,

                range,
            }
        }
    )(input)
}
