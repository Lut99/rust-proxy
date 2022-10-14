//  KEYWORDS.rs
//    by Lut99
// 
//  Created:
//    12 Oct 2022, 15:15:37
//  Last edited:
//    14 Oct 2022, 10:58:22
//  Auto updated?
//    Yes
// 
//  Description:
//!   Scans 'keywords' from the input source text.
// 

use nom::IResult;
use nom::{branch, bytes::complete as bc, combinator as comb};

use crate::spec::{Input, TextRange};
use crate::tokens::Token;


/***** LIBRARY *****/
/// Scans one of the possible keyword tokens.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The `Token` that is parsed.
/// 
/// # Errors
/// This function may error if nom failed to scan a keyword token.
pub fn scan<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    branch::alt((
        comb::map(
            bc::tag("[settings]"),
            |sec: Input| {
                Token::SettingsSection(TextRange::from(sec))
            },
        ),
        comb::map(
            bc::tag("[rules]"),
            |sec: Input| {
                Token::RulesSection(TextRange::from(sec))
            },
        ),
    ))(input)
}
