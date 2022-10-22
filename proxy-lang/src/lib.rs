//  LIB.rs
//    by Lut99
// 
//  Created:
//    07 Oct 2022, 21:48:58
//  Last edited:
//    22 Oct 2022, 14:50:02
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
pub mod source;
pub mod tokens;
// pub mod ast;
pub mod scanner;
// pub mod parser;

// Declare test modules
#[cfg(test)]
pub mod tests;
