//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 21:50:04
//  Last edited:
//    13 Oct 2022, 10:56:07
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the errors that occur in the `proxy-lang` crate. Note that
//!   this are typically more informative errors than normal ones, since
//!   in this case, errors are a valid (and even expected) execution path
//!   for the program.
// 

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FResult};

use console::style;

use crate::spec::{Node, TextRange};
use crate::tokens::Token;


/***** HELPER MACROS *****/
/// Prints 'error: ' with proper formatting.
macro_rules! error {
    ($f:ident) => {
        write!($f, "{}: ", style("error").red().bold())
    };
    ($f:ident, $fmt:literal, $($t:tt)*) => {
        write!($f, concat!("{}: ", $fmt), style("error").red().bold(), $($t)*)
    };
}





/***** AUXILLARY *****/
/// Defines a helper struct that can pretty print the given error.
#[derive(Debug)]
pub struct ErrorPrettyPrinter<'a> {
    /// The error to pretty print.
    err : &'a dyn PrettyError,
}

impl<'a> Display for ErrorPrettyPrinter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        self.err.prettyprint_fmt(f)
    }
}



/// Marks that an error is a pretty error.
pub trait PrettyError: Error {
    // Child overloadable
    /// Prints the error as an error with no relation to the source text.
    /// 
    /// # Arguments
    /// - `f`: The Formatter to write to.
    /// 
    /// # Errors
    /// This function errors if we failed to write somehow. Any other errors should probably be panics, at this point (or handled gracefully).
    fn prettyprint_plain(&self, _f: &mut Formatter<'_>) -> FResult { Ok(()) }

    /// Prints the error as a simple error with a marked area in the source text.
    /// 
    /// # Arguments
    /// - `f`: The Formatter to write to.
    /// 
    /// # Errors
    /// This function errors if we failed to write somehow. Any other errors should probably be panics, at this point (or handled gracefully).
    fn prettyprint_source(&self, _f: &mut Formatter<'_>) -> FResult { Ok(()) }



    // Global
    /// Returns an ErrorPrettyPrint object that prettyprints this error.
    /// 
    /// # Returns
    /// An ErrorPrettyPrint object that implements Display.
    #[inline]
    fn prettyprint<'a>(&'a self) -> ErrorPrettyPrinter<'a> where Self: Sized {
        ErrorPrettyPrinter {
            err : self,
        }
    }

    /// Prettyprints the PrettyError by calling all of its methods. Only those defined will then produce a result.
    /// 
    /// # Arguments
    /// - `f`: The Formatter to write to.
    /// 
    /// # Errors
    /// This function errors if we failed to write somehow. Any other errors should probably be panics, at this point (or handled gracefully).
    fn prettyprint_fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Try them all, in-order
        self.prettyprint_plain(f)?;
        self.prettyprint_source(f)?;

        // Done
        Ok(())
    }
}





/***** LIBRARY *****/
/// Defines errors that may occur during scanning.
#[derive(Debug)]
pub enum ScanError {
    /// Failed to read the given reader as source text.
    ReaderReadError{ file: String, err: std::io::Error },
    /// Failed to scan (nom error)
    ScanError{ err: String },
}

impl Display for ScanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ScanError::*;
        match self {
            ReaderReadError{ file, err } => write!(f, "Failed to read from input '{}': {}", file, err),
            ScanError{ err }             => write!(f, "Syntax error: {}", err),
        }
    }
}

impl Error for ScanError {}

impl PrettyError for ScanError {
    fn prettyprint_plain(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ScanError::*;
        match self {
            ReaderReadError{ .. } => error!(f, "{}", self),
            ScanError{ .. }       => error!(f, "{}", self),
        }
    }
}



/// Defines errors that may occur during parsing.
#[derive(Debug)]
pub enum ParseError {
    /// Failed to read the given reader as source text.
    NonEmptyTokenList{ remain: Vec<Token> },

    /// Failed to parse an unsigned integer
    UIntParseError{ raw: String, err: std::num::ParseIntError, range: TextRange },
    /// Failed to parse a signed integer
    SIntParseError{ raw: String, err: std::num::ParseIntError, range: TextRange },
    /// Failed to parse a boolean
    BoolParseError{ raw: String, range: TextRange },
    /// Failed to parse (nom error)
    NomError{ errs: Vec<nom::error::ErrorKind>, ranges: Vec<TextRange> },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ParseError::*;
        match self {
            NonEmptyTokenList{ remain } => write!(f, "Failed to parse all tokens (remaining: {})", remain.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(", ")),

            UIntParseError{ raw, err, .. } => write!(f, "Failed to parse '{}' as an unsigned integer: {}", raw, err),
            SIntParseError{ raw, err, .. } => write!(f, "Failed to parse '{}' as a signed integer: {}", raw, err),
            BoolParseError{ raw, .. }      => write!(f, "Failed to parse '{}' as a boolean", raw),
            NomError{ errs, .. }           => write!(f, "Syntax error: {}", errs.iter().map(|e| format!("{:?}", e)).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl Error for ParseError {}

impl<'a> nom::error::ParseError<&'a [Token]> for ParseError {
    fn from_error_kind(input: &'a [Token], kind: nom::error::ErrorKind) -> Self {
        Self::NomError{ errs: vec![ kind ], ranges: vec![ if !input.is_empty() { TextRange::new(input[0].start(), input[input.len() - 1].end()) } else { TextRange::None } ] }
    }

    fn append(input: &'a [Token], kind: nom::error::ErrorKind, other: Self) -> Self {
        let ParseError::NomError { mut errs, mut ranges } = other;

        // Update the values
        errs.push(kind);
        ranges.push(if !input.is_empty() { TextRange::new(input[0].start(), input[input.len() - 1].end()) } else { TextRange::None });

        // Done, store
        Self::NomError{ errs, ranges }
    }
}
impl<'a> nom::error::FromExternalError<&'a [Token], nom::Err<Self>> for ParseError {
    fn from_external_error(input: &'a [Token], kind: nom::error::ErrorKind, e: nom::Err<Self>) -> Self {
        match e {
            nom::Err::Error(e)      => e,
            nom::Err::Failure(e)    => e,
            nom::Err::Incomplete(e) => { panic!("Getting `nom::Err::Incomplete` in a nested ParseError should never happen!") },
        }
    }
}

impl PrettyError for ParseError {
    fn prettyprint_plain(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ParseError::*;
        match self {
            NonEmptyTokenList{ .. } => error!(f, "{}", self),

            UIntParseError{ range, .. } => error!(f, "{}", self),
            SIntParseError{ range, .. } => error!(f, "{}", self),
            BoolParseError{ range, .. } => error!(f, "{}", self),
            NomError{ ranges, .. }      => error!(f, "{}", self),
        }
    }
}
