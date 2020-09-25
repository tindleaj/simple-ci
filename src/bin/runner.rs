//! Test Runner
//!
//! The test runner is responsible for running tests against a given commit ID and returning the results
//!
//! The test runner communicates with the dispatch server, which supplies the commit IDs to run against
//! , and which handles any results returned from the test runner.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    todo!("Implement the runner");
}
