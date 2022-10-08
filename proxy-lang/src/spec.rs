//  SPEC.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 22:12:02
//  Last edited:
//    07 Oct 2022, 22:33:59
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs that do not have a specific
//!   relation to the AST (i.e., are not nodes in it).
// 

use std::fmt::Debug;


/***** LIBRARY *****/
/// Defines a position in the source text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextPos {
    /// The TextPos is defined (as a `line`, `column` pair).
    Some(usize, usize),
    /// The TextPos is undefined (i.e., generated post-parsing).
    None,
}

impl TextPos {
    /// Constructor for the TextPos that initializes it with the given values.
    /// 
    /// # Arguments
    /// - `line`: The line number for this position. Note that this value should be one-indexed.
    /// - `col`: The column number for this position. Note that this value should be one-indexed.
    #[inline]
    pub fn new(line: usize, col: usize) -> Self {
        Self::Some(
            line,
            col,
        )
    }



    /// Updates the line number stored within this TextPos.
    /// 
    /// Does nothing if the TextPos is `TextPos::None`.
    /// 
    /// # Arguments
    /// - `new_line`: The new line number to update the TextPos to.
    #[inline]
    pub fn set_ln(&mut self, new_line: usize) { if let TextPos::Some(line, _) = self { *line = new_line } }

    /// Updates the column number stored within this TextPos.
    /// 
    /// Does nothing if the TextPos is `TextPos::None`.
    /// 
    /// # Arguments
    /// - `new_col`: The new column number to update the TextPos to.
    #[inline]
    pub fn set_col(&mut self, new_col: usize) { if let TextPos::Some(_, col) = self { *col = new_col } }



    /// Returns the line number stored within this TextPos, if any.
    #[inline]
    pub fn ln(&self) -> Option<usize> { if let TextPos::Some(line, _) = self { Some(*line) } else { None } }

    /// Returns the column number stored within this TextPos, if any.
    #[inline]
    pub fn col(&self) -> Option<usize> { if let TextPos::Some(_, col) = self { Some(*col) } else { None } }
}



/// Defines a(n inclusive) range in the source text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextRange {
    /// The TextRange is defined (as a `start`, `stop` pair (inclusive)).
    Some(TextPos, TextPos),
    /// The TextPos is undefined (i.e., generated post-parsing).
    None,
}

impl TextRange {
    /// Constructor for the TextRange that initializes it with the given positions.
    /// 
    /// # Arguments
    /// - `start`: The start position for this range. Note that this value should be inclusive.
    /// - `stop`: The stop position for this range. Note that this value should be inclusive.
    #[inline]
    pub fn new(start: TextPos, stop: TextPos) -> Self {
        Self::Some(
            start,
            stop,
        )
    }



    /// Updates the start position stored within this TextRange.
    /// 
    /// Does nothing if the TextRange is `TextRange::None`.
    /// 
    /// # Arguments
    /// - `new_start`: The new start position to update the TextRange to.
    #[inline]
    pub fn set_start(&mut self, new_start: TextPos) { if let TextRange::Some(start, _) = self { *start = new_start } }

    /// Updates the stop position stored within this TextRange.
    /// 
    /// Does nothing if the TextRange is `TextRange::None`.
    /// 
    /// # Arguments
    /// - `new_stop`: The new stop position to update the TextRange to.
    #[inline]
    pub fn set_end(&mut self, new_stop: TextPos) { if let TextRange::Some(_, stop) = self { *stop = new_stop } }



    /// Returns the start position stored within this TextRange, if any. If none, then returns `TextPos::None`.
    #[inline]
    pub fn start(&self) -> TextPos { if let TextRange::Some(start, _) = self { *start } else { TextPos::None } }

    /// Returns the column number stored within this TextRange, if any. If none, then returns `TextPos::None`.
    #[inline]
    pub fn end(&self) -> TextPos { if let TextRange::Some(_, stop) = self { *stop } else { TextPos::None } }
}



/// Defines how a node in the AST looks like.
pub trait Node: Clone + Debug {
    /// Returns the entire range of the node in the parent source text.
    fn range(&self) -> TextRange;

    /// Returns the start position of this node's source text.
    #[inline]
    fn start(&self) -> TextPos { self.range().start() }

    /// Returns the end position of this node's source text.
    #[inline]
    fn end(&self) -> TextPos { self.range().end() }
}
