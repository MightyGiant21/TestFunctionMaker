use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct TestObj {
    pub contents: String,
    pub number_of_functions_in_module: u64,
    pub names_of_functions: Vec<String>,
}

impl TestObj {
    pub fn init_obj(path: &str) -> TestObj {
        let mut file_to_read = File::open(path).unwrap();
        let mut contents = String::new();

        file_to_read.read_to_string(&mut contents).unwrap();

        TestObj {
            contents: contents,
            number_of_functions_in_module: 0,
            names_of_functions: Vec::new(),
        }
    }

    pub fn get_names_and_how_many_functions_in_module(&mut self) {
        let mut function_name = String::new();

        // Iterate over contents to find a function
        for (i, char) in self.contents.chars().enumerate() {
            if char == 'f' &&
            self.contents.chars().nth(i + 1).unwrap() == 'n' &&
            self.contents.chars().nth(i + 2).unwrap() == ' ' {
                        self.number_of_functions_in_module += 1;
                        let mut counter = 0;
                        function_name.push_str("    ");
                        while self.contents.chars().nth(i + counter).unwrap() != '(' {
                            // Push each char into a string
                            function_name.push(self.contents.chars().nth(i + counter).unwrap());
                            counter += 1;
                        }
                        function_name.push_str("() { \n \n    } \n\n");
                        self.names_of_functions.push(function_name.clone());
                        function_name.drain(..);
                }
            }
    }

    pub fn add_test_marcos_to_functions(&mut self) {
        let test_marco = String::from("    #[test]\n");

        // Add macro to each function name
        // Function name is a vec of strings
        // Index the vec and insert test to each string
        for i in 0..self.names_of_functions.len() {
            if i < self.names_of_functions.len() {
                self.names_of_functions.insert(i * 2, test_marco.clone());
            }
        }
    }

    pub fn add_skeleton_to_test_functions(&mut self) {
        let mut boilerplate = String::from("#[cfg(test)]\nmod tests {\n");
        // Push functions into test skeleton
        for i in 0..self.names_of_functions.len() {
            boilerplate.push_str(&self.names_of_functions[i])
        }
        boilerplate.push_str("\n}");
        self.contents = boilerplate;
    }

    // // // // // // // // // // // // // Helpers for testing // // // // // // // // // // // // //

    pub fn get_names_and_how_many_functions_in_module_for_tdd(&mut self) -> Vec<String> {
        let mut function_name = String::new();

        // Iterate over contents to find a function
        for (i, char) in self.contents.chars().enumerate() {
            if char == 'f' {
                if self.contents.chars().nth(i + 1).unwrap() == 'n' {
                    if self.contents.chars().nth(i + 2).unwrap() == ' ' {
                        self.number_of_functions_in_module += 1;
                        let mut counter = 0;
                        function_name.push_str("    ");
                        while self.contents.chars().nth(i + counter).unwrap() != '{' {
                            // Push each char into a string
                            function_name.push(self.contents.chars().nth(i + counter).unwrap());
                            counter += 1;
                        }
                        function_name.push_str("{ \n \n} \n");
                        self.names_of_functions.push(function_name.clone());
                        function_name.drain(..);
                    }
                }
            }
        }

        self.names_of_functions.clone()
    }

    pub fn add_test_marcos_to_functions_for_tdd(&mut self) -> Vec<String> {
        let test_marco = String::from("    #[test]\n");

        // Add macro to each function name
        // Function name is a vec of strings
        // Index the vec and insert test to each string
        for i in 0..self.names_of_functions.len() {
            if i < self.names_of_functions.len() {
                self.names_of_functions.insert(i * 2, test_marco.clone());
            }
        }
        self.names_of_functions.clone()
    }
}