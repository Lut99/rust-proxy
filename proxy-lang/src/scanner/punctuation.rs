//  PUNCTUATION.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 13:14:06
//  Last edited:
//    11 Oct 2022, 18:04:54
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements scanning functions for punctuation (typically
//!   single-character tokens).
// 

use nom::IResult;
use nom::{branch, bytes::complete as bc, combinator as comb};

use crate::spec::{Input, TextRange};
use crate::tokens::Token;


/***** LIBRARY *****/
/// Scans one of the possible punctuation tokens.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The `Token` that is parsed.
/// 
/// # Errors
/// This function may error if nom failed to scan a punctuation token.
pub fn scan<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    branch::alt((
        comb::map(bc::tag("->"), |t| Token::Arrow(TextRange::from(t))),
        comb::map(bc::tag(":"),  |t| Token::Colon(TextRange::from(t))),
        comb::map(bc::tag("/"),  |t| Token::Slash(TextRange::from(t))),
        comb::map(bc::tag("."),  |t| Token::Dot(TextRange::from(t))),
    ))(input)
}
