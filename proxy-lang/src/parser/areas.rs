//  AREAS.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 23:32:03
//  Last edited:
//    14 Oct 2022, 11:01:06
//  Auto updated?
//    Yes
// 
//  Description:
//!   Parses the various areas (i.e., sections) in the config file.
// 


use nom::IResult;
use nom::{combinator as comb, multi, sequence as seq};

pub use crate::errors::ParseError as Error;
use crate::spec::{Node, TextRange};
use crate::tokens::Token;
use crate::ast::{Rule, RulesArea, Setting, SettingsArea};
use crate::parser::tag;
use crate::parser::settings;
use crate::parser::rule;


/***** LIBRARY *****/
/// Parses a settings area off the list of tokens.
/// 
/// # Arguments
/// - `input`: The list of tokens.
/// 
/// # Returns
/// A SettingsArea if we were able to parse one.
/// 
/// # Errors
/// This function returns an error if we failed to parse the area.
pub fn parse_settings<'a>(input: &'a [Token]) -> IResult<&'a [Token], SettingsArea, Error> {
    comb::map(
        seq::tuple((
            tag!(Token::SettingsSection),
            multi::many0(settings::parse),
        )),
        |(header, settings): (&'a [Token], Vec<Setting>)| {
            let range: TextRange = TextRange::new(header[0].start(), if !settings.is_empty() { settings[settings.len() - 1].end() } else { header[0].end() });
            SettingsArea {
                settings,
                range,
            }
        },
    )(input)
}

/// Parses a rule area off the list of tokens.
/// 
/// # Arguments
/// - `input`: The list of tokens.
/// 
/// # Returns
/// A RulesArea if we were able to parse one.
/// 
/// # Errors
/// This function returns an error if we failed to parse the area.
pub fn parse_rules<'a>(input: &'a [Token]) -> IResult<&'a [Token], RulesArea, Error> {
    comb::map(
        seq::tuple((
            tag!(Token::RulesSection),
            multi::many0(rule::parse),
        )),
        |(header, rules): (&'a [Token], Vec<Rule>)| {
            let range: TextRange = TextRange::new(header[0].start(), if !rules.is_empty() { rules[rules.len() - 1].end() } else { header[0].end() });
            RulesArea {
                rules,
                range,
            }
        }
    )(input)
}
