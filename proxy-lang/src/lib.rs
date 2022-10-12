//  LIB.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 21:48:58
//  Last edited:
//    08 Oct 2022, 23:00:22
//  Auto updated?
//    Yes
// 
//  Description:
//!   The `proxy-lang` crate implements a scanner, parser and (simple)
//!   compiler for the custom proxy language that is used to configure it.
// 

// Declare modules
pub mod errors;
pub mod warnings;
pub mod spec;
pub mod tokens;
pub mod ast;
pub mod scanner;

// Declare test modules
#[cfg(test)]
pub mod tests;
