//  SPEC.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 22:12:02
//  Last edited:
//    22 Oct 2022, 15:40:46
//  Auto updated?
//    Yes
// 
//  Description:
//!   Defines (public) interfaces and structs that do not have a specific
//!   relation to the AST (i.e., are not nodes in it).
// 

use std::fmt::Debug;


/***** LIBRARY *****/
/// Defines how a node in the AST looks like.
pub trait Node<T>: Clone + Debug {
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
    fn source<'a>(&'a self) -> &'a Option<T>;
}
