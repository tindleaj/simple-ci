use std::process::Command;

/// Watches target repo for new commits
pub fn poll() {
    loop {
        let output = Command::new("git")
            .arg("log")
            .arg("--oneline")
            .output()
            .unwrap();
        println!("{:?}", output);
        std::thread::sleep(std::time::Duration::from_millis(2_000));
    }
}
