Automatic test file generator for when you have lots of functions to test.
========

Clone the repo, build in release mode and then run test_maker from the release folder. First arg is the file eith the functions inside it, the second arg is where you want the test module to be.

Example
====

./test_maker ./src/cpu.rs ./src/tests.rs

Currently it can only create a new file and can't add to it. It will erase everything if you run it again!
