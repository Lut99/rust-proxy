//  SETTINGS.rs
//    by Lut99
// 
//  Created:
//    13 Oct 2022, 09:39:20
//  Last edited:
//    14 Oct 2022, 10:47:36
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines parsing functions for various settings in the `[settings]`
//!   section.
// 

use std::str::FromStr;

use nom::IResult;
use nom::{branch, combinator as comb, multi, sequence as seq};

pub use crate::errors::ParseError as Error;
use crate::spec::{Node, TextRange};
use crate::tokens::Token;
use crate::ast::{Setting, SettingKey, SettingValue};
use crate::parser::tag;


/***** HELPER FUNCTIONS *****/
/// Parses a string's value as a SettingValue.
/// 
/// # Arguments
/// - `input`: The list of Tokens to parse from.
/// 
/// # Returns
/// The string that is defined if there was one on top of the stack.
/// 
/// # Errors
/// This function errors if it failed to parse a SettingValue.
pub fn parse_string<'a>(input: &'a [Token]) -> IResult<&'a [Token], SettingValue, Error> {
    comb::map(
        tag!(Token::String, String::new()),
        |s: &'a [Token]| {
            if let Token::String(value, range) = s[0] {
                SettingValue::String(value, range)
            } else {
                panic!("Got a non-String token when a String is the only possibility");
            }
        }
    )(input)
}

/// Parses an unsigned integer's value as a SettingValue.
/// 
/// # Arguments
/// - `input`: The list of Tokens to parse from.
/// 
/// # Returns
/// The unsigned integer that is defined if there was one on top of the stack.
/// 
/// # Errors
/// This function errors if it failed to parse a SettingValue.
pub fn parse_uint<'a>(input: &'a [Token]) -> IResult<&'a [Token], SettingValue, Error> {
    comb::map_res(
        tag!(Token::UInt, String::new()),
        |i: &'a [Token]| {
            if let Token::UInt(value, range) = i[0] {
                // Attempt to parse
                let value: u64 = match u64::from_str(&value) {
                    Ok(value) => value,
                    Err(err)  => { return Err(nom::Err::Failure(Error::UIntParseError{ raw: value, err, range })); },
                };

                // Store it
                Ok(SettingValue::UInt(value, range))
            } else {
                panic!("Got a non-UInt token when a UInt is the only possibility");
            }
        }
    )(input)
}

/// Parses a signed integer's value as a SettingValue.
/// 
/// # Arguments
/// - `input`: The list of Tokens to parse from.
/// 
/// # Returns
/// The signed integer that is defined if there was one on top of the stack.
/// 
/// # Errors
/// This function errors if it failed to parse a SettingValue.
pub fn parse_sint<'a>(input: &'a [Token]) -> IResult<&'a [Token], SettingValue, Error> {
    comb::map_res(
        tag!(Token::SInt, String::new()),
        |i: &'a [Token]| {
            if let Token::SInt(value, range) = i[0] {
                // Attempt to parse
                let value: i64 = match i64::from_str(&value) {
                    Ok(value) => value,
                    Err(err)  => { return Err(nom::Err::Failure(Error::SIntParseError{ raw: value, err, range })); },
                };

                // Store it
                Ok(SettingValue::SInt(value, range))
            } else {
                panic!("Got a non-SInt token when a SInt is the only possibility");
            }
        }
    )(input)
}

/// Parses a boolean's value as a SettingValue.
/// 
/// # Arguments
/// - `input`: The list of Tokens to parse from.
/// 
/// # Returns
/// The boolean that is defined if there was one on top of the stack.
/// 
/// # Errors
/// This function errors if it failed to parse a SettingValue.
pub fn parse_bool<'a>(input: &'a [Token]) -> IResult<&'a [Token], SettingValue, Error> {
    comb::map_res(
        tag!(Token::Bool, String::new()),
        |i: &'a [Token]| {
            if let Token::SInt(value, range) = i[0] {
                // Attempt to parse
                let value: bool = match value.as_str() {
                    "true"  => true,
                    "false" => false,
                    _       => { return Err(nom::Err::Failure(Error::BoolParseError{ raw: value, range })); },
                };

                // Store it
                Ok(SettingValue::Bool(value, range))
            } else {
                panic!("Got a non-Bool token when a Bool is the only possibility");
            }
        }
    )(input)
}



/// Parses a list of values as a SettingValue.
/// 
/// # Arguments
/// - `input`: The list of Tokens to parse from.
/// 
/// # Returns
/// The list of values as a SettingValue that is defined if there was one on top of the stack.
/// 
/// # Errors
/// This function errors if it failed to parse a SettingValue.
pub fn parse_list<'a>(input: &'a [Token]) -> IResult<&'a [Token], SettingValue, Error> {
    comb::map(
        seq::tuple((
            tag!(Token::LSquare),
            multi::many0(branch::alt((
                parse_string,
                parse_uint,
                parse_sint,
                parse_bool,
            ))),
            tag!(Token::RSquare),
        )),
        |(l, values, r): (&'a [Token], Vec<SettingValue>, &'a [Token])| {
            SettingValue::List(values, TextRange::new(l[0].start(), r[0].end()))
        }
    )(input)
}

/// Parses a dictionary / struct notation as a SettingsValue.
/// 
/// # Arguments
/// - `input`: The list of Tokens to parse from.
/// 
/// # Returns
/// The set of values as nested Settings in a SettingValue.
/// 
/// # Errors
/// This function errors if it failed to parse a SettingValue.
pub fn parse_dict<'a>(input: &'a [Token]) -> IResult<&'a [Token], SettingValue, Error> {
    comb::map(
        seq::tuple((
            tag!(Token::LCurly),
            multi::many0(parse),
            tag!(Token::RCurly),
        )),
        |(l, settings, r): (&'a [Token], Vec<Setting>, &'a [Token])| {
            SettingValue::Dict(settings, TextRange::new(l[0].start(), r[0].end()))
        }
    )(input)
}





/***** LIBRARY *****/
/// Parses a setting in the SettingsArea.
/// 
/// # Arguments
/// - `input`: The list of Tokens to parse from.
/// 
/// # Returns
/// The Setting that is defined if there was one on top of the stack.
/// 
/// # Errors
/// This function errors if we could find one on top of the stack.
pub fn parse<'a>(input: &'a [Token]) -> IResult<&'a [Token], Setting, Error> {
    comb::map(
        seq::tuple((
            tag!(Token::Identifier, String::new()),
            tag!(Token::Colon),
            branch::alt((
                parse_string,
                parse_uint,
                parse_sint,
                parse_bool,

                parse_list,
                parse_dict,
            )),
            tag!(Token::Comma),
        )),
        |(key, colon, value, comma): (&'a [Token], &'a [Token], SettingValue, &'a [Token])| {
            Setting {
                key   : if let Token::Identifier(key, range) = key[0] { SettingKey{ value: key, range } } else { panic!("Got a non-Identifier even when that should be the only possibility") },
                value,

                range : TextRange::new(key[0].start(), comma[0].end()),
            }
        },
    )(input)
}
