//  AREAS.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 23:32:03
//  Last edited:
//    11 Oct 2022, 23:35:14
//  Auto updated?
//    Yes
// 
//  Description:
//!   Parses the various areas (i.e., sections) in the config file.
// 


use nom::IResult;
use nom::{bytes::complete as bc, sequence as seq};

use crate::spec::TokenList;
use crate::tokens::Token;
use crate::ast::SettingsArea;


/***** LIBRARY *****/
/// Parses a settings area off the list of tokens.
/// 
/// # Arguments
/// - `input`: The list of tokens.
/// 
/// # Returns
/// A Pattern if we were able to parse one.
/// 
/// # Errors
/// This function returns an error if we failed to parse the area.
pub fn parse<E: nom::error::ParseError<TokenList>>(input: TokenList) -> IResult<TokenList, SettingsArea, E> {
    seq::tuple((
        
    ))(input)
}
