use crate::utils;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Watches target repo for new commits
pub fn poll(repo: &str) {
    loop {
        update_repo(repo);
        std::thread::sleep(std::time::Duration::from_millis(2_000));
    }
}

/// Identify new commits and report them to the observer
fn update_repo(repo: &str) -> Result<(), std::io::Error> {
    // Remove old .commit_id. we don't care about an error here
    remove_file(".commit_id");

    // Check to see if the target repo exists
    if !Path::new(repo).exists() {
        panic!("Repository folder not found");
    }

    // git reset --hard HEAD in target repo
    Command::new("git")
        .current_dir(repo)
        .arg("reset")
        .arg("--hard")
        .arg("HEAD")
        .output()
        .expect("Could not reset git");

    // Get the newest commit id before pulling
    let commit_id = get_newest_commit_id(repo);

    // update repo
    Command::new("git")
        .current_dir(repo)
        .arg("pull")
        .output()
        .expect("Could not pull from repository");

    let new_commit_id = get_newest_commit_id(repo);

    if new_commit_id != commit_id {
        let mut file = File::create(".commit_id")?;

        write!(&mut file, "{}", new_commit_id)?;
    }

    Ok(())
}

fn get_newest_commit_id(repo: &str) -> String {
    // get the most recent local commit
    let commit = Command::new("git")
        .current_dir(repo)
        .arg("log")
        .arg("-n1")
        .output()
        .expect("Could not call 'git log' on repository")
        .stdout;

    let commit = String::from_utf8(commit).unwrap();

    commit.split_whitespace().nth(1).unwrap().into()
}
