//  WHITESPACE.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 17:36:05
//  Last edited:
//    22 Oct 2022, 14:55:16
//  Auto updated?
//    Yes
// 
//  Description:
//!   Contains a function for parsing some whitespace (or EOF).
// 

use nom::IResult;
use nom::{branch, character::complete as bc, combinator as comb};

use crate::scanner::Input;


/***** LIBRARY *****/
/// Scans a sequence of at least one whitespace characters (where EOF is also one).
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// Nothing on success (since we don't wanna parse whitespace).
/// 
/// # Errors
/// This function may error if nom failed to scan a whitespace.
pub fn scan<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, (), E> {
    comb::value(
        (),
        branch::alt((
            bc::multispace1,
            comb::eof,
        )),
    )(input)
}
