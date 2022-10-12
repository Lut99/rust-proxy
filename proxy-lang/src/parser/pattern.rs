//  PATTERN.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 23:08:57
//  Last edited:
//    11 Oct 2022, 23:32:43
//  Auto updated?
//    Yes
// 
//  Description:
//!   Matches a pattern. This is quite a variable and thus complicated
//!   one.
// 

use nom::IResult;

use crate::spec::TokenList;
use crate::ast::Pattern;


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
pub fn parse<E: nom::error::ParseError<TokenList>>(input: TokenList) -> IResult<TokenList, Pattern, E> {
    
}
