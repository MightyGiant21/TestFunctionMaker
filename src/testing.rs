#[cfg(test)]
mod testing {
    use crate::file_generator::TestObj;

    #[test]
    fn get_names_and_how_many_functions_in_module() {
        let mut test_obj = TestObj::init_obj("./src/module.rs");
        let contents = test_obj.contents;

        let mut function_name = String::new();

        // Iterate over contents to find a function
        for (i, char) in contents.chars().enumerate() {
            if char == 'f' {
                if contents.chars().nth(i + 1).unwrap() == 'n' {
                    if contents.chars().nth(i + 2).unwrap() == ' ' {
                        test_obj.number_of_functions_in_module += 1;
                        let mut counter = 0;
                        function_name.push_str("    ");
                        while contents.chars().nth(i + counter).unwrap() != '{' {
                            // Push each char into a string
                            function_name.push(contents.chars().nth(i + counter).unwrap());
                            counter += 1;
                        }
                        function_name.push_str("{ \n \n} \n\n");
                        test_obj.names_of_functions.push(function_name.clone());
                        function_name.drain(..);
                    }
                }
            }
        }

        let mut s = String::new();

        for i in 0..test_obj.names_of_functions.len() {
            s.push_str(&test_obj.names_of_functions[i])
        }

        assert_eq!(
            test_obj.names_of_functions,
            [
                "    fn does_it_work() { \n \n} \n\n",
                "    fn i_doubt_it() { \n \n} \n\n",
                "    fn third_time_lucky() { \n \n} \n\n"
            ]
        );
    }

    #[test]
    fn add_test_marcos_to_functions() {
        let mut test_obj = TestObj::init_obj("./src/module.rs");
        let test_marco = String::from("    #[test]\n");

        test_obj.names_of_functions = test_obj.get_names_and_how_many_functions_in_module_for_tdd();
        // Add macro to each function name
        // Function name is a vec of strings
        // Index the vec and insert test to each string
        // When we index with i, the macro gets pushed into location 0, 1, 2 etc
        // Index needs to skip one
        for i in 0..test_obj.names_of_functions.len() {
            if i < test_obj.names_of_functions.len() {
                test_obj
                    .names_of_functions
                    .insert(i * 2, test_marco.clone());
            }
        }

        assert_eq!(
            test_obj.names_of_functions,
            [
                "    #[test]\n",
                "fn does_it_work() { \n \n} \n",
                "    #[test]\n",
                "fn i_doubt_it() { \n \n} \n",
                "    #[test]\n",
                "fn third_time_lucky() { \n \n} \n"
            ]
        );
    }

    #[test]
    fn add_skeleton_to_test_functions() {
        let mut test_obj = TestObj::init_obj("./src/module.rs");
        test_obj.names_of_functions = test_obj.get_names_and_how_many_functions_in_module_for_tdd();
        test_obj.names_of_functions = test_obj.add_test_marcos_to_functions_for_tdd();

        let mut boilerplate = String::from("#[cfg(test)]\nmod tests {\n");
        // Push functions into test skeleton
        for i in 0..test_obj.names_of_functions.len() {
            boilerplate.push_str(&test_obj.names_of_functions[i])
        }
        boilerplate.push_str("\n}");
        test_obj.contents = boilerplate;

        assert_eq!(test_obj.contents, "#[cfg(test)]\nmod tests {\n#[test]\nfn does_it_work() { \n \n} \n#[test]\nfn i_doubt_it() { \n \n} \n#[test]\nfn third_time_lucky() { \n \n} \n\n}");
    }
}
