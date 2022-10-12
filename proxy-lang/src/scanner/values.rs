//  VALUES.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 13:25:46
//  Last edited:
//    11 Oct 2022, 18:29:32
//  Auto updated?
//    Yes
// 
//  Description:
//!   Parses specific value tokens such as ports or path parts.
// 

use nom::IResult;
use nom::{branch, bytes::complete as bc, character::complete as cc, combinator as comb, multi, sequence as seq};

use crate::spec::{Input, TextPos, TextRange};
use crate::tokens::Token;
use crate::scanner::whitespace as ws;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use crate::tests::{input, range};
    use super::*;

    #[test]
    fn test_values() {
        // Attempt to parse some section stuff
        assert_eq!(scan::<nom::error::Error<Input>>(input!("[hello]")).ok(), Some((input!("", 7), Token::Section("hello".into(), range!(1:1 - 1:7)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("[HelLo]")).ok(), Some((input!("", 7), Token::Section("HelLo".into(), range!(1:1 - 1:7)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("[HelLo42]")).ok(), Some((input!("", 9), Token::Section("HelLo42".into(), range!(1:1 - 1:9)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("[]")).is_err(), true);

        // Attempt to parse some action stuff
        assert_eq!(scan::<nom::error::Error<Input>>(input!("!hello")).ok(), Some((input!("", 6), Token::Action("hello".into(), range!(1:1 - 1:6)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("!HelLo")).ok(), Some((input!("", 6), Token::Action("HelLo".into(), range!(1:1 - 1:6)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("!HelLo42")).ok(), Some((input!("", 8), Token::Action("HelLo42".into(), range!(1:1 - 1:8)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("!")).is_err(), true);

        // Simply attempt to parse some identifier stuff
        assert_eq!(scan::<nom::error::Error<Input>>(input!("hello_there")).ok(), Some((input!("", 11), Token::Identifier("hello_there".into(), range!(1:1 - 1:11)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("http")).ok(), Some((input!("", 4), Token::Identifier("http".into(), range!(1:1 - 1:4)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("http42")).ok(), Some((input!("", 6), Token::Identifier("http42".into(), range!(1:1 - 1:6)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("42http")).ok(), Some((input!("", 6), Token::Identifier("42http".into(), range!(1:1 - 1:6)))));

        // Attempt to do some port stuff
        assert_eq!(scan::<nom::error::Error<Input>>(input!("42")).ok(), Some((input!("", 2), Token::Port("42".into(), range!(1:1 - 1:2)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("65535")).ok(), Some((input!("", 5), Token::Port("65535".into(), range!(1:1 - 1:5)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("-65536")).is_err(), true);

        // Attempt to do some aterisk stuff
        assert_eq!(scan::<nom::error::Error<Input>>(input!("!hello")).ok(), Some((input!("", 6), Token::Action("hello".into(), range!(1:1 - 1:6)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("!HelLo")).ok(), Some((input!("", 6), Token::Action("HelLo".into(), range!(1:1 - 1:6)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("!HelLo42")).ok(), Some((input!("", 8), Token::Action("HelLo42".into(), range!(1:1 - 1:8)))));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("!")).is_err(), true);

        // Do some mix and match
        let tokens: Vec<Token> = crate::scanner::scan("<test>", "http ftp\n\n// Cool!!!!!\n [sec]   \t   42 ssh\n 65535 /* epic */ 42".as_bytes()).unwrap();
        assert_eq!(tokens, vec![
            Token::Identifier("http".into(), range!(1:1 - 1:4)),
            Token::Identifier("ftp".into(), range!(1:6 - 1:8)),
            Token::Section("sec".into(), range!(4:2 - 4:6)),
            Token::Port("42".into(), range!(4:14 - 4:15)),
            Token::Identifier("ssh".into(), range!(4:17 - 4:19)),
            Token::Port("65535".into(), range!(5:2 - 5:6)),
            Token::Port("42".into(), range!(5:19 - 5:20)),
        ]);
    }
}





/***** HELPER FUNCTIONS *****/
/// Scans a section header.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan a section header.
fn scan_section<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        seq::tuple((bc::tag("["), comb::cut(cc::alphanumeric1), comb::cut(bc::tag("]")))),
        |(l, name, r): (Input, Input, Input)| {
            // Return that as a token
            Token::Section((*name.fragment()).into(), TextRange::new(TextPos::start_of(&l), TextPos::end_of(&r)))
        },
    )(input)
}

/// Scans an action.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan an action.
fn scan_action<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        seq::tuple((bc::tag("!"), comb::cut(cc::alphanumeric1))),
        |(l, name): (Input, Input)| {
            // Return that as a token
            Token::Action((*name.fragment()).into(), TextRange::new(TextPos::start_of(&l), TextPos::end_of(&name)))
        },
    )(input)
}

/// Scans a path identifier (i.e., a word).
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan an identifier.
fn scan_identifier<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        multi::many1(branch::alt((
            cc::alphanumeric1,
            bc::is_a("_%"),
        ))),
        |ident: Vec<Input>| {
            // Merge all of the matched sections together, taking note of the range
            let mut text  : String    = if !ident.is_empty() { String::from(**&ident[0].fragment()) } else { String::new() };
            let mut range : TextRange = if !ident.is_empty() { TextRange::from(&ident[0]) } else { TextRange::None };
            for part in ident.into_iter().skip(1) {
                text.push_str(part.fragment());
                range.set_end(TextPos::end_of(&part));
            }

            // Return that as a token
            Token::Identifier(text, range)
        },
    )(input)
}

/// Scans a port number.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan a port number.
fn scan_port<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        seq::terminated(
            cc::digit1,
            ws::scan,
        ),
        |digits: Input| {
            // Wrap it in a Token, done (we parse down the line)
            Token::Port((*digits.fragment()).into(), TextRange::from(digits))
        }
    )(input)
}





/***** LIBRARY *****/
/// Scans one of the possible value tokens.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The `Token` that is parsed.
/// 
/// # Errors
/// This function may error if nom failed to scan a value token.
pub fn scan<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    branch::alt((
        scan_section,
        scan_action,
        scan_port,
        scan_identifier,
    ))(input)
}
