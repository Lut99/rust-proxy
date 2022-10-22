//  SCANNER.rs
//    by Lut99
// 
//  Created:
//    08 Oct 2022, 20:45:32
//  Last edited:
//    22 Oct 2022, 15:57:25
//  Auto updated?
//    Yes
// 
//  Description:
//!   Implements the toplevel scanning functions.
// 

use std::io::Read;

use nom::IResult;
use nom::{branch, combinator as comb};

pub use crate::errors::ScanError as Error;
use crate::source::{SourceRef, SourceText};
use crate::scanner::{Input, Token};
use crate::scanner::whitespace;
use crate::scanner::comments;
use crate::scanner::punctuation;
use crate::scanner::keywords;
use crate::scanner::values;


/***** TESTS *****/
#[cfg(test)]
mod tests {
    use crate::spec::Node;
    use crate::tests::run_test_on_files;
    use super::*;


    /// Runs tests on the files in the tests folder.
    #[test]
    fn test_files() {
        run_test_on_files(|path, source| {
            // Run the scanner
            let tokens: Vec<crate::tokens::Token<SourceText>> = match scan(&format!("{}", path.display()), source.as_bytes()) {
                Ok(tokens) => tokens,
                Err(err)   => { panic!("Scanner failed: {}", err); },
            };

            // Print what is happening
            let mut last_line: usize = 1;
            for t in tokens {
                if t.source().as_ref().unwrap().start().0 != last_line {
                    last_line = t.source().as_ref().unwrap().start().0;
                    println!();
                }
                print!("{} ", t);
            }
            println!();
        });
    }
}





/***** HELPER FUNCTIONS *****/
/// Scans a single token.
/// 
/// # Arguments
/// - `input`: The input text to scan.
/// 
/// # Returns
/// The Token if we were able to parse one.
/// 
/// # Errors
/// A nom error if we failed (either because no parser matched or because there was a genuine error).
fn scan_token<'a, E: nom::error::ParseError<Input<'a>>>(input: Input<'a>) -> IResult<Input<'a>, Option<Token<'a>>, E> {
    branch::alt((
        comb::value(
            None,
            whitespace::scan,
        ),
        comb::value(
            None,
            comments::scan,
        ),

        comb::map(
            punctuation::scan,
            |p| Some(p),
        ),
        comb::map(
            keywords::scan,
            |k| Some(k),
        ),
        comb::map(
            values::scan,
            |v| Some(v),
        ),
    ))(input)
}





/***** LIBRARY *****/
/// Parse the given source text as a stream of tokens.
/// 
/// # Arguments
/// - `file`: Some name / path that the user can use to identify the given reader.
/// - `reader`: The reader that contains the source text to read from.
/// 
/// # Returns
/// The vector of Tokens that are parsed.
/// 
/// # Errors
/// This function errors if the input was ill-formed.
pub fn scan(file: impl AsRef<str>, reader: impl Read) -> Result<Vec<crate::tokens::Token<SourceText>>, Error> {
    let file: &str = file.as_ref();
    let mut reader = reader;

    // Consume the reader to string
    let mut source: String = String::new();
    if let Err(err) = reader.read_to_string(&mut source) {
        return Err(Error::ReaderReadError{ file: file.into(), err });
    }

    // Parse tokens until eof
    let mut input  : SourceRef  = SourceRef::new(file, &source);
    let mut tokens : Vec<Token> = vec![];
    while !input.is_empty() {
        // Parse it
        match scan_token::<nom::error::VerboseError<SourceRef>>(input) {
            Ok((rest, Some(token))) => {
                tokens.push(token);
                input = rest;
            },
            Ok((rest, None))        => {
                input = rest;
            },

            Err(err) => { return Err(Error::ScanError{ err: format!("{}", err) }); },
        }
    }

    // Done, return the list
    Ok(tokens.into_iter().map(|t| t.into()).collect())
}
