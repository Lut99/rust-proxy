//  COMMENTS.rs
//    by Lut99
// 
//  Created:
//    08 Oct 2022, 20:54:53
//  Last edited:
//    11 Oct 2022, 18:27:53
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements nom functions for scanning comments.
// 

use nom::IResult;
use nom::{branch, bytes::complete as bc, combinator as comb, multi, sequence as seq};

use crate::spec::Input;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use crate::tests::input;
    use super::*;

    #[test]
    fn test_comments() {
        // Simply attempt to parse some comment stuff
        assert_eq!(scan::<nom::error::Error<Input>>(input!("// Hello there!")).ok(), Some((input!("", 15), ())));
        assert_eq!(scan::<nom::error::Error<Input>>(input!("/* Hello there! */")).ok(), Some((input!("", 18), ())));

        // Parse a multiple comment
        let (r, _) = scan::<nom::error::Error<Input>>(input!("// Hello there!\n/* Hello there! */")).unwrap();
        let (r, _) = scan::<nom::error::Error<Input>>(r).unwrap();
        assert_eq!(r, input!("", 34, 2));
    }
}





/***** HELPER FUNCTIONS *****/
/// Scans a comment starting with '//'.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// Nothing on success (since we don't wanna parse comments).
/// 
/// # Errors
/// This function may error if nom failed to scan a comment.
fn scan_singleline<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, (), E> {
    comb::value(
        (),
        seq::preceded(
            bc::tag("// "),
            multi::many_till(
                seq::pair(
                    comb::not(branch::alt((
                        bc::tag("\n"),
                        comb::eof,
                    ))),
                    bc::take(1usize),
                ),
                branch::alt((
                    bc::tag("\n"),
                    comb::eof,
                )),
            ),
        ),
    )(input)
}

/// Scans a comment starting with '/*' and ending with `*/` (multiline).
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// Nothing on success (since we don't wanna parse comments).
/// 
/// # Errors
/// This function may error if nom failed to scan a comment.
fn scan_multiline<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, (), E> {
    comb::value(
        (),
        seq::preceded(
            bc::tag("/* "),
            multi::many_till(
                seq::pair(
                    comb::not(bc::tag("*/")),
                    bc::take(1usize),
                ),
                bc::tag("*/"),
            ),
        ),
    )(input)
}





/***** LIBRARY *****/
/// Scans one of the possible comments.
/// 
/// # Arguments
/// - `input`: The Input to scan.
/// 
/// # Returns
/// Nothing on success (since we don't wanna parse comments).
/// 
/// # Errors
/// This function may error if nom failed to scan a comment.
pub fn scan<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, (), E> {
    branch::alt((
        scan_singleline,
        scan_multiline,
    ))(input)
}
