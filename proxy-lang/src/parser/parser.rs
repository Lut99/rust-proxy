//  PARSER.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 23:04:50
//  Last edited:
//    12 Oct 2022, 15:41:06
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the toplevel parsing functions.
// 

use nom::IResult;
use nom::{branch, combinator as comb, multi};

pub use crate::errors::ParseError as Error;
use crate::spec::{TextRange, TokenList};
use crate::tokens::Token;
use crate::ast::{Config, Pattern};
use crate::parser::areas;


/***** HELPER FUNCTIONS *****/
/// Parses the toplevel Config thing.
/// 
/// # Arguments
/// - `input`: The input tokens to scan.
/// 
/// # Returns
/// The Config if we were able to parse it.
/// 
/// # Errors
/// A nom error if we failed (either because no parser matched or because there was a genuine error).
fn parse_config<'a, E: nom::error::ParseError<TokenList>>(input: &'a [Token]) -> IResult<TokenList, Config, E> {
    comb::map(
        branch::alt((
            multi::many0(areas::parse_settings),
        )),
        |patterns: Vec<Pattern>| {
            Config {
                config   : vec![],
                patterns : vec![],

                range : TextRange::None,
            }
        },
    )(input)
}





/***** LIBRARY *****/
/// Parses the given list of tokens into an AST.
/// 
/// # Arguments
/// - `input`: The list of tokens to parse.
/// 
/// # Returns
/// The toplevel Config node of the AST.
/// 
/// # Errors
/// This function errors if we failed to parse the input.
pub fn parse(input: Vec<Token>) -> Result<Config, Error> {
    // Simply parse a config directly
    match parse_config::<nom::error::VerboseError<TokenList>>(TokenList::new(input)) {
        Ok((rest, config)) => {
            if !rest.is_empty() { return Err(Error::NonEmptyTokenList { remain: rest }); }
            Ok(config)
        },

        Err(err) => { return Err(Error::ParseError{ err }); },
    }
}
