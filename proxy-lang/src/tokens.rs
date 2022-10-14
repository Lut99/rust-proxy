//  TOKENS.rs
//    by Lut99
// 
//  Created:
//    08 Oct 2022, 20:33:31
//  Last edited:
//    14 Oct 2022, 11:55:18
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the possible tokens produced by the scanner.
// 

use std::fmt::{Display, Formatter, Result as FResult};
use std::mem;

use crate::spec::{Node, TextRange};


/***** AUXILLARY *****/
/// Defines a wrapper around a Token slice to implement all of the functions we need.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TokenList<'a> {
    /// The tokens themselves
    tokens : &'a [Token],
}

impl<'a> TokenList<'a> {
    /// Constructor for the TokenList that initializes it from a slice of tokens.
    /// 
    /// # Arguments
    /// - `tokens`: The tokens to wrap around.
    /// 
    /// # Returns
    /// A new TokenList instance.
    #[inline]
    pub fn new(tokens: impl 'a + AsRef<[Token]>) -> Self {
        Self {
            tokens : tokens.as_ref(),
        }
    }



    /// Returns whether there are no Tokens in this list (true) or if there is at least one (false).
    #[inline]
    pub fn is_empty(&self) -> bool { self.tokens.is_empty() }

    /// Returns the number of Tokens in the list.
    #[inline]
    pub fn len(&self) -> usize { self.tokens.len() }



    /// Returns an iterator over the TokenList.
    #[inline]
    pub fn iter<'b>(&'a self) -> std::slice::Iter<'a, Token> { self.into_iter() }

    /// Returns a muteable iterator over the TokenList.
    #[inline]
    pub fn iter_mut<'b>(&'a mut self) -> std::slice::IterMut<'a, Token> { self.into_iter() }
}

impl<'a> nom::InputTake for TokenList<'a> {
    fn take(&self, count: usize) -> Self {
        if count > self.len() { panic!("Cannot take {} elements from TokenList of {} elements", count, self.len()); }
        TokenList::new(&self.tokens[..count])
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        if count > self.len() { panic!("Cannot split {} elements from TokenList of {} elements", count, self.len()); }
        (TokenList::new(&self.tokens[..count]), TokenList::new(&self.tokens[count..]))
    }
}
impl<'a> nom::InputLength for TokenList<'a> {
    fn input_len(&self) -> usize {
        self.len()
    }
}
impl<'a> nom::InputIter for TokenList<'a> {
    type Item     = &'a Token;
    type Iter     = std::iter::Enumerate<std::slice::Iter<'a, Token>>;
    type IterElem = std::slice::Iter<'a, Token>;

    fn iter_elements(&self) -> Self::IterElem {
        
    }

    fn iter_indices(&self) -> Self::Iter {
        
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool
    {
        
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        
    }
}

impl<'a> IntoIterator for TokenList<'a> {
    type Item     = &'a Token;
    type IntoIter = std::slice::Iter<'a, Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter()
    }
}
impl<'a, 'b> IntoIterator for &'b TokenList<'a> {
    type Item     = &'a Token;
    type IntoIter = std::slice::Iter<'a, Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter()
    }
}
impl<'a, 'b> IntoIterator for &'b mut TokenList<'a> {
    type Item     = &'a mut Token;
    type IntoIter = std::slice::IterMut<'a, Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter_mut()
    }
}





/***** LIBRARY *****/
/// Defines the possible tokens produced by the scanner.
#[derive(Clone, Debug, Eq)]
pub enum Token {
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
    /// An unsigned integer
    UInt(String, TextRange),
    /// A signed integer.
    SInt(String, TextRange),
    /// A boolean value.
    Bool(String, TextRange),

    /// The `[settings]` keyword/section
    SettingsSection(TextRange),
    /// The `[rules]` keyword/section
    RulesSection(TextRange),

    /// The arrow `->` symbol
    Arrow(TextRange),
    /// The left square bracket
    LSquare(TextRange),
    /// The right square bracket
    RSquare(TextRange),
    /// The left curly bracket
    LCurly(TextRange),
    /// The right curly bracket
    RCurly(TextRange),
    /// The colon `:` symbol
    Colon(TextRange),
    /// The slash `/` symbol
    Slash(TextRange),
    /// The dot `.` symbol
    Dot(TextRange),
    /// The comma `,` symbol
    Comma(TextRange),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        use Token::*;
        match self {
            Action(act, _)    => write!(f, "ACTION<{}>", act),
            Protocol(prot, _) => write!(f, "PROTOCOL<{}>", prot),
            Identifier(id, _) => write!(f, "IDENTIFIER<{}>", id),
            Port(port, _)     => write!(f, "PORT<{}>", port),
            Aterisk(name, _)  => write!(f, "ATERISK{}", if let Some(name) = name { format!("<{}>", name) } else { std::string::String::new() }),

            String(val, _)    => write!(f, "STRING<\"{}\">", val),
            UInt(val, _)      => write!(f, "UINT<{}>", val),
            SInt(val, _)      => write!(f, "SINT<{}>", val),
            Bool(val, _)      => write!(f, "BOOL<{}>", val),

            SettingsSection(_) => write!(f, "SETTINGS_SECTION"),
            RulesSection(_)    => write!(f, "RULES_SECTION"),

            Arrow(_)   => write!(f, "ARROW"),
            LSquare(_) => write!(f, "LSQUARE"),
            RSquare(_) => write!(f, "RSQUARE"),
            LCurly(_)  => write!(f, "LCURLY"),
            RCurly(_)  => write!(f, "RCURLY"),
            Colon(_)   => write!(f, "COLON"),
            Slash(_)   => write!(f, "SLASH"),
            Dot(_)     => write!(f, "DOT"),
            Comma(_)   => write!(f, "COMMA"),
        }
    }
}

impl Node for Token {
    fn range(&self) -> TextRange {
        use Token::*;
        match self {
            Action(_, range)     => *range,
            Protocol(_, range)   => *range,
            Identifier(_, range) => *range,
            Port(_, range)       => *range,
            Aterisk(_, range)    => *range,

            String(_, range) => *range,
            UInt(_, range)   => *range,
            SInt(_, range)   => *range,
            Bool(_, range)   => *range,

            SettingsSection(range) => *range,
            RulesSection(range)    => *range,

            Arrow(range)   => *range,
            LSquare(range) => *range,
            RSquare(range) => *range,
            LCurly(range)  => *range,
            RCurly(range)  => *range,
            Colon(range)   => *range,
            Slash(range)   => *range,
            Dot(range)     => *range,
            Comma(range)   => *range,
        }
    }
}

impl PartialEq for Token {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
