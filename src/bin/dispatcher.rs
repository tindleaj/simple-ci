//! Dispatcher
//!
//! The dispatcher is a service that delegates testing tasks and returns results.
//!
//! The dispatcher listens on a port for requests from test runners and from the repository observer.
//! It allows test runners to register themselves, and when given a commit ID from the repository
//! observer, it will dispatch a test runner against the new commit. It also gracefully handles any
//! problems with the runners and will redistribute the commit ID to a new test runner if anything
//! goes wrong.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dispatcher = cimple::dispatcher::Dispatcher::new();

    dispatcher.serve()?;

    Ok(())
}
