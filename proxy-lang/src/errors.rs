//  ERRORS.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 21:50:04
//  Last edited:
//    22 Oct 2022, 14:49:29
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

use console::{style, Style};

use crate::spec::Node;
use crate::source::SourceText;
use crate::tokens::{Token, TokenList};


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

    /// Prints the error as multiple errors in succession.
    /// 
    /// # Arguments
    /// - `f`: The Formatter to write to.
    /// 
    /// # Errors
    /// This function errors if we failed to write somehow. Any other errors should probably be panics, at this point (or handled gracefully).
    fn prettyprint_multiple(&self, _f: &mut Formatter<'_>) -> FResult { Ok(()) }



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
    NonEmptyTokenList{ remain: Vec<Token<SourceText>> },
    /// Failed to get the a token (got EOF instead).
    EofError{ expected: Token<SourceText> },
    /// Failed to get a token (got another one instead).
    UnexpectedTokenError{ got: Token<SourceText>, expected: Token<SourceText> },

    /// Failed to parse an unsigned integer
    UIntParseError{ raw: String, err: std::num::ParseIntError, source: Option<SourceText> },
    /// Failed to parse a signed integer
    SIntParseError{ raw: String, err: std::num::ParseIntError, source: Option<SourceText> },
    /// Failed to parse a boolean
    BoolParseError{ raw: String, source: Option<SourceText> },
    /// Failed to parse (nom error)
    NomError{ errs: Vec<(nom::error::ErrorKind, Option<SourceText>)> },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ParseError::*;
        match self {
            NonEmptyTokenList{ remain }           => write!(f, "Failed to parse all tokens (remaining: {})", remain.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(", ")),
            EofError{ expected }                  => write!(f, "Syntax error: expected {}, got EOF", expected),
            UnexpectedTokenError{ got, expected } => write!(f, "Syntax error: expected {}, got {}", got, expected),

            UIntParseError{ raw, err, .. } => write!(f, "Failed to parse '{}' as an unsigned integer: {}", raw, err),
            SIntParseError{ raw, err, .. } => write!(f, "Failed to parse '{}' as a signed integer: {}", raw, err),
            BoolParseError{ raw, .. }      => write!(f, "Failed to parse '{}' as a boolean", raw),
            NomError{ errs, .. }           => write!(f, "Syntax error: {}", errs.iter().map(|(e, _)| format!("{:?}", e)).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl Error for ParseError {}

impl<'a> nom::error::ParseError<TokenList<'a>> for ParseError {
    fn from_error_kind(input: TokenList<'a>, kind: nom::error::ErrorKind) -> Self {
        Self::NomError{ errs: vec![ (kind, if !input.is_empty() { if let (Some(lhs), Some(rhs)) = (input[0].source(), input[input.len() - 1].source()) { Some((lhs + rhs).into()) } else { None } } else { None }) ] }
    }

    fn append(input: TokenList<'a>, kind: nom::error::ErrorKind, other: Self) -> Self {
        if let ParseError::NomError { mut errs } = other {
            // Update the values
            errs.push((kind, if !input.is_empty() { if let (Some(lhs), Some(rhs)) = (input[0].source(), input[input.len() - 1].source()) { Some((lhs + rhs).into()) } else { None } } else { None }));

            // Done, store
            Self::NomError{ errs }
        } else {
            panic!("Cannot append non-NomError to ParseError");
        }
    }
}
impl<'a> nom::error::FromExternalError<TokenList<'a>, nom::Err<Self>> for ParseError {
    fn from_external_error(_input: TokenList<'a>, _kind: nom::error::ErrorKind, e: nom::Err<Self>) -> Self {
        match e {
            nom::Err::Error(e)      => e,
            nom::Err::Failure(e)    => e,
            nom::Err::Incomplete(_) => { panic!("Getting `nom::Err::Incomplete` in a nested ParseError should never happen!") },
        }
    }
}

impl PrettyError for ParseError {
    fn prettyprint_plain(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ParseError::*;
        match self {
            NonEmptyTokenList{ .. }    |
            EofError{ .. }             |
            UnexpectedTokenError{ .. } => {
                // Print the header with the message, that's all
                writeln!(f, "{}{}", style("error").bold().red(), style(format!(": {}", self)).bold())?;
                writeln!(f)?;
                Ok(())
            },

            // Ignore the rest (for other functions)
            _ => Ok(()),
        }
    }

    fn prettyprint_source(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ParseError::*;
        match self {
            UIntParseError{ source, .. } |
            SIntParseError{ source, .. } |
            BoolParseError{ source, .. } => {
                // Print the header with the message
                writeln!(f, "{}{}", style("error").bold().red(), style(format!(": {}", self)).bold())?;

                // Write the source reference, if any
                if let Some(source) = source {
                    write!(f, "{}", source.display(Style::new().bold().red()))?;
                }
                writeln!(f)?;

                // Done
                Ok(())
            },

            // Ignore the rest (for other functions)
            _ => Ok(()),
        }
    }

    fn prettyprint_multiple(&self, f: &mut Formatter<'_>) -> FResult {
        use self::ParseError::*;
        match self {
            NomError{ errs, .. } => {
                for (_, source) in errs {
                    // Print the header with the message
                    writeln!(f, "{}{}", style("error").bold().red(), style(format!(": {}", self)).bold())?;

                    // Write the source reference, if any
                    if let Some(source) = source {
                        write!(f, "{}", source.display(Style::new().bold().red()))?;
                    }
                    writeln!(f)?;
                }

                // Done
                Ok(())
            },

            // Ignore the rest (for other functions)
            _ => Ok(()),
        }
    }
}
