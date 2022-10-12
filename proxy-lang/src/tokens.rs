//  TOKENS.rs
//    by Lut99
// 
//  Created:
//    08 Oct 2022, 20:33:31
//  Last edited:
//    11 Oct 2022, 22:49:27
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the possible tokens produced by the scanner.
// 

use std::fmt::{Display, Formatter, Result as FResult};

use crate::spec::{Node, TextRange};


/***** LIBRARY *****/
/// Defines the possible tokens produced by the scanner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    /// A section header
    Section(String, TextRange),
    /// Some action
    Action(String, TextRange),
    /// Any protocol (e.g., `http://`)
    Protocol(String, TextRange),
    /// Any identifier / word / path element / w/e
    Identifier(String, TextRange),
    /// A port number (unparsed as of yet)
    Port(String, TextRange),
    /// An aterisk, possibly named.
    Aterisk(Option<String>, TextRange),
    /// A string literal
    String(String, TextRange),

    /// The arrow `->` symbol
    Arrow(TextRange),
    /// The colon `:` symbol
    Colon(TextRange),
    /// The slash `/` symbol
    Slash(TextRange),
    /// The dot `.` symbol
    Dot(TextRange),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Token::*;
        match self {
            Section(sec, _)   => write!(f, "SECTION<{}>", sec),
            Action(act, _)    => write!(f, "ACTION<{}>", act),
            Protocol(prot, _) => write!(f, "PROTOCOL<{}>", prot),
            Identifier(id, _) => write!(f, "IDENTIFIER<{}>", id),
            Port(port, _)     => write!(f, "PORT<{}>", port),
            Aterisk(name, _)  => write!(f, "ATERISK{}", if let Some(name) = name { format!("<{}>", name) } else { std::string::String::new() }),
            String(val, _)    => write!(f, "STRING<\"{}\">", val),

            Arrow(_) => write!(f, "ARROW"),
            Colon(_) => write!(f, "COLON"),
            Slash(_) => write!(f, "SLASH"),
            Dot(_)   => write!(f, "DOT"),
        }
    }
}

impl Node for Token {
    fn range(&self) -> TextRange {
        use Token::*;
        match self {
            Section(_, range)    => *range,
            Action(_, range)     => *range,
            Protocol(_, range)   => *range,
            Identifier(_, range) => *range,
            Port(_, range)       => *range,
            Aterisk(_, range)    => *range,
            String(_, range)     => *range,

            Arrow(range) => *range,
            Colon(range) => *range,
            Slash(range) => *range,
            Dot(range)   => *range,
        }
    }
}
