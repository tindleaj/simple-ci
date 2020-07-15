use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

pub enum Request {
    Status,
    Dispatch(String),
    Register,
    Results,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Response {
    Ok,
    ReceivedDispatch(String),
    Err,
}

pub struct Dispatcher {
    runners: Vec<String>,
    dead: bool,
    dispatched_commits: Vec<String>,
    pending_commits: Vec<String>,
}

impl Dispatcher {
    pub fn new() -> Self {
        Dispatcher {
            runners: vec!["".into()],
            dead: false,
            dispatched_commits: vec!["".into()],
            pending_commits: vec!["".into()],
        }
    }

    /// Listens on a port for requests from a test runner or an observer.
    /// Takes commit IDs from an observer and dispatches a runner against that commit.
    /// Registers tests runners, and adds failed runs back into a pending queue.
    ///
    pub fn serve(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8888")?;

        let heartbeat = thread::spawn(|| loop {
            check();
            thread::sleep(std::time::Duration::from_secs(5))
        });

        let redistributor = thread::spawn(|| loop {
            redistribute();
            thread::sleep(std::time::Duration::from_secs(5))
        });

        for stream in listener.incoming() {
            handle(stream?)
        }

        heartbeat.join();
        redistributor.join();

        Ok(())
    }
}

// Impl Drop to handle hearbeat and redistributor joins

// handle incoming request on its own thread
fn handle(mut stream: TcpStream) {
    let thread_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        stream.read_to_end(&mut buf);

        println!("{:?}", buf);
    });
}

// Periodically ping each registered test runner, if they are unresponsive then remove it from the pool and add
// its commit id back to the pending list of commits
fn check() {
    println!("check")
}

fn redistribute() {
    println!("redistribute")
}
