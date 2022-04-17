mod module;
mod file_generator;
mod tests;

use std::io::Write;

use crate::file_generator::Files;

fn main() {
    let mut test_file = Files::create_test_file("./src/module.rs");
    test_file.generate_test_file_skeleton();

    let contents = test_file.convert_contents_to_string();

    writeln!(test_file.test_file, "{}", contents).unwrap();
}

fn add_test_marco_to_function(contents: &str) -> String {
    let mut test_macro = String::from("#[test]\n");
    let contents = contents.to_owned();

    test_macro.push_str(&contents);

    test_macro
}