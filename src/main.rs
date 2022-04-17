mod file_generator;
mod module;
mod testing;
mod tests;

use std::fs::File;
use std::io::Write;

use crate::file_generator::TestObj;

fn main() {
    let mut test_obj = TestObj::init_obj("./src/module.rs");
    let mut test_file = File::create("./src/tests.rs").unwrap();

    test_obj.get_names_and_how_many_functions_in_module();
    test_obj.add_test_marcos_to_functions();
    test_obj.add_skeleton_to_test_functions();

    writeln!(test_file, "{}", test_obj.contents).unwrap();
}

// Create a test.rs file

// Read contents of module to be tested into a string or vec

// Find out how many functions are in the module

// Find out how many chars are in each function and store as name
// Create a test block which pushes a test macro then pushes a function name. Repeat for each function
// Push all blocks into one large block
// Push large block into test skeleton
// Write it all into test.rs

// // // // // // // // // // // // // // // Struct // // // // // // // // // // // // // // // // //

// test_file: File
// content: String
// number_of_functions_in_module: u64
// names_of_functions: Vec<string>
