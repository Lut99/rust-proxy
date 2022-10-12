//  TESTS.rs
//    by Lut99
// 
//  Created:
//    08 Oct 2022, 22:57:03
//  Last edited:
//    11 Oct 2022, 17:31:37
//  Auto updated?
//    Yes
// 
//  Description:
//!   Contains utilities that are used during testing only.
// 

use std::io::Read;
use std::fs::{self, DirEntry, File, ReadDir};
use std::path::PathBuf;


/***** CONSTANTS *****/
/// The path to the test directory.
pub const TEST_DIR: &str = "../tests";





/***** LIBRARY MACROS *****/
/// Creates a located span for us.
macro_rules! input {
    ($text:literal) => {
        crate::spec::Input::new($text)
    };

    ($text:literal, $offset:expr) => {
        unsafe { crate::spec::Input::new_from_raw_offset($offset, 1, $text, ()) }
    };
    ($text:literal, $offset:expr, $line:expr) => {
        unsafe { crate::spec::Input::new_from_raw_offset($offset, $line, $text, ()) }
    };
}
pub(crate) use input;



/// Shortcut for creating a new range
macro_rules! range {
    ($x1:literal : $y1:literal - $x2:literal : $y2:literal) => {
        TextRange::new(TextPos::new($x1, $y1), TextPos::new($x2, $y2))
    };
}
pub(crate) use range;







/***** LIBRARY *****/
/// Runs the given closure on the source text of every file in the `tests` directory.
/// 
/// # Arguments
/// - `f`: The closure to call. It has the following signature:
///   - `path`: The (full) path of the current file.
///   - `source`: The contents of the file as a string.
/// 
/// # Panics
/// This function panics if we failed to read any of the files or directories. Additionally, if the closure fails to run the test, it should panic as well.
pub fn run_test_on_files<F>(f: F)
where
    F: FnMut(PathBuf, String),
{
    let mut f: F = f;

    // Start by reading the directory
    let entries: ReadDir = match fs::read_dir(&TEST_DIR) {
        Ok(entries) => entries,
        Err(err)    => { panic!("Failed to read directory '{}': {}", TEST_DIR, err); },  
    };

    // Iterate over them all
    for (i, entry) in entries.enumerate() {
        // Resolve it
        let entry: DirEntry = match entry {
            Ok(entry) => entry,
            Err(err)  => { panic!("Failed to read entry {} in directory '{}': {}", i, TEST_DIR, err); },
        };
        let entry_path: PathBuf = entry.path();

        // Read the file
        let mut handle: File = match File::open(&entry_path) {
            Ok(handle) => handle,
            Err(err)   => { panic!("Failed to open file '{}': {}", entry_path.display(), err); },
        };
        let mut source: String = String::new();
        if let Err(err) = handle.read_to_string(&mut source) { panic!("Failed to read file '{}': {}", entry_path.display(), err); }

        // We can now run the closure
        println!("File '{}':", entry_path.display());
        println!("{}", (0..80).map(|_| '-').collect::<String>());
        f(entry_path, source);
        println!("{}", (0..80).map(|_| '-').collect::<String>());
        println!();
        println!();
    }
}
