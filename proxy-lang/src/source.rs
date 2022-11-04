//  SOURCE.rs
//    by Lut99
// 
//  Created:
//    17 Oct 2022, 19:29:02
//  Last edited:
//    04 Nov 2022, 08:18:42
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines the `SourceText` and `SourceRef` structs, which can both be
//!   used to link AST nodes back to their parsed location in the source
//!   text. Furthermore, they are both [nom](https://github.com/Geal/nom)
//!   compatible and can thus be used as easy input wrappers (much like
//!   `LocatedSpan` in [nom_locate](https://github.com/Geal/nom).
// 

use std::fmt::{Debug, Display, Formatter, Result as FResult};
use std::ops::{Add, AddAssign, RangeFrom};

use console::{style, Style};
use nom::CompareResult;


/***** HELPER MACROS *****/
/// Generates a string of the given number of spaces.
macro_rules! spaces {
    ($n:expr) => {
        (0..$n).map(|_| ' ').collect::<String>()
    };
}




/***** TESTS *****/
#[cfg(test)]
mod tests {
    use crate::tests::assert_scan;
    use super::*;

    #[test]
    fn test_source() {
        // Create some random source
        assert_scan!(nom::combinator::value((), nom::bytes::complete::tag::<&str, SourceRef, nom::error::VerboseError<SourceRef>>("//")), "// Hello there!", 2);
    }
}





/***** AUXILLARY *****/
/// Auxillary struct that can write a SourceRef or SourceText to the given writer.
pub struct SourceTextDisplay<'a, T> {
    /// The thing to display.
    source : &'a T,
    /// The style (general colour) to display it with.
    style  : Style,
}

impl<'a, T> Display for SourceTextDisplay<'a, T>
where
    SourceText: From<&'a T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        // Get the proper source text
        let source: SourceText = self.source.into();
        if source.size() == 0 { return Ok(()); }
        // Quick helper closure for deciding when the current (line, col) is within the range
        let is_in_range = |i: usize, j: usize| -> bool {
            let start : (usize, usize) = source.start();
            let end   : (usize, usize) = source.end();

            // Switch on multi-line mode or not
            if start.0 == end.0 {
                i == start.0 && j >= start.1 - 1 && j <= end.1 - 1
            } else {
                (i == start.0 && j >= start.1 - 1) || (i > start.0 && i < end.0) || (i == end.0 && j <= end.1 - 1)
            }
        };

        // Compute the maximum line length
        let max_line_len: usize = ((source.end().0 as f32).log10() + 1.0).floor() as usize;

        // Write the file thingy + a "whitespace"
        println!("{:?}", source);
        writeln!(f, "{}{} {}:{}:{}", spaces!(max_line_len), style("-->").bright().blue(), source.name(), source.start().0, source.start().1)?;
        writeln!(f, "{} {}", spaces!(max_line_len), style("|").bright().blue())?;

        // Write the lines that are marked
        for (i, l) in source.lines() {
            // Write the start of the line with context
            let sline: String = format!("{}", i);
            write!(f, "{}{} {} ", spaces!(max_line_len - sline.len()), sline, style("|").bright().blue())?;
    
            // Start writing the line itself, highlighing what is necessary
            for (j, c) in l.char_indices() {
                
                if is_in_range(i, j) {
                    write!(f, "{}", self.style.apply_to(c))?;
                } else {
                    write!(f, "{}", c)?;
                }
            }

            // Write the end-of-line
            writeln!(f)?;

            // Now go in again, applying the marker thingies

            // Write the start of the line
            write!(f, "{} {} ", spaces!(max_line_len), style("|").bright().blue())?;
            for (j, _) in l.char_indices() {
                if is_in_range(i, j) {
                    write!(f, "{}", self.style.apply_to('^'))?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        // Done
        Ok(())
    }
}





/***** LIBRARY *****/
/// Defines a reference for source text such that it can be used to link errors to it.
#[derive(Clone, Copy)]
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
}

impl<'a> SourceRef<'a> {
    /// Constructor for the SourceRef that creates it from the given "filename" and source text.
    /// 
    /// # Arguments
    /// - `name`: The (file)name of the source text. Should basically be some way for the user to identify the origin of the source text.
    /// - `source`: The actual source text itself.
    /// 
    /// # Returns
    /// A new SourceRef instance.
    #[inline]
    pub fn new(name: &'a str, source: &'a str) -> Self {
        let source_len: usize = source.len();
        Self {
            source,
            offset : 0,
            size   : source_len,

            name,
        }
    }

    /// Unsafe function that creates a SourceRef with custom offset & size.
    /// 
    /// Be careful they are in the range of the given source!
    /// 
    /// # Arguments
    /// - `name`: The (file)name of the source text. Should basically be some way for the user to identify the origin of the source text.
    /// - `source`: The actual source text itself.
    /// - `offset`: The offset of this reference's fragment in the larger source text.
    /// - `size`: The size of this reference's fragment in the larger source text.
    /// 
    /// # Returns
    /// A new SourceRef instance.
    #[inline]
    pub unsafe fn new_with_raw_offset(name: &'a str, source: &'a str, offset: usize, size: usize) -> Self {
        Self {
            source,
            offset,
            size,

            name,
        }
    }



    /// Gros the SourceRef by the given amount to the right.
    /// 
    /// # Panics
    /// This function panics if this causes the SourceRef to go out-of-bounds.
    pub fn enlarge(&mut self, n: usize) {
        if self.offset + self.size + n > self.source.len() { panic!("Enlarging a SourceRef with offset {} and {} characters (ending at {}) with {} characters overflows for a source text of {} characters", self.offset, self.size, self.offset + self.size - 1, n, self.source.len()); }
        self.size += n;
    }



    /// Returns the name of the internal source text.
    #[inline]
    pub fn name(&self) -> &str { self.name }

    /// Returns the internal source text as a whole.
    #[inline]
    pub fn source(&self) -> &str { self.source }

    /// Returns the internal offset.
    #[inline]
    pub fn offset(&self) -> usize { self.offset }
    /// Returns the internal size.
    #[inline]
    pub fn size(&self) -> usize { self.size }
    /// Returns if there are still elements left in this SourceRef.
    #[inline]
    pub fn is_empty(&self) -> bool { self.size == 0 }



    /// Converts this SourceRef into an owned SourceText.
    /// 
    /// # Returns
    /// A new SourceText instance that clones relevant pieces into an ownable structure.
    /// 
    /// # Panics
    /// This function panics if the internal `offset` is out-of-range for the internal `source` reference.
    pub fn to_source_text(&self) -> SourceText {
        // EZ early quit if we're empty
        if self.size == 0 {
            return SourceText{
                source : String::new(),
                offset : self.offset,
                size   : self.size,

                name  : self.name.into(),
                start : (usize::MAX, usize::MAX),
                end   : (usize::MAX, usize::MAX),
            };
        }

        // Extract the relevant lines from the general source
        let mut source       : Option<&str>                               = None;
        let mut line_i       : usize                                      = 1;
        let mut col_i        : usize                                      = 1;
        let mut line_start   : usize                                      = 0;
        let mut source_start : Option<usize>                              = None;
        let mut start        : Option<(usize, usize)>                     = None;
        let mut end          : Option<(usize, usize)>                     = None;
        let mut iter         : std::iter::Peekable<std::str::CharIndices> = self.source.char_indices().peekable();
        while let Some((i, c)) = iter.next() {
            // Mark start and/or end positions
            if i == self.offset                 { start = Some((line_i, col_i)); }
            if i == self.offset + self.size - 1 { end   = Some((line_i, col_i)); }

            // A newline (or end-of-file) is where it all happens
            if c == '\n' || iter.peek().is_none() {
                // If we have been within the offset range, store it
                if self.offset <= i && self.offset + self.size - 1 >= line_start {
                    if source_start.is_none() { source_start = Some(line_start); }
                    source = Some(&self.source[*source_start.as_ref().unwrap()..i + 1]);
                }

                // Move to the next line
                line_i     += 1;
                col_i       = 1;
                line_start  = i + 1;

                // We can early quit the search if we've moved outside of the range
                if i >= self.offset + self.size { break; }
            } else {
                // Advance the column number
                col_i += 1;
            }
        }
        let source       : &str           = source.unwrap_or_else(|| panic!("Offset {} is out-of-range for source text of length {}", self.offset, self.source.len()));
        let source_start : usize          = source_start.unwrap_or_else(|| panic!("Offset {} is out-of-range for source text of length {}", self.offset, self.source.len()));
        let start        : (usize, usize) = start.unwrap_or_else(|| panic!("Offset {} is out-of-range for source text of length {}", self.offset, self.source.len()));
        let end          : (usize, usize) = end.unwrap_or_else(|| panic!("Offset {} is out-of-range for source text of length {}", self.offset, self.source.len()));

        // Compute new offsets that are relative to the selected source range
        let offset : usize = self.offset - source_start;
        let size   : usize = if self.size <= source.len() { self.size } else { source.len() };

        // Put that into ourselves
        SourceText {
            source : source.into(),
            offset,
            size,

            name : self.name.into(),
            start,
            end,
        }
    }

    /// Returns the internal source range for this reference.
    /// 
    /// # Panics
    /// This function panics if the internal `offset` and/or `size` is out-of-range for the internal `source` reference.
    #[inline]
    pub fn as_str(&self) -> &str { &self.source[self.offset..self.offset + self.size] }

    /// Returns a SourceTextDisplay that can be used to properly display the source reference as an error context.
    /// 
    /// # Arguments
    /// - `style`: The `console::Style` to use for accents (typically, use red for errors, yellow for warnings and blue for notes).
    /// 
    /// # Returns
    /// A new SourceTextDisplay instance that implements `Display`.
    #[inline]
    pub fn display<'b, 'c>(&'b self, style: Style) -> SourceTextDisplay<'b, Self> {
        SourceTextDisplay {
            source : self,
            style,
        }
    }
}

impl<'a> PartialEq for SourceRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        // Only compare the ranges & source text being the same
        (self.source as *const str) == (other.source as *const str) && self.offset == other.offset && self.size == other.size
    }
}

impl<'a> Add for SourceRef<'a> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}
impl<'a> Add for &SourceRef<'a> {
    type Output = SourceRef<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        // Simply create a new SourceRef that spans both
        if (self.source as *const str) != (rhs.source as *const str) { panic!("Cannot add two SourceRef's with difference source tests ({} VS {})", self.name, rhs.name); }
        SourceRef {
            source : self.source,
            offset : self.offset,
            size   : (rhs.offset + rhs.size) - self.offset,

            name : self.name,
        }
    }
}
impl<'a> AddAssign for SourceRef<'a> {
    fn add_assign(&mut self, rhs: Self) {
        // Simply create a new SourceRef that spans both
        if (self.source as *const str) != (rhs.source as *const str) { panic!("Cannot add two SourceRef's with difference source tests ({} VS {})", self.name, rhs.name); }
        self.size = (rhs.offset + rhs.size) - self.offset;
    }
}

impl<'a> nom::InputLength for SourceRef<'a> {
    #[inline]
    fn input_len(&self) -> usize { self.size }
}
impl<'a> nom::InputTake for SourceRef<'a> {
    fn take(&self, count: usize) -> Self {
        if count > self.size { panic!("Cannot `take()` {} characters of a SourceRef of size {}", count, self.size); }
        Self {
            source : self.source,
            offset : self.offset,
            size   : count,

            name : self.name,
        }
    }
    fn take_split(&self, count: usize) -> (Self, Self) {
        if count > self.size { panic!("Cannot `take_split()` {} characters of a SourceRef of size {}", count, self.size); }

        // Return the source refs as a tuple
        (
            Self {
                source : self.source,
                offset : self.offset + count,
                size   : self.size - count,

                name : self.name,
            },
            Self {
                source : self.source,
                offset : self.offset,
                size   : count,

                name : self.name,
            },
        )
    }
}
impl<'a> nom::InputIter for SourceRef<'a> {
    type Item     = char;
    type IterElem = std::str::Chars<'a>;
    type Iter     = std::str::CharIndices<'a>;

    fn iter_elements(&self) -> Self::IterElem {
        self.source[self.offset..self.offset + self.size].chars()
    }
    fn iter_indices(&self) -> Self::Iter {
        self.source[self.offset..self.offset + self.size].char_indices()
    }
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool
    {
        self.source[self.offset..self.offset + self.size].char_indices().find_map(|(i, c)| if predicate(c) { Some(i) } else { None })
    }
    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        let mut count: usize = count;
        while let Some((i, _)) = self.source[self.offset..self.offset + self.size].char_indices().next() {
            count -= 1;
            if count == 0 { return Ok(i); }
        }
        Err(nom::Needed::new(count))
    }
}
impl<'a> nom::UnspecializedInput for SourceRef<'a> {}
impl<'a> nom::Compare<&str> for SourceRef<'a> {
    fn compare(&self, t: &str) -> CompareResult {
        if self.size < t.len() { return CompareResult::Incomplete; }
        if &self.source[self.offset..self.offset + t.len()] == t {
            CompareResult::Ok
        } else {
            CompareResult::Error
        }
    }
    fn compare_no_case(&self, t: &str) -> CompareResult {
        if self.size < t.len() { return CompareResult::Incomplete; }
        if self.source[self.offset..self.offset + t.len()].to_lowercase() == t.to_lowercase() {
            CompareResult::Ok
        } else {
            CompareResult::Error
        }
    }
}
impl<'a> nom::Slice<RangeFrom<usize>> for SourceRef<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        if range.start >= self.size { panic!("Cannot `slice()` {} characters of a SourceRef of size {}", range.start, self.size); }
        println!("Slicing '{}' -> '{}'", &self.source[self.offset..self.offset + self.size], &self.source[(self.offset + range.start)..(self.offset + range.start) + (self.size - range.start)]);
        Self {
            source : &self.source,
            offset : self.offset + range.start,
            size   : self.size - range.start,

            name : self.name,
        }
    }
}
impl<'a> nom::Offset for SourceRef<'a> {
    fn offset(&self, second: &Self) -> usize {
        if self.offset >= second.offset {
            self.offset - second.offset
        } else {
            second.offset - self.offset
        }
    }
}

impl<'a> Debug for SourceRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        if self.offset < self.source.len() && self.offset + self.size <= self.source.len() {
            write!(f, "SourceRef<'{}', \"{}\">", self.name, self.source[self.offset..self.offset + self.size].replace("\n", "\\n").replace("\r", "\\r").replace("\t", "\\t"))
        } else {
            write!(f, "SourceRef<'{}', !OUT_OF_BOUNDS ({} > {} || {} >= {})!>", self.name, self.offset, self.source.len(), self.offset + self.size, self.source.len())
        }
    }
}

impl<'a> AsRef<SourceRef<'a>> for SourceRef<'a> {
    fn as_ref(&self) -> &SourceRef<'a> {
        self
    }
}
impl<'a> From<&SourceRef<'a>> for SourceRef<'a> {
    fn from(value: &SourceRef<'a>) -> Self {
        *value
    }
}



/// Defines an owned piece of source text, which has the line(s) it concerns already baked-in.
#[derive(Clone, Debug)]
pub struct SourceText {
    // Actual text reference (used to produce the source)
    /// The line(s) that are represented by this source text.
    source : String,
    /// The offset of this piece of source text in the lines. Will be offset relative to it.
    offset : usize,
    /// The length (in number of characters/bytes) in the original source.
    size   : usize,

    // Debug data (used to produce the entire line)
    /// Reference to the source's name (probably a filename, but might also be things like `<test>` or `<stdin>`).
    name  : String,
    /// Defines the start of the source reference as a `(line, col)` pair (inclusive). Both are one-indexed, and relative to the _source_, not the internal `lines`.
    start : (usize, usize),
    /// Defines the end of the source reference as a `(line, col)` pair (inclusive). Both are one-indexed, and relative to the _source_, not the internal `lines`.
    end   : (usize, usize),
}

impl SourceText {
    /// Returns the name of the internal source text.
    #[inline]
    pub fn name(&self) -> &str { &self.name }

    /// Returns the internal source text as a whole.
    #[inline]
    pub fn source(&self) -> &str { &self.source }
    /// Returns the internal source text as separate lines.
    #[inline]
    pub fn lines(&self) -> impl Iterator<Item = (usize, &str)> { self.source.lines().enumerate().map(|(i, l)| (self.start.0 + i, l)) }

    /// Returns the internal offset. Note that these are relative to the smaller internal fragment than they are to the source-global offset in `SourceRef`.
    #[inline]
    pub fn offset(&self) -> usize { self.offset }
    /// Returns the internal size. Note that these are relative to the smaller internal fragment than they are to the source-global size in `SourceRef`.
    #[inline]
    pub fn size(&self) -> usize { self.size }

    /// Returns the starting position, for debugging.
    /// 
    /// # Returns
    /// A tuple with `(line, column)`.
    #[inline]
    pub fn start(&self) -> (usize, usize) { self.start }
    /// Returns the end position (inclusive), for debugging.
    /// 
    /// # Returns
    /// A tuple with `(line, column)`.
    #[inline]
    pub fn end(&self) -> (usize, usize) { self.end }



    /// Returns the internal source range for this reference.
    /// 
    /// # Panics
    /// This function panics if the internal `offset` and/or `size` is out-of-range for the internal `source` reference.
    #[inline]
    pub fn as_str(&self) -> &str { &self.source[self.offset..self.offset + self.size] }

    /// Returns a SourceTextDisplay that can be used to properly display the source reference as an error context.
    /// 
    /// # Arguments
    /// - `style`: The `console::Style` to use for accents (typically, use red for errors, yellow for warnings and blue for notes).
    /// 
    /// # Returns
    /// A new SourceTextDisplay instance that implements `Display`.
    #[inline]
    pub fn display<'a>(&'a self, style: Style) -> SourceTextDisplay<'a, Self> {
        SourceTextDisplay {
            source : self,
            style,
        }
    }
}

impl PartialEq for SourceText {
    fn eq(&self, other: &Self) -> bool {
        // Only compare the ranges
        self.source == other.source && self.offset == other.offset && self.size == other.size
    }
}

impl<'a> From<SourceRef<'a>> for SourceText {
    #[inline]
    fn from(value: SourceRef<'a>) -> Self {
        value.to_source_text()
    }
}
impl<'a> From<&SourceRef<'a>> for SourceText {
    #[inline]
    fn from(value: &SourceRef<'a>) -> Self {
        value.to_source_text()
    }
}

impl AsRef<SourceText> for SourceText {
    fn as_ref(&self) -> &SourceText {
        self
    }
}
impl From<&SourceText> for SourceText {
    fn from(value: &SourceText) -> Self {
        value.clone()
    }
}
