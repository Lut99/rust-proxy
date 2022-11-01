//  TOKENS.rs
//    by Lut99
// 
//  Created:
//    08 Oct 2022, 20:33:31
//  Last edited:
//    27 Oct 2022, 18:02:43
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the possible tokens produced by the scanner.
// 

use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::mem;
use std::ops::Index;

use crate::spec::Node;
use crate::source::{SourceRef, SourceText};


/***** AUXILLARY *****/
/// Defines a wrapper around a Token slice to implement all of the functions we need.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TokenList<'a> {
    /// The tokens themselves
    tokens : &'a [Token<SourceRef<'a>>],
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
    pub fn new(tokens: &'a [Token<SourceRef<'a>>]) -> Self {
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
    pub fn iter<'b>(&'a self) -> std::slice::Iter<'a, Token<SourceRef<'a>>> { self.into_iter() }
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
    type Item     = &'a Token<SourceRef<'a>>;
    type Iter     = std::iter::Enumerate<std::slice::Iter<'a, Token<SourceRef<'a>>>>;
    type IterElem = std::slice::Iter<'a, Token<SourceRef<'a>>>;

    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.iter()
    }

    fn iter_indices(&self) -> Self::Iter {
        self.tokens.iter().enumerate()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool
    {
        for (i, t) in self.iter_indices() {
            if predicate(t) { return Some(i); }
        }
        None
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if count > self.tokens.len() { return Err(nom::Needed::new(count - self.tokens.len())); }
        Ok(count)
    }
}

impl<'a> Index<usize> for TokenList<'a> {
    type Output = Token<SourceRef<'a>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}

impl<'a> IntoIterator for TokenList<'a> {
    type Item     = &'a Token<SourceRef<'a>>;
    type IntoIter = std::slice::Iter<'a, Token<SourceRef<'a>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter()
    }
}
impl<'a, 'b> IntoIterator for &'b TokenList<'a> {
    type Item     = &'a Token<SourceRef<'a>>;
    type IntoIter = std::slice::Iter<'a, Token<SourceRef<'a>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter()
    }
}





/***** LIBRARY *****/
/// Defines the possible tokens produced by the scanner.
#[derive(Clone, Debug, Eq)]
pub enum Token<T> {
    /// Some action
    Action(String, Option<T>),
    /// Any protocol (e.g., `http://`)
    Protocol(String, Option<T>),
    /// Any identifier / word / path element / w/e
    Identifier(String, Option<T>),
    /// A port number (unparsed as of yet)
    Port(String, Option<T>),
    /// An aterisk, possibly named.
    Aterisk(Option<String>, Option<T>),

    /// A string literal
    String(String, Option<T>),
    /// An unsigned integer
    UInt(String, Option<T>),
    /// A signed integer.
    SInt(String, Option<T>),
    /// A boolean value.
    Bool(String, Option<T>),

    /// The `[settings]` keyword/section
    SettingsSection(Option<T>),
    /// The `[rules]` keyword/section
    RulesSection(Option<T>),

    /// The arrow `->` symbol
    Arrow(Option<T>),
    /// The left square bracket
    LSquare(Option<T>),
    /// The right square bracket
    RSquare(Option<T>),
    /// The left curly bracket
    LCurly(Option<T>),
    /// The right curly bracket
    RCurly(Option<T>),
    /// The colon `:` symbol
    Colon(Option<T>),
    /// The slash `/` symbol
    Slash(Option<T>),
    /// The dot `.` symbol
    Dot(Option<T>),
    /// The comma `,` symbol
    Comma(Option<T>),
}

impl<T> Display for Token<T> {
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

impl<T> Node<T> for Token<T>
where
    T: Clone + Debug,
{
    fn source<'a>(&'a self) -> &'a Option<T> {
        use Token::*;
        match self {
            Action(_, source)     => source,
            Protocol(_, source)   => source,
            Identifier(_, source) => source,
            Port(_, source)       => source,
            Aterisk(_, source)    => source,

            String(_, source) => source,
            UInt(_, source)   => source,
            SInt(_, source)   => source,
            Bool(_, source)   => source,

            SettingsSection(source) => source,
            RulesSection(source)    => source,

            Arrow(source)   => source,
            LSquare(source) => source,
            RSquare(source) => source,
            LCurly(source)  => source,
            RCurly(source)  => source,
            Colon(source)   => source,
            Slash(source)   => source,
            Dot(source)     => source,
            Comma(source)   => source,
        }
    }
}

impl<T> PartialEq for Token<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<'a> From<Token<SourceRef<'a>>> for Token<SourceText> {
    fn from(value: Token<SourceRef<'a>>) -> Self {
        use Token::*;
        match value {
            Action(act, source)    => Action(act, source.map(|s| s.into())),
            Protocol(prot, source) => Protocol(prot, source.map(|s| s.into())),
            Identifier(id, source) => Identifier(id, source.map(|s| s.into())),
            Port(port, source)     => Port(port, source.map(|s| s.into())),
            Aterisk(name, source)  => Aterisk(name, source.map(|s| s.into())),

            String(val, source) => String(val, source.map(|s| s.into())),
            UInt(val, source)   => UInt(val, source.map(|s| s.into())),
            SInt(val, source)   => SInt(val, source.map(|s| s.into())),
            Bool(val, source)   => Bool(val, source.map(|s| s.into())),

            SettingsSection(source) => SettingsSection(source.map(|s| s.into())),
            RulesSection(source)    => RulesSection(source.map(|s| s.into())),

            Arrow(source)   => Arrow(source.map(|s| s.into())),
            LSquare(source) => LSquare(source.map(|s| s.into())),
            RSquare(source) => RSquare(source.map(|s| s.into())),
            LCurly(source)  => LCurly(source.map(|s| s.into())),
            RCurly(source)  => RCurly(source.map(|s| s.into())),
            Colon(source)   => Colon(source.map(|s| s.into())),
            Slash(source)   => Slash(source.map(|s| s.into())),
            Dot(source)     => Dot(source.map(|s| s.into())),
            Comma(source)   => Comma(source.map(|s| s.into())),
        }
    }
}
