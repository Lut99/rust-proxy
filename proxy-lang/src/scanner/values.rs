//  VALUES.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 13:25:46
//  Last edited:
//    22 Oct 2022, 15:57:13
//  Auto updated?
//    Yes
// 
//  Description:
//!   Parses specific value tokens such as ports or path parts.
// 

use nom::IResult;
use nom::{branch, bytes::complete as bc, character::complete as cc, combinator as comb, multi, sequence as seq};

use crate::source::SourceRef;
use crate::scanner::{Input, Token};
use crate::scanner::whitespace as ws;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use crate::source::SourceText;
    // use crate::tests::{input, range};
    use super::*;

    #[test]
    fn test_values() {
        // // Attempt to parse some action stuff
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("!hello")).ok(), Some((input!("", 6), Token::Action("hello".into(), range!(1:1 - 1:6)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("!HelLo")).ok(), Some((input!("", 6), Token::Action("HelLo".into(), range!(1:1 - 1:6)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("!HelLo42")).ok(), Some((input!("", 8), Token::Action("HelLo42".into(), range!(1:1 - 1:8)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("!")).is_err(), true);

        // // Do some protocol stuff
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("http://")).ok(), Some((input!("", 7), Token::Protocol("http".into(), range!(1:1 - 1:7)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("ftp://")).ok(), Some((input!("", 6), Token::Protocol("ftp".into(), range!(1:1 - 1:6)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("ssh://")).ok(), Some((input!("", 6), Token::Protocol("ssh".into(), range!(1:1 - 1:6)))));

        // // Simply attempt to parse some identifier stuff
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("hello_there")).ok(), Some((input!("", 11), Token::Identifier("hello_there".into(), range!(1:1 - 1:11)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("http")).ok(), Some((input!("", 4), Token::Identifier("http".into(), range!(1:1 - 1:4)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("http42")).ok(), Some((input!("", 6), Token::Identifier("http42".into(), range!(1:1 - 1:6)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("42http")).ok(), Some((input!("", 6), Token::Identifier("42http".into(), range!(1:1 - 1:6)))));

        // // Attempt to do some port stuff
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("42")).ok(), Some((input!("", 2), Token::Port("42".into(), range!(1:1 - 1:2)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("65535")).ok(), Some((input!("", 5), Token::Port("65535".into(), range!(1:1 - 1:5)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("-65536")).is_err(), true);

        // // Attempt to do some action stuff
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("!hello")).ok(), Some((input!("", 6), Token::Action("hello".into(), range!(1:1 - 1:6)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("!HelLo")).ok(), Some((input!("", 6), Token::Action("HelLo".into(), range!(1:1 - 1:6)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("!HelLo42")).ok(), Some((input!("", 8), Token::Action("HelLo42".into(), range!(1:1 - 1:8)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("!")).is_err(), true);

        // // Attempt to do some aterisk stuff
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("*")).ok(), Some((input!("", 1), Token::Aterisk(None, range!(1:1 - 1:1)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("**")).ok(), Some((input!("", 2), Token::Aterisk(None, range!(1:1 - 1:2)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("*1")).ok(), Some((input!("", 2), Token::Aterisk(Some("1".into()), range!(1:1 - 1:2)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("*12")).ok(), Some((input!("2", 2), Token::Aterisk(Some("1".into()), range!(1:1 - 1:2)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("**12")).ok(), Some((input!("12", 2), Token::Aterisk(None, range!(1:1 - 1:2)))));

        // // Strings
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("\"Test\"")).ok(), Some((input!("", 6), Token::String("Test".into(), range!(1:1 - 1:6)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("\"Tes\\\'t\"")).ok(), Some((input!("", 8), Token::String("Tes\'t".into(), range!(1:1 - 1:8)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("\"Tes\\\"t\"")).ok(), Some((input!("", 8), Token::String("Tes\"t".into(), range!(1:1 - 1:8)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("\"Tes\\nt\"")).ok(), Some((input!("", 8), Token::String("Tes\nt".into(), range!(1:1 - 1:8)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("\"Tes\\tt\"")).ok(), Some((input!("", 8), Token::String("Tes\tt".into(), range!(1:1 - 1:8)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("\"Tes\\rt\"")).ok(), Some((input!("", 8), Token::String("Tes\rt".into(), range!(1:1 - 1:8)))));
        // assert_eq!(scan::<nom::error::Error<Input>>(input!("\"Tes\\\\t\"")).ok(), Some((input!("", 8), Token::String("Tes\\t".into(), range!(1:1 - 1:8)))));

        // // Do some mix and match
        // let tokens: Vec<crate::tokens::Token<SourceText>> = crate::scanner::scan("<test>", "http ftp\n\n// Cool!!!!!\n    \t   42 ssh\n 65535 /* epic */ 42".as_bytes()).unwrap();
        // assert_eq!(tokens, vec![
        //     crate::tokens::Token::<SourceText>::Identifier("http".into(), range!(1:1 - 1:4)),
        //     crate::tokens::Token::<SourceText>::Identifier("ftp".into(), range!(1:6 - 1:8)),
        //     crate::tokens::Token::<SourceText>::Port("42".into(), range!(4:9 - 4:10)),
        //     crate::tokens::Token::<SourceText>::Identifier("ssh".into(), range!(4:12 - 4:14)),
        //     crate::tokens::Token::<SourceText>::Port("65535".into(), range!(5:2 - 5:6)),
        //     crate::tokens::Token::<SourceText>::Port("42".into(), range!(5:19 - 5:20)),
        // ]);
    }
}





/***** HELPER FUNCTIONS *****/
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
            Token::Action(name.as_str().into(), Some(l + name))
        },
    )(input)
}

/// Scans a protocol identifier (i.e., a `<prot>://` thing).
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan an identifier.
fn scan_protocol<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        seq::tuple((
            cc::alphanumeric1,
            bc::tag("://"),
        )),
        |(prot, slash): (Input, Input)| {
            Token::Protocol(prot.as_str().into(), Some(prot + slash))
        }
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
            let mut text   : String            = if !ident.is_empty() { String::from(ident[0].as_str()) } else { String::new() };
            let mut source : Option<SourceRef> = if !ident.is_empty() { Some(ident[0]) } else { None };
            for part in ident.into_iter().skip(1) {
                text.push_str(part.as_str());
                source = Some(source.unwrap() + part);
            }

            // Return that as a token
            Token::Identifier(text, source)
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
            Token::Port(digits.as_str().into(), Some(digits))
        }
    )(input)
}

/// Scans an aterisk, possibly named.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan an aterisk.
fn scan_aterisk<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        seq::pair(
            bc::tag("*"),
            comb::opt(cc::one_of("0123456789*")),
        ),
        |(aterisk, tag): (Input, Option<char>)| {
            // Construct the name
            let name: Option<String> = match tag {
                Some(c) => if c != '*' { Some(String::from(c)) } else { None },
                None    => None,
            };

            // Construct the range
            let mut source: SourceRef = aterisk;
            if tag.is_some() {
                source.enlarge(1);
            }

            // Construct a token out of those
            Token::Aterisk(name, Some(source))
        }
    )(input)
}



/// Scans a string literal.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan a string.
fn scan_string<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        seq::tuple((
            bc::tag("\""),
            bc::escaped(
                seq::pair(
                    comb::not(cc::one_of("\\\"")),
                    bc::take(1usize),
                ),
                '\\',
                cc::one_of("\\'\"ntr"),
            ),
            bc::tag("\""),
        )),
        |(l, text, r): (Input, Input, Input)| {
            // Resolve the text
            let mut value: String = String::with_capacity(text.size());
            let mut escaped: bool = false;
            for c in text.as_str().chars() {
                if escaped && c == 'n' {
                    value.push('\n');
                    escaped = false;
                } else if escaped && c == 'r' {
                    value.push('\r');
                    escaped = false;
                } else if escaped && c == 't' {
                    value.push('\t');
                    escaped = false;
                } else if !escaped && c == '\\' {
                    escaped = true;
                } else {
                    value.push(c);
                    escaped = false;
                }
            }

            // Construct a token out of those
            Token::String(value, Some(l + r))
        }
    )(input)
}

/// Scans an (unsinged) integer literal.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan an integer.
fn scan_uint<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        cc::digit1,
        |digits: Input| {
            Token::UInt(digits.as_str().into(), Some(digits))
        }
    )(input)
}

/// Scans a(n) (signed) integer literal.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan an integer.
fn scan_sint<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        seq::tuple((multi::many1(bc::tag("-")), cc::digit1)),
        |(signs, digits): (Vec<Input>, Input)| {
            Token::UInt(format!("{}{}", signs.iter().map(|s| s.as_str()).collect::<String>(), digits.as_str()), Some(signs[0] + digits))
        }
    )(input)
}

/// Scans a boolean literal.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// The parsed `Token`.
/// 
/// # Errors
/// This function may error if nom failed to scan a boolean.
fn scan_bool<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Token, E> {
    comb::map(
        branch::alt((
            bc::tag("true"),
            bc::tag("false"),
        )),
        |val: Input| {
            Token::Bool(val.as_str().into(), Some(val))
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
        scan_action,
        scan_port,
        scan_protocol,
        scan_identifier,
        scan_aterisk,

        scan_string,
        scan_uint,
        scan_sint,
        scan_bool,
    ))(input)
}
