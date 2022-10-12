//  MOD.rs
//    by Lut99
// 
//  Created:
//    11 Oct 2022, 23:04:18
//  Last edited:
//    11 Oct 2022, 23:32:36
//  Auto updated?
//    Yes
// 
//  Description:
//!   The parser module takes the tokens from the scanner and generates an
//!   AST with it (defined in `ast.rs`).
// 

// Declare the submodules
pub mod pattern;
pub mod areas;
pub mod parser;

// Pull stuff into the global namespace
pub use parser::{parse, Error};
