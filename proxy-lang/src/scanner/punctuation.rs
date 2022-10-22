//  PUNCTUATION.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 13:14:06
//  Last edited:
//    22 Oct 2022, 15:21:43
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements scanning functions for punctuation (typically
//!   single-character tokens).
// 

use nom::IResult;
use nom::{branch, bytes::complete as bc, combinator as comb};

use crate::scanner::{Input, Token};


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
pub fn scan<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token<'a>, E> {
    branch::alt((
        comb::map(bc::tag("->"), |t: Input<'a>| Token::Arrow(Some(t))),
        comb::map(bc::tag(":"),  |t: Input<'a>| Token::Colon(Some(t))),
        comb::map(bc::tag("["),  |t: Input<'a>| Token::LSquare(Some(t))),
        comb::map(bc::tag("]"),  |t: Input<'a>| Token::RSquare(Some(t))),
        comb::map(bc::tag("{"),  |t: Input<'a>| Token::LCurly(Some(t))),
        comb::map(bc::tag("}"),  |t: Input<'a>| Token::RCurly(Some(t))),
        comb::map(bc::tag("/"),  |t: Input<'a>| Token::Slash(Some(t))),
        comb::map(bc::tag("."),  |t: Input<'a>| Token::Dot(Some(t))),
        comb::map(bc::tag(","),  |t: Input<'a>| Token::Comma(Some(t))),
    ))(input)
}
