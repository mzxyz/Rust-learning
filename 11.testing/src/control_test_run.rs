// cargo test compiles your code in test mode and 
// runs the resulting test binary. You can specify
// command line options to change the default 
// behavior of cargo test. For example, the default 
// behavior of the binary produced by cargo test is 
// to run all the tests in parallel and capture 
// output generated during test runs, preventing 
// the output from being displayed and making it 
// easier to read the output related to the test results.

// Running cargo test --help displays the options you can 
// use with cargo test, and running cargo test -- --help 
// displays the options you can use after the separator --.

// 1. Running Tests in Parallel or Consecutively
// When you run multiple tests, by default they run in parallel using threads
// So make sure your tests don’t depend on each other or on any 
// shared state, including a shared environment, such as 
// the current working directory or environment variables.

// If you don’t want to run the tests in parallel or if you want more 
// fine-grained control over the number of threads used, you can send 
// the --test-threads flag and the number of threads you want to use to the test binary. 

// set the number of test threads to 1, telling the program not to use any parallelism.
//===== cargo test -- --test-threads=1 ====//


// 2. Showing Function Output

// If we want to see printed values for passing tests as well, we can 
// disable the output capture behavior by using the --nocapture flag:
// cargo test -- --nocapture

// 3. Running a Subset of Tests by Name
// cargo test it_works

// 4. Filtering to Run Multiple Tests
// We can specify part of a test name, and any test whose name matches that value will be run
// Also note that the module in which a test appears becomes part of the test’s name, 
// so we can run all the tests in a module by filtering on the module’s name.

// 5. Ignoring Some Tests Unless Specifically Requested
// Sometimes a few specific tests can be very time-consuming to execute, so you might want to 
// exclude them during most runs of cargo test. Rather than listing as arguments all tests you 
// do want to run, you can instead annotate the time-consuming tests using the ignore attribute
// to exclude them