//! Repository Observer
//!
//! The repository observer monitors a repository and notifies the dispatcher when a new commit is seen.
//!
//! The observer will poll the repository periodically, and when a change is seen, it will tell the dispatcher
//! the newest commit ID to run tests against. The observer checks for new commits by finding the current
//! commit ID in its repository, then updates the repository, and lastly, it finds the latest commit ID and
//! compares them. For the purposes of this example, the observer will only dispatch tests against the
//! latest commit. This means that if two commits are made between a periodic check, the observer will
//! only run tests against the latest commit.

use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    cimple::observer::poll("../ci-clone/")?;

    Ok(())
}
