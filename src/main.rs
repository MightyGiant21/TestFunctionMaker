use std::{fs::File, io::{BufReader, Read, Write}};
mod module;



// Create a test file
// Write the test boilerplate to the test file
// Open .rs file
// Read the file into a buffer
// Iterate over the .rs file and store location of each function
// Pass a ref to the .rs file and the function locations as args
// Insert test macro above each function




fn main() {
    // Create a test file
    let mut test_file = File::create("tests.rs").unwrap();

    // Create test boilerplate 
    let mut contents = generate_test_file_skeleton();
    
    // Open file to tbe tested
    let mut file_to_read = File::open("./module.rs").unwrap();

    // Create a buffer to hold contents of file that needs testing
    let mut buf_reader = BufReader::new(file_to_read);
    
    // Write contents of file into the string
    buf_reader.read_to_string(&mut contents).unwrap();

    // iterate_over_file_and_return_index_of_each_function(&mut file_to_read);


    writeln!(test_file, "{}", contents).unwrap();
}

fn generate_test_file_skeleton() -> String {
    let mut test_file_contents = String::new();
    
    test_file_contents.push_str("#[cfg(test)]\nmod tests {\n\\n}");

    test_file_contents
}

fn add_test_marco_to_function(contents: &str) -> String {
    let mut test_macro = String::from("#[test]\n");
    let contents = contents.to_owned();

    test_macro.push_str(&contents);

    test_macro
}

fn iterate_over_file_and_return_index_of_each_function(file: &mut File) {

}