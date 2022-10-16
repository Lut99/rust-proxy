//  SPEC.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 22:12:02
//  Last edited:
//    16 Oct 2022, 17:04:33
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs that do not have a specific
//!   relation to the AST (i.e., are not nodes in it).
// 

use std::fmt::Debug;
use std::ops::Deref;
use std::ptr;

use nom_locate::LocatedSpan;

use crate::tokens::Token;


/***** LIBRARY *****/
// /// Defines the input used to the scanner.
// pub type Input<'a> = LocatedSpan<&'a str>;

/// Defines a reference for source text such that it can be used to link errors to it.
#[derive(Clone, Copy, Debug, Eq)]
pub struct SourceRef<'a> {
    // Actual text reference (used to produce the source)
    /// Reference to the source text as a whole
    source : &'a str,
    /// The offset of this piece of source text in the original source.
    offset : usize,
    /// The length (in number of characters/bytes) in the original source.
    size   : usize,

    // Debug data (used to produce the entire line)
    /// Reference to the source's name (probably a filename, but might also be things like `<test>` or `<stdin>`).
    name  : &'a str,
    /// Defines the start of the source reference as a `(line, col)` pair (inclusive). Both are one-indexed.
    start : (usize, usize),
    /// Defines the end of the source reference as a `(line, col)` pair (inclusive). Both are one-indexed.
    end   : (usize, usize),
}

impl<'a> Deref for SourceRef<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Sanity checks
        if self.offset             >= self.source.len() { panic!("Offset {} is out-of-bounds for source text '{}' of {} characters", self.offset, self.name, self.source.len()); }
        if self.offset + self.size >  self.source.len() { panic!("Offset + size {} is out-of-bounds for source text '{}' of {} characters", self.offset + self.size, self.name, self.source.len()); }

        // Return the referenced part of the source text
        &self.source[self.offset..self.offset + self.size]
    }
}

impl<'a> PartialEq for SourceRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.source, other.source) && self.offset == other.offset && self.size == other.size
    }
}



// /// Defines a position in the source text.
// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub enum TextPos {
//     /// The TextPos is defined (as a `line`, `column` pair).
//     Some(usize, usize),
//     /// The TextPos is undefined (i.e., generated post-parsing).
//     None,
// }

// impl TextPos {
//     /// Constructor for the TextPos that initializes it with the given values.
//     /// 
//     /// # Arguments
//     /// - `line`: The line number for this position. Note that this value should be one-indexed.
//     /// - `col`: The column number for this position. Note that this value should be one-indexed.
//     #[inline]
//     pub fn new(line: usize, col: usize) -> Self {
//         Self::Some(
//             line,
//             col,
//         )
//     }

//     /// Constructor for the TextPos that initializes it pointing to the _first_ character of the given span.
//     /// 
//     /// # Arguments
//     /// - `input`: The Input span to take the initial position from.
//     /// 
//     /// # Returns
//     /// A new TextPos instance.
//     /// 
//     /// # Panics
//     /// This function panics if the given span was empty.
//     pub fn start_of<'a>(input: &Input<'a>) -> Self {
//         if input.is_empty() { panic!("Cannot call `TextPos::start_of()` on empty Input"); }
//         Self::Some(input.location_line() as usize, input.get_column())
//     }

//     /// Constructor for the TextPos that initializes it pointing to the _last_ character of the given span.
//     /// 
//     /// # Arguments
//     /// - `input`: The Input span to take the last position from.
//     /// 
//     /// # Returns
//     /// A new TextPos instance.
//     /// 
//     /// # Panics
//     /// This function panics if the given span was empty.
//     pub fn end_of<'a>(input: &Input<'a>) -> Self {
//         if input.is_empty() { panic!("Cannot call `TextPos::end_of()` on empty Input"); }

//         // Get the starting position and move to the end of the thing
//         let (mut line, mut col): (usize, usize) = (input.location_line() as usize, input.get_column());
//         for c in input.fragment().chars().skip(1) {
//             if c == '\n' {
//                 line += 1;
//                 col   = 1;
//             } else {
//                 col += 1;
//             }
//         }

//         // Done, use that as self
//         Self::Some(line, col)
//     }



//     /// Updates the line number stored within this TextPos.
//     /// 
//     /// Does nothing if the TextPos is `TextPos::None`.
//     /// 
//     /// # Arguments
//     /// - `new_line`: The new line number to update the TextPos to.
//     #[inline]
//     pub fn set_line(&mut self, new_line: usize) { if let TextPos::Some(line, _) = self { *line = new_line } }

//     /// Updates the column number stored within this TextPos.
//     /// 
//     /// Does nothing if the TextPos is `TextPos::None`.
//     /// 
//     /// # Arguments
//     /// - `new_col`: The new column number to update the TextPos to.
//     #[inline]
//     pub fn set_col(&mut self, new_col: usize) { if let TextPos::Some(_, col) = self { *col = new_col } }



//     /// Returns the line number stored within this TextPos, if any.
//     #[inline]
//     pub fn line(&self) -> Option<usize> { if let TextPos::Some(line, _) = self { Some(*line) } else { None } }

//     /// Returns the column number stored within this TextPos, if any.
//     #[inline]
//     pub fn col(&self) -> Option<usize> { if let TextPos::Some(_, col) = self { Some(*col) } else { None } }
// }



// /// Defines a(n inclusive) range in the source text.
// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// pub enum TextRange {
//     /// The TextRange is defined (as a `start`, `stop` pair (inclusive)).
//     Some(TextPos, TextPos),
//     /// The TextPos is undefined (i.e., generated post-parsing).
//     None,
// }

// impl TextRange {
//     /// Constructor for the TextRange that initializes it with the given positions.
//     /// 
//     /// # Arguments
//     /// - `start`: The start position for this range. Note that this value should be inclusive.
//     /// - `stop`: The stop position for this range. Note that this value should be inclusive.
//     #[inline]
//     pub fn new(start: TextPos, stop: TextPos) -> Self {
//         Self::Some(
//             start,
//             stop,
//         )
//     }



//     /// Updates the start position stored within this TextRange.
//     /// 
//     /// Does nothing if the TextRange is `TextRange::None`.
//     /// 
//     /// # Arguments
//     /// - `new_start`: The new start position to update the TextRange to.
//     #[inline]
//     pub fn set_start(&mut self, new_start: TextPos) { if let TextRange::Some(start, _) = self { *start = new_start } }

//     /// Updates the stop position stored within this TextRange.
//     /// 
//     /// Does nothing if the TextRange is `TextRange::None`.
//     /// 
//     /// # Arguments
//     /// - `new_stop`: The new stop position to update the TextRange to.
//     #[inline]
//     pub fn set_end(&mut self, new_stop: TextPos) { if let TextRange::Some(_, stop) = self { *stop = new_stop } }



//     /// Returns the start position stored within this TextRange, if any. If none, then returns `TextPos::None`.
//     #[inline]
//     pub fn start(&self) -> TextPos { if let TextRange::Some(start, _) = self { *start } else { TextPos::None } }

//     /// Returns the column number stored within this TextRange, if any. If none, then returns `TextPos::None`.
//     #[inline]
//     pub fn end(&self) -> TextPos { if let TextRange::Some(_, stop) = self { *stop } else { TextPos::None } }
// }

// impl<'a> From<Input<'a>> for TextRange {
//     #[inline]
//     fn from(value: Input<'a>) -> TextRange {
//         TextRange::from(&value)
//     }
// }

// impl<'a> From<&Input<'a>> for TextRange {
//     #[inline]
//     fn from(value: &Input<'a>) -> TextRange {
//         TextRange::Some(TextPos::start_of(value), TextPos::end_of(value))
//     }
// }



/// Defines how a node in the AST looks like.
pub trait Node: Clone + Debug {
    // /// Returns the entire range of the node in the parent source text.
    // fn range(&self) -> TextRange;

    // /// Returns the start position of this node's source text.
    // #[inline]
    // fn start(&self) -> TextPos { self.range().start() }

    // /// Returns the end position of this node's source text.
    // #[inline]
    // fn end(&self) -> TextPos { self.range().end() }



    // Child-overridable
    /// Returns the reference to the source text that this node is created from. If it does not have such an origin, returns None.
    /// 
    /// # Returns
    /// The SourceRef if this Node had a source.
    /// 
    /// # Errors
    /// This function may error for child-specific reasons.
    /// 
    /// # Panics
    /// This function may panic for child-specific reasons.
    fn get_source_ref<'a>(&'a self) -> Option<SourceRef<'a>>;



    // Global
    /// Returns the lines in the source text that are spanned by the SourceRef returned by `Self::get_source_ref()`.
    /// 
    /// Note that `Self::get_source_ref()` may return `None` if this Node did not originate from the source (i.e., was created / inserted later). In that case, this function returns None too.
    /// 
    /// # Returns
    /// Multiple slices of the string in the source text that is referred by this Node's SourceRef (unless `Self::get_source_ref()` returns `None`).
    /// 
    /// # Panics
    /// This function panics if, for some reason, offsets and such are out-of-range for the given source.
    fn get_source_lines<'a>(&'a self) -> Option<Vec<&'a str>> {
        // Get the reference
        let sref : SourceRef = match self.get_source_ref() {
            Some(sref) => sref,
            None       => { return None; },
        };

        // Sanity check
        if sref.offset             >= sref.source.len() { panic!("Offset {} is out-of-bounds in source text '{}' with {} characters", sref.offset, sref.name, sref.source.len()); }
        if sref.offset + sref.size >  sref.source.len() { panic!("Offset + size {} is out-of-bounds in source text '{}' with {} characters", sref.offset + sref.size, sref.name, sref.source.len()); }

        // Search for the line start's position
        let mut start : Option<usize> = None;
        for (i, c) in sref.source[..sref.offset + 1].char_indices().rev() {
            if c == '\n' { start = Some(i); }
        }
        let mut start : usize = match start {
            Some(start) => start,
            None        => 0,
        };

        // From there on out we search the end of the lines
        let mut res : Vec<&'a str> = Vec::with_capacity(1);
        for (i, c) in sref.source[start..].char_indices() {
            // If it's a newline, push onto the result list
            if c == '\n' {
                res.push(&sref.source[start..i + 1]);
                start = i + 1;

                // If we've exceeded the end point, we're good to stop
                if i >= sref.offset + sref.size { break; }
            }
        }

        // Done
        Some(res)
    }
}
