//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 21:50:04
//  Last edited:
//    11 Oct 2022, 23:21:10
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

use crate::spec::TokenList;


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
    NonEmptyTokenList{ remain: TokenList },
    /// Failed to parse (nom error)
    ParseError{ err: nom::Err<nom::error::VerboseError<TokenList>> },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ParseError::*;
        match self {
            NonEmptyTokenList{ remain } => write!(f, "Failed to parse all tokens (remaining: {})", remain.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(", ")),
            ParseError{ err }           => write!(f, "Syntax error: {}", err),
        }
    }
}

impl Error for ParseError {}

impl PrettyError for ParseError {
    fn prettyprint_plain(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ParseError::*;
        match self {
            NonEmptyTokenList{ .. } => error!(f, "{}", self),
            ParseError{ .. }        => error!(f, "{}", self),
        }
    }
}
