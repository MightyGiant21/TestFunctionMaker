use std::{fs::File, io::Read};

pub struct Files {
    pub file_to_read: File, 
    pub test_file: File,
    pub contents: Vec<char>
}

impl Files {
    pub fn create_test_file(rom: &str) -> Files {
        // Create a test file
        let test_file = File::create("./src/tests.rs").unwrap();
        let file_to_read = File::open(rom).unwrap();
        
        Files {
            file_to_read: file_to_read,
            test_file: test_file,
            contents: Vec::new()
        }
    }
   
    pub fn generate_test_file_skeleton(&mut self) {
        let boilerplate = String::from("#[cfg(test)]\nmod tests {\n
        \n}");
        self.contents = boilerplate.chars().collect();
    }

    pub fn convert_contents_to_string(&mut self) -> String {
        let mut chars_as_string = String::new();
        
        for i in 0..self.contents.len() {
            chars_as_string.push(self.contents[i])
        }

        chars_as_string
    }

    pub fn insert_functions_into_test_file(&mut self) {
        // Find the index for the start of each function
        // Find the index of the end of the function. Look for \n
        // Iterate over the function and save the string into a variable
        // Push the string to the test file

        // Push contents of file into a string
        let mut string = String::new();
        let mut start_and_end_index: Vec<(u8, u8)> = Vec::new();
        self.file_to_read.read_to_string(&mut string).unwrap();

        // Get start and end index of function
        // Can use this to calc length of chars
        for (i, char) in string.chars().enumerate()  {
            if char == 'f' {
                if string.chars().next() == Some('n') {
                    if string.chars().next() == Some(' ') {
                        start_and_end_index[0].0 = i as u8
                    }
                }
            }

            if char == '{' {
                start_and_end_index[0].1 = i as u8
            }
        }
    }
}