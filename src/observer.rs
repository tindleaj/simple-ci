use crate::{communicate, Request, Response};
use std::error::Error;
use std::fs::{remove_file, File};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

/// Watches target repo for new commits
pub fn poll(repo: &str) -> Result<(), Box<dyn Error>> {
    loop {
        update_repo(repo)?;

        if Path::new(".commit_id").is_file() {
            let status = communicate("localhost", 8888, Request::Status)?;
            println!("Dispatcher status: {:#?}", status);

            if status == Response::Ok {
                let mut file = File::open(".commit_id")?;
                let mut commit_id = String::new();

                file.read_to_string(&mut commit_id)?;

                println!("Sending commit_id `{}` to dispatcher", commit_id);
                let response = communicate("localhost", 8888, Request::Dispatch { commit_id })?;

                dbg!(&response);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}

/// Identify new commits and report them to the observer
fn update_repo(repo: &str) -> Result<(), Box<dyn Error>> {
    // Remove old .commit_id if it exists
    if File::open(".commit_id").is_ok() {
        remove_file(".commit_id")?
    }

    // Check to see if the target repo exists
    if !Path::new(repo).exists() {
        panic!("Repository dir `{}` not found", repo);
    }

    // git reset --hard origin/HEAD in target repo
    Command::new("git")
        .current_dir(repo)
        .arg("reset")
        .arg("--hard")
        .arg("origin/HEAD")
        .output()?;

    // Get the newest commit id before pulling
    let commit_id = get_newest_commit_id(repo)?;

    // update repo
    Command::new("git").current_dir(repo).arg("pull").output()?;

    let new_commit_id = get_newest_commit_id(repo)?;

    if new_commit_id != commit_id {
        let mut file = File::create(".commit_id")?;

        println!("New commit found, updating .commit_id:");
        println!("\tPrevious: {}", commit_id);
        println!("\tNew: {}", new_commit_id);

        write!(&mut file, "{}", new_commit_id)?;
    }

    Ok(())
}

fn get_newest_commit_id(repo: &str) -> Result<String, Box<dyn Error>> {
    // get the most recent local commit
    let commit = Command::new("git")
        .current_dir(repo)
        .arg("log")
        .arg("-n1")
        .output()?
        .stdout;

    let commit = String::from_utf8(commit).unwrap();

    Ok(commit.split_whitespace().nth(1).unwrap().into())
}
