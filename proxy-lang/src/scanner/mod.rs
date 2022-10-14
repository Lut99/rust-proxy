//  MOD.rs
//    by Lut99
// 
//  Created:
//    08 Oct 2022, 20:31:32
//  Last edited:
//    12 Oct 2022, 15:15:09
//  Auto updated?
//    Yes
// 
//  Description:
//!   The scanner module implements everything needed to convert source to
//!   a stream of tokens.
// 

// Declare modules
pub mod whitespace;
pub mod comments;
pub mod punctuation;
pub mod keywords;
pub mod values;
pub mod scanner;

// Pull stuff into the global namespace
pub use scanner::{scan, Error};
